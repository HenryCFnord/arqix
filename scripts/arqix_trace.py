#!/usr/bin/env python3
"""Python oracle for the arqix trace family: scan, check, coverage, matrix.

Reference implementation per the oracle policy in arc42 chapter 8: this
module defines the behavioural contract (JSON/CSV schemas, exit codes,
determinism) that the Rust Trace Engine must match; the un-ignored skeleton
tests under tests/cli_trace.rs, run with ARQIX_BIN pointing at
scripts/arqix, are the conformance suite.

Unlike the repo-bound checker scripts, everything here is relative to the
current working directory so the oracle works inside fixture repositories.

Semantics:
- Requirement documents are Markdown files whose frontmatter carries an
  `id: REQ-XX-YY-ZZ-NN`; the kind is read from `arqix:classes/…`, and a
  missing kind is treated as functional (strictest default).
- Markers are `// arqix:verifies|implements REQ-…` comment lines in Rust
  sources (REQ-03-01-05-01) and `<!-- arqix:verifies|implements REQ-… -->`
  HTML comments in Markdown (REQ-03-01-05-02). A marker only counts when
  the line starts with the comment token, so string literals containing
  marker text are ignored.
- Unresolved references stay visible in the graph (REQ-03-01-05-05).
- Output is deterministic: identical corpus => identical bytes
  (REQ-00-00-00-01).

Exit codes: 0 = success, 1 = findings (coverage gaps, unknown requirement),
2 = usage error.
"""

import csv
import io
import json
import re
import sys
from pathlib import Path

REQ_ID_RE = re.compile(r"^REQ-\d{2}-\d{2}-\d{2}-\d{2}$")
FRONTMATTER_ID_RE = re.compile(r"^id:\s*[\"']?([\w][\w-]*)[\"']?\s*$")
FRONTMATTER_IRI_RE = re.compile(r"^iri:\s*(\S+)\s*$")
FRONTMATTER_TITLE_RE = re.compile(r"^title:\s*[\"']?(.+?)[\"']?\s*$")
TOP_KEY_RE = re.compile(r"^([\w.-]+):")
TRIPLE_PRED_RE = re.compile(r"^-\s+predicate:\s*arqix:properties/(\S+)\s*$")
TRIPLE_OBJ_INLINE_RE = re.compile(r"^object:\s*(arqix:\S+)\s*$")
TRIPLE_OBJ_ITEM_RE = re.compile(r"^-\s+(arqix:\S+)\s*$")
CLASS_ITEM_RE = re.compile(r"^-\s+arqix:classes/(\S+)\s*$")
KIND_RE = re.compile(
    r"arqix:classes/(functional-requirement|quality-requirement|constraint)"
)
RS_MARKER_RE = re.compile(r"^//\s*arqix:(verifies|implements)\s+(\S+)\s*$")
MD_MARKER_RE = re.compile(r"^<!--\s*arqix:(verifies|implements)\s+(\S+)\s*-->\s*$")

SKIP_DIRS = {".git", "target", "node_modules", "__pycache__", "fixtures"}

KIND_SHORT = {
    "functional-requirement": "functional",
    "quality-requirement": "quality",
    "constraint": "constraint",
}

# Bumped on any JSON shape change until the Rust port passes conformance;
# from then on the layer stability rules of ADR-0006 apply.
SCHEMA_VERSION = 1


def read_corpus(root):
    """Walk `root` and return {relative posix path: text} for .md and .rs files."""
    corpus = {}
    root = Path(root)
    for path in sorted(root.rglob("*")):
        if path.suffix not in (".md", ".rs") or not path.is_file():
            continue
        # Templates are scaffolds with placeholder IDs, not corpus documents.
        if path.name.endswith(".tpl.md"):
            continue
        rel = path.relative_to(root)
        if any(part in SKIP_DIRS for part in rel.parts):
            continue
        try:
            corpus[rel.as_posix()] = path.read_text(encoding="utf-8")
        except UnicodeDecodeError:
            continue
    return corpus


