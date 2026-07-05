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
FRONTMATTER_ID_RE = re.compile(r"^id:\s*[\"']?(REQ-\d{2}-\d{2}-\d{2}-\d{2})[\"']?\s*$")
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


def build_model(corpus):
    """Build the trace model from a {path: text} corpus.

    Returns (requirements, edges):
      requirements: {req_id: {"kind": short_kind, "file": path}}
      edges: sorted list of {"from", "to", "kind", "line"}
    """
    requirements = {}
    for path, text in sorted(corpus.items()):
        if not path.endswith(".md"):
            continue
        block = frontmatter_lines(text)
        req_id = None
        for line in block:
            if m := FRONTMATTER_ID_RE.match(line.strip()):
                req_id = m.group(1)
                break
        if req_id is None:
            continue
        kind_match = KIND_RE.search("\n".join(block))
        kind = KIND_SHORT[kind_match.group(1)] if kind_match else "functional"
        requirements[req_id] = {
            "kind": kind,
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
                edges.append(
                    {
                        "from": path,
                        "to": target,
                        "kind": link_kind,
                        "line": line_no,
                        "ignored": path.endswith(".rs")
                        and _attached_test_is_ignored(lines, line_no),
                    }
                )
    edges.sort(key=lambda e: (e["from"], e["line"], e["to"], e["kind"]))
    return requirements, edges


def _attached_test_is_ignored(lines, marker_line_no):
    """True when the marker's contiguous comment/attribute block ends in an
    `#[ignore]`d function — a marker on an ignored test plans verification,
    it does not provide it (red-skeleton lifecycle, ADR-0006)."""
    for line in lines[marker_line_no:]:
        stripped = line.strip()
        if stripped.startswith("#[ignore"):
            return True
        if stripped.startswith(("//", "#[")):
            continue
        return False
    return False


def story_of(req_id):
    """Derive the owning story from a requirement ID: REQ-XX-YY-ZZ-NN ->
    US-XX-YY-ZZ; None for the cross-cutting foundation domain 00-00-00."""
    stem = req_id[4:12]
    return None if stem == "00-00-00" else f"US-{stem}"


def graph(requirements, edges):
    """The scan result: the canonical core graph (ADR-0006 layer 1) with
    node and edge collections (REQ-03-01-05-04)."""
    nodes = []
    for req_id in sorted(requirements):
        info = requirements[req_id]
        nodes.append(
            {
                "id": req_id,
                "type": "requirement",
                "kind": info["kind"],
                "kind_declared": info["kind_declared"],
                "story": story_of(req_id),
                "file": info["file"],
            }
        )
    referenced = {e["to"] for e in edges}
    for target in sorted(referenced - set(requirements)):
        nodes.append({"id": target, "type": "requirement", "unresolved": True})
    for source in sorted({e["from"] for e in edges}):
        nodes.append({"id": source, "type": "artefact"})
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
        if e["to"] in links:
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
                "story": story_of(req_id),
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
    }
    for e in edges:
        if e["to"] == req_id:
            result[e["kind"]].append(
                {"file": e["from"], "line": e["line"], "ignored": e["ignored"]}
            )
    return result, 0


def matrix(requirements, edges):
    """Requirement/test matrix as CSV with stable headers (REQ-03-01-02-01/-03)."""
    report, _ = coverage(requirements, edges)
    out = io.StringIO()
    writer = csv.writer(out, lineterminator="\n")
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
    requirements, edges = build_model(corpus)

    if verb == "scan":
        payload = graph(requirements, edges)
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
                locs = ", ".join(f"{l['file']}:{l['line']}" for l in result[kind]) or "none"
                print(f"{result['requirement']}: {kind}: {locs}")
        return code

    if verb == "matrix":
        sys.stdout.write(matrix(requirements, edges))
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
    _, edges = build_model({"a.rs": "// arqix:verifies REQ-11-11-11-01\nfn t() {}\n"})
    assert_eq(len(edges), 1)
    assert_eq(edges[0]["to"], "REQ-11-11-11-01")
    assert_eq(edges[0]["kind"], "verifies")


@case("markdown html comment becomes an edge")
def _(assert_eq):
    _, edges = build_model({"a.md": "<!-- arqix:implements REQ-11-11-11-01 -->\n"})
    assert_eq(len(edges), 1)
    assert_eq(edges[0]["kind"], "implements")