def frontmatter_lines(text):
    """Yield the lines of the leading frontmatter block, if any."""
    lines = text.splitlines()
    if not lines or lines[0].strip() != "---":
        return []
    block = []
    for line in lines[1:]:
        if line.strip() == "---":
            return block
        block.append(line)
    return []


def parse_document(path, text):
    """Parse one Markdown document's frontmatter into a document record.

    Returns None for documents without an `id`. The record carries the
    ontology triples as (predicate, object-IRI, file line) so every YAML
    header link — derived-from, has-requirement, is-part-of-workflow,
    has-persona, guides-implementation-of, … — becomes a graph edge
    (REQ-03-01-05-03 generalised to the whole corpus).
    """
    block = frontmatter_lines(text)
    if not block:
        return None
    doc = {"id": None, "iri": None, "title": None, "file": path,
           "classes": [], "triples": []}
    section = None
    predicate = None
    for idx, line in enumerate(block):
        file_line = idx + 2  # frontmatter body starts on file line 2
        stripped = line.strip()
        if line and not line[0].isspace():
            if m := TOP_KEY_RE.match(line):
                section = m.group(1)
                predicate = None
            if m := FRONTMATTER_ID_RE.match(stripped):
                doc["id"] = m.group(1)
            elif m := FRONTMATTER_IRI_RE.match(stripped):
                doc["iri"] = m.group(1)
            elif m := FRONTMATTER_TITLE_RE.match(stripped):
                doc["title"] = m.group(1)
            continue
        if section == "rdf":
            if m := CLASS_ITEM_RE.match(stripped):
                doc["classes"].append(m.group(1))
        elif section == "triples":
            if m := TRIPLE_PRED_RE.match(stripped):
                predicate = m.group(1)
            elif predicate and (m := TRIPLE_OBJ_INLINE_RE.match(stripped)):
                doc["triples"].append((predicate, m.group(1), file_line))
            elif predicate and (m := TRIPLE_OBJ_ITEM_RE.match(stripped)):
                doc["triples"].append((predicate, m.group(1), file_line))
    return doc if doc["id"] else None


def document_type(doc):
    for cls in doc["classes"]:
        if cls in KIND_SHORT:
            return "requirement"
        return cls
    return "requirement" if REQ_ID_RE.match(doc["id"]) else "document"


def build_model(corpus):
    """Build the trace model from a {path: text} corpus.

    Returns (requirements, edges, documents):
      requirements: {req_id: {"kind", "file", "kind_declared"}}
      edges: sorted list of {"from", "to", "kind", "line", "ignored"};
             marker edges originate from artefact paths, frontmatter-link
             edges from document IDs (with a "file" field for location)
      documents: {doc_id: {"file", "iri", "type"}}
    """
    documents = {}
    requirements = {}
    iri_map = {}
    parsed = []
    for path, text in sorted(corpus.items()):
        if not path.endswith(".md"):
            continue
        doc = parse_document(path, text)
        if doc is None:
            continue
        parsed.append(doc)
        documents[doc["id"]] = {
            "file": path,
            "iri": doc["iri"],
            "title": doc["title"],
            "type": document_type(doc),
        }
        if doc["iri"]:
            iri_map[doc["iri"]] = doc["id"]
        if document_type(doc) == "requirement":
            kind_match = KIND_RE.search(" ".join(
                f"arqix:classes/{c}" for c in doc["classes"]
            ))
            requirements[doc["id"]] = {
                "kind": KIND_SHORT[kind_match.group(1)] if kind_match else "functional",
                "file": path,
                "kind_declared": kind_match is not None,
            }

    edges = []
    for path, text in sorted(corpus.items()):
        marker_re = RS_MARKER_RE if path.endswith(".rs") else MD_MARKER_RE
        lines = text.splitlines()
        for line_no, line in enumerate(lines, start=1):
            if m := marker_re.match(line.strip()):
                link_kind, target = m.group(1), m.group(2)
                attached = (
                    _attached_test(lines, line_no)
                    if path.endswith(".rs")
                    else {"ignored": False, "test": None}
                )
                edges.append(
                    {
                        "from": path,
                        "to": target,
                        "kind": link_kind,
                        "line": line_no,
                        "ignored": attached["ignored"],
                        "test": attached["test"],
                    }
                )
    for doc in parsed:
        for predicate, obj_iri, line_no in doc["triples"]:
            edges.append(
                {
                    "from": doc["id"],
                    "to": iri_map.get(obj_iri, obj_iri),
                    "kind": predicate,
                    "line": line_no,
                    "file": doc["file"],
                    "ignored": False,
                    "test": None,
                }
            )
    edges.sort(key=lambda e: (e["from"], e["line"], e["to"], e["kind"]))
    return requirements, edges, documents


def _attached_test(lines, marker_line_no):
    """Inspect the marker's contiguous comment/attribute block up to the
    function it annotates: whether that function is `#[ignore]`d (a marker
    on an ignored test plans verification, it does not provide it —
    red-skeleton lifecycle, ADR-0006) and the function's name (location
    context on the edge, ADR-0007)."""
    ignored = False
    for line in lines[marker_line_no:]:
        stripped = line.strip()
        if stripped.startswith("#[ignore"):
            ignored = True
            continue
        if stripped.startswith(("//", "#[")):
            continue
        if m := re.match(r"(?:pub\s+)?fn\s+(\w+)", stripped):
            return {"ignored": ignored, "test": m.group(1)}
        break
    return {"ignored": ignored, "test": None}


def story_of(req_id):
    """Derive the owning story from a requirement ID: REQ-XX-YY-ZZ-NN ->
    US-XX-YY-ZZ; None for the cross-cutting foundation domain 00-00-00."""
    stem = req_id[4:12]
    return None if stem == "00-00-00" else f"US-{stem}"


def owner_story(req_id, edges):
    """The owning story per the canonical-owner model: the first
    derived-from link in the requirement's frontmatter; falls back to the
    ID-derived story when no links exist (e.g. fixture corpora)."""
    for e in edges:
        if (
            e["from"] == req_id
            and e["kind"] == "derived-from"
            and str(e["to"]).startswith("US-")
        ):
            return e["to"]
    return story_of(req_id)


def graph(requirements, edges, documents):
    """The scan result: the canonical core graph (ADR-0006 layer 1) with
    node and edge collections (REQ-03-01-05-04). Nodes cover every corpus
    document (stories, workflows, personas, ADRs, units, …), edges cover
    markers and all frontmatter triples."""
    nodes = []
    for doc_id in sorted(documents):
        info = documents[doc_id]
        node = {"id": doc_id, "type": info["type"], "file": info["file"]}
        if doc_id in requirements:
            req = requirements[doc_id]
            node["kind"] = req["kind"]
            node["kind_declared"] = req["kind_declared"]
            node["story"] = owner_story(doc_id, edges)
        nodes.append(node)
    referenced = {e["to"] for e in edges}
    known = set(documents)
    for target in sorted(referenced - known):
        nodes.append(
            {
                "id": target,
                "type": "requirement" if REQ_ID_RE.match(str(target)) else "unknown",
                "unresolved": True,
            }
        )
    for source in sorted({e["from"] for e in edges} - known):
        # Node identity rule (ADR-0007): artefacts declare no ID, so their
        # repository-relative path is both id and file.
        nodes.append({"id": source, "type": "artefact", "file": source})
    return {"schema_version": SCHEMA_VERSION, "nodes": nodes, "edges": edges}