@case("marker inside a string literal is ignored")
def _(assert_eq):
    _, edges = build_model({"a.rs": '    "// arqix:verifies REQ-11-11-11-01",\n'})
    assert_eq(edges, [])


@case("requirement discovery reads id and kind from frontmatter")
def _(assert_eq):
    reqs, _ = build_model({"docs/r.md": REQ_DOC, "docs/q.md": REQ_DOC_QUALITY})
    assert_eq(reqs["REQ-11-11-11-01"]["kind"], "functional")
    assert_eq(reqs["REQ-11-11-11-02"]["kind"], "quality")


@case("missing kind defaults to functional")
def _(assert_eq):
    reqs, _ = build_model({"docs/r.md": REQ_DOC_NO_KIND})
    assert_eq(reqs["REQ-11-11-11-03"]["kind"], "functional")


@case("unresolved reference stays visible in the graph")
def _(assert_eq):
    reqs, edges = build_model({"a.rs": "// arqix:verifies REQ-99-88-77-66\nfn t() {}\n"})
    g = graph(reqs, edges)
    unresolved = [n for n in g["nodes"] if n.get("unresolved")]
    assert_eq(len(unresolved), 1)
    assert_eq(unresolved[0]["id"], "REQ-99-88-77-66")


@case("uncovered functional requirement is a TRC-COV-001 error")
def _(assert_eq):
    reqs, edges = build_model({"docs/r.md": REQ_DOC})
    report, code = coverage(reqs, edges)
    assert_eq(code, 1)
    assert_eq(report["diagnostics"][0]["code"], "TRC-COV-001")
    assert_eq(report["diagnostics"][0]["severity"], "error")
    assert_eq(report["diagnostics"][0]["file"], "docs/r.md")


@case("undeclared kind is a TRC-KIND-001 warning, not an error")
def _(assert_eq):
    reqs, edges = build_model(
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
    reqs, edges = build_model({"docs/r.md": REQ_DOC})
    assert_eq(graph(reqs, edges)["schema_version"], SCHEMA_VERSION)
    report, _ = coverage(reqs, edges)
    assert_eq(report["schema_version"], SCHEMA_VERSION)
    result, _ = check(reqs, edges, "REQ-11-11-11-01")
    assert_eq(result["schema_version"], SCHEMA_VERSION)


@case("verified functional requirement passes coverage")
def _(assert_eq):
    reqs, edges = build_model(
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
    reqs, edges = build_model({"docs/q.md": REQ_DOC_QUALITY})
    _, code = coverage(reqs, edges)
    assert_eq(code, 0)


@case("check reports locations for a requirement")
def _(assert_eq):
    reqs, edges = build_model(
        {"docs/r.md": REQ_DOC, "a.rs": "// arqix:verifies REQ-11-11-11-01\nfn t() {}\n"}
    )
    result, code = check(reqs, edges, "REQ-11-11-11-01")
    assert_eq(code, 0)
    assert_eq(result["verifies"], [{"file": "a.rs", "line": 1, "ignored": False}])
    assert_eq(result["implements"], [])


@case("check on unknown requirement is a finding")
def _(assert_eq):
    result, code = check({}, [], "REQ-00-11-22-33")
    assert_eq(code, 1)
    assert_eq(result["error"], "unknown requirement")


@case("matrix header is stable")
def _(assert_eq):
    reqs, edges = build_model({"docs/r.md": REQ_DOC})
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
    reqs, edges = build_model({"docs/r.md": REQ_DOC, "a.rs": IGNORED_TEST})
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
    reqs, edges = build_model({"docs/r.md": REQ_DOC, "a.rs": active})
    assert_eq(edges[0]["ignored"], False)
    report, _ = coverage(reqs, edges)
    assert_eq(report["summary"]["functional"]["verified"], 1)


@case("story is derived from the requirement id")
def _(assert_eq):
    assert_eq(story_of("REQ-11-11-11-01"), "US-11-11-11")
    assert_eq(story_of("REQ-00-00-00-05"), None)
    reqs, edges = build_model({"docs/r.md": REQ_DOC})
    report, _ = coverage(reqs, edges)
    assert_eq(report["requirements"][0]["story"], "US-11-11-11")


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