def coverage(requirements, edges):
    """Coverage as a diagnostics projection (ADR-0006 layer 2).

    Coverage follows the red-skeleton lifecycle (ADR-0006): a marker on
    an active test *verifies*, a marker on an `#[ignore]`d test only
    *plans* verification, no marker at all leaves the requirement
    *uncovered*. A coverage gap is a diagnostic in the tool-wide findings
    format (severity, stable code, message, location — REQ-00-00-00-03):
    TRC-COV-001 (error) for an uncovered functional requirement
    (REQ-01-01-08-01), TRC-COV-002 (warning) for a planned-only one,
    TRC-KIND-001 (warning) for a requirement without a declared kind,
    treated as functional. Only error diagnostics drive the exit code.
    """
    links = {
        req_id: {"verifies": [], "planned": [], "implements": []}
        for req_id in requirements
    }
    for e in edges:
        if e["to"] in links and e["kind"] in ("verifies", "implements"):
            bucket = "planned" if e["kind"] == "verifies" and e["ignored"] else e["kind"]
            links[e["to"]][bucket].append(f"{e['from']}:{e['line']}")

    rows = []
    summary = {}
    diagnostics = []
    for req_id in sorted(requirements):
        info = requirements[req_id]
        kind = info["kind"]
        verified_by = sorted(links[req_id]["verifies"])
        planned_by = sorted(links[req_id]["planned"])
        implemented_by = sorted(links[req_id]["implements"])
        rows.append(
            {
                "id": req_id,
                "kind": kind,
                "story": owner_story(req_id, edges),
                "verified_by": verified_by,
                "planned_by": planned_by,
                "implemented_by": implemented_by,
            }
        )
        bucket = summary.setdefault(
            kind, {"total": 0, "verified": 0, "planned": 0, "uncovered": 0}
        )
        bucket["total"] += 1
        if verified_by:
            bucket["verified"] += 1
        elif planned_by:
            bucket["planned"] += 1
            if kind == "functional":
                diagnostics.append(
                    {
                        "severity": "warning",
                        "code": "TRC-COV-002",
                        "message": f"functional requirement {req_id} is only planned: "
                        "all verifies markers sit on ignored tests",
                        "requirement": req_id,
                        "file": info["file"],
                    }
                )
        else:
            bucket["uncovered"] += 1
            if kind == "functional":
                diagnostics.append(
                    {
                        "severity": "error",
                        "code": "TRC-COV-001",
                        "message": f"functional requirement {req_id} has no verifies marker",
                        "requirement": req_id,
                        "file": info["file"],
                    }
                )
        if not info["kind_declared"]:
            diagnostics.append(
                {
                    "severity": "warning",
                    "code": "TRC-KIND-001",
                    "message": f"requirement {req_id} declares no kind; treated as functional",
                    "requirement": req_id,
                    "file": info["file"],
                }
            )

    diagnostics.sort(key=lambda d: (d["file"], d["code"], d["requirement"]))
    report = {
        "schema_version": SCHEMA_VERSION,
        "diagnostics": diagnostics,
        "requirements": rows,
        "summary": summary,
    }
    errors = sum(1 for d in diagnostics if d["severity"] == "error")
    return report, (1 if errors else 0)


def coverage_text(report):
    lines = [f"{d['file']}: {d['code']}: {d['message']}" for d in report["diagnostics"]]
    lines += ["| requirement | kind | verified by | planned by | implemented by |",
              "| --- | --- | --- | --- | --- |"]
    for row in report["requirements"]:
        lines.append(
            "| {id} | {kind} | {v} | {p} | {i} |".format(
                id=row["id"],
                kind=row["kind"],
                v="; ".join(row["verified_by"]) or "—",
                p="; ".join(row["planned_by"]) or "—",
                i="; ".join(row["implemented_by"]) or "—",
            )
        )
    for kind in sorted(report["summary"]):
        s = report["summary"][kind]
        lines.append(
            f"{kind}: {s['verified']} verified, {s['planned']} planned, "
            f"{s['uncovered']} uncovered (of {s['total']})"
        )
    errors = sum(1 for d in report["diagnostics"] if d["severity"] == "error")
    warnings = sum(1 for d in report["diagnostics"] if d["severity"] == "warning")
    lines.append(f"coverage: {errors} error(s), {warnings} warning(s)")
    return "\n".join(lines)


def check(requirements, edges, req_id):
    """Per-requirement marker report (REQ-03-01-06-01..03)."""
    if req_id not in requirements:
        return {
            "schema_version": SCHEMA_VERSION,
            "requirement": req_id,
            "error": "unknown requirement",
        }, 1
    result = {
        "schema_version": SCHEMA_VERSION,
        "requirement": req_id,
        "verifies": [],
        "implements": [],
        "derived_from": [],
    }
    for e in edges:
        if e["to"] == req_id and e["kind"] in ("verifies", "implements"):
            result[e["kind"]].append(
                {
                    "file": e["from"],
                    "line": e["line"],
                    "ignored": e["ignored"],
                    "test": e["test"],
                }
            )
        elif e["from"] == req_id and e["kind"] == "derived-from":
            result["derived_from"].append(e["to"])
    return result, 0


def matrix(requirements, edges, matrix_type="req-test"):
    """Traceability matrices as CSV with stable headers per type
    (REQ-03-01-02-01/-02/-03): req-test links requirements to markers,
    us-req links stories to their derived requirements."""
    out = io.StringIO()
    writer = csv.writer(out, lineterminator="\n")
    if matrix_type == "us-req":
        writer.writerow(["story", "requirement"])
        pairs = sorted(
            {
                (e["to"], e["from"])
                for e in edges
                if e["kind"] == "derived-from" and str(e["to"]).startswith("US-")
            }
        )
        for story, req_id in pairs:
            writer.writerow([story, req_id])
        return out.getvalue()

    report, _ = coverage(requirements, edges)
    writer.writerow(
        ["requirement", "kind", "verified_markers", "planned_markers", "implements_markers"]
    )
    for row in report["requirements"]:
        writer.writerow(
            [
                row["id"],
                row["kind"],
                ";".join(row["verified_by"]),
                ";".join(row["planned_by"]),
                ";".join(row["implemented_by"]),
            ]
        )
    return out.getvalue()


def emit_json(payload):
    print(json.dumps(payload, indent=2, sort_keys=True))


def run(verb, args, fmt, root="."):
    corpus = read_corpus(root)
    requirements, edges, documents = build_model(corpus)

    if verb == "scan":
        payload = graph(requirements, edges, documents)
        if fmt == "json":
            emit_json(payload)
        else:
            for e in payload["edges"]:
                print(f"{e['from']}:{e['line']}: {e['kind']} {e['to']}")
            print(f"scanned: {len(payload['nodes'])} node(s), {len(edges)} edge(s)")
        return 0

    if verb == "coverage":
        report, code = coverage(requirements, edges)
        if fmt == "json":
            emit_json(report)
        else:
            print(coverage_text(report))
        return code

    if verb == "check":
        if len(args) != 1:
            print("error: `trace check` requires exactly one requirement ID", file=sys.stderr)
            return 2
        result, code = check(requirements, edges, args[0])
        if fmt == "json":
            emit_json(result)
        elif "error" in result:
            print(f"{result['requirement']}: {result['error']}")
        else:
            for kind in ("verifies", "implements"):
                locs = ", ".join(
                    f"{l['file']}:{l['line']}"
                    + (f" ({l['test']})" if l["test"] else "")
                    for l in result[kind]
                ) or "none"
                print(f"{result['requirement']}: {kind}: {locs}")
            stories = ", ".join(result["derived_from"]) or "none"
            print(f"{result['requirement']}: derived-from: {stories}")
        return code

    if verb == "matrix":
        matrix_type = "req-test"
        if args[:1] == ["--type"] and len(args) == 2:
            matrix_type = args[1]
        elif args:
            print("error: `trace matrix` accepts only --type req-test|us-req",
                  file=sys.stderr)
            return 2
        if matrix_type not in ("req-test", "us-req"):
            print(f"error: unknown matrix type '{matrix_type}'", file=sys.stderr)
            return 2
        sys.stdout.write(matrix(requirements, edges, matrix_type))
        return 0

    print(f"error: unknown trace verb '{verb}'", file=sys.stderr)
    return 2


SELFTEST_CASES = []


def case(name):
    def register(fn):
        SELFTEST_CASES.append((name, fn))
        return fn
    return register


REQ_DOC = """---
id: REQ-11-11-11-01
rdf:
  type:
    - arqix:classes/functional-requirement
---
body
"""

REQ_DOC_QUALITY = """---
id: REQ-11-11-11-02
rdf:
  type:
    - arqix:classes/quality-requirement
---
body
"""

REQ_DOC_NO_KIND = """---
id: REQ-11-11-11-03
---
body
"""


@case("rust marker becomes an edge")
def _(assert_eq):
    _, edges, _ = build_model({"a.rs": "// arqix:verifies REQ-11-11-11-01\nfn t() {}\n"})
    assert_eq(len(edges), 1)
    assert_eq(edges[0]["to"], "REQ-11-11-11-01")
    assert_eq(edges[0]["kind"], "verifies")


@case("markdown html comment becomes an edge")
def _(assert_eq):
    _, edges, _ = build_model({"a.md": "<!-- arqix:implements REQ-11-11-11-01 -->\n"})
    assert_eq(len(edges), 1)
    assert_eq(edges[0]["kind"], "implements")


@case("marker inside a string literal is ignored")
def _(assert_eq):
    _, edges, _ = build_model({"a.rs": '    "// arqix:verifies REQ-11-11-11-01",\n'})
    assert_eq(edges, [])


@case("requirement discovery reads id and kind from frontmatter")
def _(assert_eq):
    reqs, _, _ = build_model({"docs/r.md": REQ_DOC, "docs/q.md": REQ_DOC_QUALITY})
    assert_eq(reqs["REQ-11-11-11-01"]["kind"], "functional")
    assert_eq(reqs["REQ-11-11-11-02"]["kind"], "quality")


@case("missing kind defaults to functional")
def _(assert_eq):
    reqs, _, _ = build_model({"docs/r.md": REQ_DOC_NO_KIND})
    assert_eq(reqs["REQ-11-11-11-03"]["kind"], "functional")


@case("unresolved reference stays visible in the graph")
def _(assert_eq):
    reqs, edges, docs = build_model({"a.rs": "// arqix:verifies REQ-99-88-77-66\nfn t() {}\n"})
    g = graph(reqs, edges, docs)
    unresolved = [n for n in g["nodes"] if n.get("unresolved")]
    assert_eq(len(unresolved), 1)
    assert_eq(unresolved[0]["id"], "REQ-99-88-77-66")


@case("uncovered functional requirement is a TRC-COV-001 error")
def _(assert_eq):
    reqs, edges, docs = build_model({"docs/r.md": REQ_DOC})
    report, code = coverage(reqs, edges)
    assert_eq(code, 1)
    assert_eq(report["diagnostics"][0]["code"], "TRC-COV-001")
    assert_eq(report["diagnostics"][0]["severity"], "error")
    assert_eq(report["diagnostics"][0]["file"], "docs/r.md")


@case("undeclared kind is a TRC-KIND-001 warning, not an error")
def _(assert_eq):
    reqs, edges, docs = build_model(
        {"docs/r.md": REQ_DOC_NO_KIND,
         "a.rs": "// arqix:verifies REQ-11-11-11-03\nfn t() {}\n"}
    )
    report, code = coverage(reqs, edges)
    assert_eq(code, 0)
    assert_eq(len(report["diagnostics"]), 1)
    assert_eq(report["diagnostics"][0]["code"], "TRC-KIND-001")
    assert_eq(report["diagnostics"][0]["severity"], "warning")


@case("json outputs carry the schema version")
def _(assert_eq):
    reqs, edges, docs = build_model({"docs/r.md": REQ_DOC})
    assert_eq(graph(reqs, edges, docs)["schema_version"], SCHEMA_VERSION)
    report, _ = coverage(reqs, edges)
    assert_eq(report["schema_version"], SCHEMA_VERSION)
    result, _ = check(reqs, edges, "REQ-11-11-11-01")
    assert_eq(result["schema_version"], SCHEMA_VERSION)


@case("verified functional requirement passes coverage")
def _(assert_eq):
    reqs, edges, docs = build_model(
        {"docs/r.md": REQ_DOC, "a.rs": "// arqix:verifies REQ-11-11-11-01\nfn t() {}\n"}
    )
    report, code = coverage(reqs, edges)
    assert_eq(code, 0)
    assert_eq(
        report["summary"]["functional"],
        {"total": 1, "verified": 1, "planned": 0, "uncovered": 0},
    )


@case("uncovered quality requirement is not a finding")
def _(assert_eq):
    reqs, edges, docs = build_model({"docs/q.md": REQ_DOC_QUALITY})
    _, code = coverage(reqs, edges)
    assert_eq(code, 0)


@case("check reports locations for a requirement")
def _(assert_eq):
    reqs, edges, docs = build_model(
        {"docs/r.md": REQ_DOC, "a.rs": "// arqix:verifies REQ-11-11-11-01\nfn t() {}\n"}
    )
    result, code = check(reqs, edges, "REQ-11-11-11-01")
    assert_eq(code, 0)
    assert_eq(
        result["verifies"],
        [{"file": "a.rs", "line": 1, "ignored": False, "test": "t"}],
    )
    assert_eq(result["implements"], [])


@case("check on unknown requirement is a finding")
def _(assert_eq):
    result, code = check({}, [], "REQ-00-11-22-33")
    assert_eq(code, 1)
    assert_eq(result["error"], "unknown requirement")


@case("matrix header is stable")
def _(assert_eq):
    reqs, edges, docs = build_model({"docs/r.md": REQ_DOC})
    header = matrix(reqs, edges).splitlines()[0]
    assert_eq(
        header,
        "requirement,kind,verified_markers,planned_markers,implements_markers",
    )


IGNORED_TEST = (
    "// arqix:verifies REQ-11-11-11-01\n"
    "#[test]\n"
    '#[ignore = "US-11-11-11: not implemented"]\n'
    "fn t() {}\n"
)


@case("marker on an ignored test plans, it does not verify")
def _(assert_eq):
    reqs, edges, docs = build_model({"docs/r.md": REQ_DOC, "a.rs": IGNORED_TEST})
    assert_eq(edges[0]["ignored"], True)
    report, code = coverage(reqs, edges)
    assert_eq(code, 0)
    assert_eq(
        report["summary"]["functional"],
        {"total": 1, "verified": 0, "planned": 1, "uncovered": 0},
    )
    assert_eq(report["diagnostics"][0]["code"], "TRC-COV-002")
    assert_eq(report["diagnostics"][0]["severity"], "warning")


@case("marker on an active test verifies")
def _(assert_eq):
    active = "// arqix:verifies REQ-11-11-11-01\n#[test]\nfn t() {}\n"
    reqs, edges, docs = build_model({"docs/r.md": REQ_DOC, "a.rs": active})
    assert_eq(edges[0]["ignored"], False)
    report, _ = coverage(reqs, edges)
    assert_eq(report["summary"]["functional"]["verified"], 1)


@case("story is derived from the requirement id")
def _(assert_eq):
    assert_eq(story_of("REQ-11-11-11-01"), "US-11-11-11")
    assert_eq(story_of("REQ-00-00-00-05"), None)
    reqs, edges, docs = build_model({"docs/r.md": REQ_DOC})
    report, _ = coverage(reqs, edges)
    assert_eq(report["requirements"][0]["story"], "US-11-11-11")


REQ_DOC_LINKED = """---
id: REQ-11-11-11-01
iri: arqix:requirements/req-11-11-11-01
rdf:
  type:
    - arqix:classes/functional-requirement
triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-22-22-22
---
body
"""

STORY_DOC = """---
id: US-22-22-22
title: Provide a Linked Example
iri: arqix:user-stories/us-22-22-22
rdf:
  type:
    - arqix:classes/user-story
triples:
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-11-11-11-01
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-22-22
---
body
"""

LINKED_CORPUS = {"docs/r.md": REQ_DOC_LINKED, "docs/s.md": STORY_DOC}


@case("frontmatter triples become graph edges, resolved via iri")
def _(assert_eq):
    _, edges, _ = build_model(LINKED_CORPUS)
    kinds = {(e["from"], e["kind"], e["to"]) for e in edges}
    assert_eq(("REQ-11-11-11-01", "derived-from", "US-22-22-22") in kinds, True)
    assert_eq(("US-22-22-22", "has-requirement", "REQ-11-11-11-01") in kinds, True)


@case("inline object and unresolved workflow iri stay visible")
def _(assert_eq):
    reqs, edges, docs = build_model(LINKED_CORPUS)
    kinds = {(e["kind"], e["to"]) for e in edges}
    assert_eq(("is-part-of-workflow", "arqix:workflows/wf-22-22") in kinds, True)
    g = graph(reqs, edges, docs)
    unresolved = [n for n in g["nodes"] if n.get("unresolved")]
    assert_eq(unresolved, [
        {"id": "arqix:workflows/wf-22-22", "type": "unknown", "unresolved": True}
    ])


@case("owner story comes from derived-from, not the id")
def _(assert_eq):
    reqs, edges, _ = build_model(LINKED_CORPUS)
    report, _ = coverage(reqs, edges)
    assert_eq(report["requirements"][0]["story"], "US-22-22-22")


@case("us-req matrix lists derived pairs")
def _(assert_eq):
    reqs, edges, _ = build_model(LINKED_CORPUS)
    lines = matrix(reqs, edges, "us-req").splitlines()
    assert_eq(lines[0], "story,requirement")
    assert_eq(lines[1], "US-22-22-22,REQ-11-11-11-01")


@case("check reports derived-from stories")
def _(assert_eq):
    reqs, edges, _ = build_model(LINKED_CORPUS)
    result, _ = check(reqs, edges, "REQ-11-11-11-01")
    assert_eq(result["derived_from"], ["US-22-22-22"])


@case("story document becomes a typed node")
def _(assert_eq):
    reqs, edges, docs = build_model(LINKED_CORPUS)
    g = graph(reqs, edges, docs)
    story_nodes = [n for n in g["nodes"] if n["id"] == "US-22-22-22"]
    assert_eq(story_nodes[0]["type"], "user-story")
    assert_eq(docs["US-22-22-22"]["title"], "Provide a Linked Example")


@case("marker edge carries the attached test name")
def _(assert_eq):
    _, edges, _ = build_model({"a.rs": IGNORED_TEST})
    assert_eq(edges[0]["test"], "t")
    _, edges, _ = build_model({"a.md": "<!-- arqix:verifies REQ-11-11-11-01 -->\n"})
    assert_eq(edges[0]["test"], None)


@case("artefact node id is its path, with file per the identity rule")
def _(assert_eq):
    reqs, edges, docs = build_model(
        {"docs/r.md": REQ_DOC, "a.rs": "// arqix:verifies REQ-11-11-11-01\nfn t() {}\n"}
    )
    g = graph(reqs, edges, docs)
    artefacts = [n for n in g["nodes"] if n["type"] == "artefact"]
    assert_eq(artefacts, [{"id": "a.rs", "type": "artefact", "file": "a.rs"}])


@case("model is deterministic regardless of input order")
def _(assert_eq):
    a = {"docs/r.md": REQ_DOC, "a.rs": "// arqix:verifies REQ-11-11-11-01\nfn t() {}\n"}
    b = dict(reversed(list(a.items())))
    assert_eq(build_model(a), build_model(b))


def selftest():
    failed = 0
    for name, fn in SELFTEST_CASES:
        try:
            def assert_eq(actual, expected):
                if actual != expected:
                    raise AssertionError(f"expected {expected!r}, got {actual!r}")
            fn(assert_eq)
            print(f"ok   {name}")
        except AssertionError as exc:
            print(f"FAIL {name}: {exc}")
            failed += 1
    print(f"selftest: {len(SELFTEST_CASES) - failed}/{len(SELFTEST_CASES)} passed")
    return 1 if failed else 0


def main(argv=None):
    argv = sys.argv[1:] if argv is None else argv
    if argv == ["--selftest"]:
        return selftest()
    fmt = "text"
    rest = []
    i = 0
    while i < len(argv):
        if argv[i] == "--format" and i + 1 < len(argv):
            fmt = argv[i + 1]
            i += 2
        else:
            rest.append(argv[i])
            i += 1
    if not rest:
        print("usage: arqix_trace.py scan|check|coverage|matrix [args] [--format json]",
              file=sys.stderr)
        return 2
    return run(rest[0], rest[1:], fmt)


if __name__ == "__main__":
    sys.exit(main())
