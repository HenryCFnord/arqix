#!/usr/bin/env python3
"""Frontmatter, formatting, and ontology-vocabulary checker for arqix docs.

Checks the architecture documents (stories, requirements, personas,
workflows, ADRs) and the ontology documents (classes, properties,
individuals, index) for frontmatter consistency, canonical formatting,
and correct use of the ontology vocabulary. Complements
scripts/check_requirements.py, which covers the US/REQ link semantics
and the EARS sentence rules.

docs/en/templates/ (placeholder values) and docs/en/plans/ (different
frontmatter schema) are intentionally out of scope.

Like check_requirements.py, this script is a stdlib-only reference
implementation for the later arqix (Rust) checker: stable rule IDs,
deterministic output ordering, --json output, and stable exit codes.

Exit codes:
    0  no findings
    1  findings (errors or warnings) reported
    2  usage or I/O error

Rule IDs:
    FMT-001  frontmatter must open with '---' on line 1 and be closed
    FMT-002  blank line(s) directly after the opening '---'
    FMT-003  top-level keys unknown or out of canonical order
    FMT-004  trailing whitespace, or file does not end with exactly
             one newline
    FMT-005  meta.created/meta.updated not ISO YYYY-MM-DD, or
             created > updated
    FMT-006  meta.lang does not match the document tree (en)
    FM-001   required key missing or empty for the document family
    FM-002   id does not match the filename convention
    FM-003   iri does not match the expected namespace/id form
    FM-004   slug does not match the filename tail (architecture docs)
    FM-005   first body heading does not match the expected heading
    FM-006   duplicate id or iri across scanned documents
    ONT-001  triple predicate is not a defined ontology property
    ONT-002  arqix: rdf.type is not a defined ontology class
    ONT-003  arqix: triple object does not resolve to a scanned document
    ONT-004  rdfs sub-class-of/domain/range target is not a defined class
    ONT-005  owl inverse-of target has no property document (warning;
             suppressed by --allow-undefined-inverse)
    ONT-006  ontology index lists a class that is not defined
"""

import argparse
import json
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from check_requirements import Finding  # noqa: E402

ISO_DATE = re.compile(r"^\d{4}-\d{2}-\d{2}$")
NON_ARQIX_TYPES = {"rdfs:Class", "rdf:Property"}
REQUIRED_META = ["lifecycle-status", "owner", "created", "updated", "lang", "generated"]

# family -> (directory, canonical top-level key order, required keys)
FAMILIES = {
    "story": ("docs/en/architecture/stories",
              ["id", "title", "slug", "iri", "rdf", "triples", "properties",
               "external-references", "meta"],
              ["id", "title", "slug", "iri"]),
    "req": ("docs/en/architecture/req",
            ["id", "title", "slug", "iri", "rdf", "triples", "properties",
             "external-references", "meta"],
            ["id", "title", "slug", "iri"]),
    "persona": ("docs/en/architecture/personas",
                ["id", "title", "slug", "iri", "rdf", "triples", "properties",
                 "external-references", "meta"],
                ["id", "title", "slug", "iri"]),
    "workflow": ("docs/en/architecture/workflows",
                 ["id", "title", "slug", "iri", "rdf", "triples", "properties",
                  "external-references", "meta"],
                 ["id", "title", "slug", "iri"]),
    "adr": ("docs/en/architecture/adr",
            ["id", "title", "slug", "iri", "rdf", "triples", "properties",
             "external-references", "meta"],
            ["id", "title", "slug", "iri"]),
    "ont-class": ("docs/ontology/classes",
                  ["id", "label", "iri", "rdf", "rdfs", "triples", "properties",
                   "external-references", "owl", "meta"],
                  ["id", "label", "iri"]),
    "ont-property": ("docs/ontology/properties",
                     ["id", "label", "iri", "rdf", "rdfs", "owl", "triples",
                      "properties", "external-references", "meta"],
                     ["id", "label", "iri"]),
    "ont-individual": ("docs/ontology/individuals",
                       ["id", "label", "iri", "rdf", "triples", "properties",
                        "external-references", "meta"],
                       ["id", "label", "iri"]),
}

# architecture family -> (id prefix, iri namespace)
ARCH_NS = {
    "story": ("US-", "arqix:user-stories/"),
    "req": ("REQ-", "arqix:requirements/"),
    "persona": ("PER-", "arqix:personas/"),
    "workflow": ("WF-", "arqix:workflows/"),
    "adr": ("ADR-", "arqix:adrs/"),
}
ONT_ID_PREFIX = {"ont-class": "class-", "ont-property": "property-",
                 "ont-individual": "individual-"}
ONT_NS = {"ont-class": "arqix:classes/", "ont-property": "arqix:properties/",
          "ont-individual": "arqix:individuals/"}


class Doc:
    """Parsed arqix document with enough raw detail for format checks."""

    def __init__(self, path, text, family):
        self.path = path
        self.family = family
        self.lines = text.split("\n")
        self.text = text
        self.top_keys = []          # top-level keys in file order
        self.scalars = {}           # top-level scalar key -> value
        self.rdf_types = []
        self.triples = []           # (predicate, [objects])
        self.rdfs = {}              # subkey -> [values]
        self.owl = {}               # subkey -> value
        self.meta = {}
        self.body = ""
        self.fm_ok = self._parse()

    def _parse(self):
        lines = self.lines
        if not lines or lines[0].strip() != "---":
            return False
        end = 1
        while end < len(lines) and lines[end].strip() != "---":
            end += 1
        if end >= len(lines):
            return False
        self.fm_lines = lines[1:end]
        self.body = "\n".join(lines[end + 1:])

        section = None
        subsection = None
        current_triple = None
        for raw in self.fm_lines:
            if not raw.strip() or raw.lstrip().startswith("#"):
                continue
            indent = len(raw) - len(raw.lstrip(" "))
            line = raw.strip()
            if indent == 0:
                current_triple = None
                subsection = None
                m = re.match(r"^([A-Za-z0-9_-]+):\s*(.*)$", line)
                if not m:
                    continue
                key, value = m.group(1), m.group(2).strip()
                if len(value) >= 2 and value[0] == '"' and value[-1] == '"':
                    value = value[1:-1]
                section = key
                self.top_keys.append(key)
                if value not in ("", "{}", "[]"):
                    self.scalars[key] = value
            elif section == "rdf" and line.startswith("- "):
                self.rdf_types.append(line[2:].strip())
            elif section == "rdfs":
                m = re.match(r"^([A-Za-z0-9_-]+):\s*(.*)$", line)
                if m and indent == 2:
                    subsection = m.group(1)
                    self.rdfs.setdefault(subsection, [])
                    if m.group(2).strip():
                        self.rdfs[subsection].append(m.group(2).strip())
                elif line.startswith("- ") and subsection:
                    self.rdfs[subsection].append(line[2:].strip())
            elif section == "owl":
                m = re.match(r"^([A-Za-z0-9_-]+):\s*(.*)$", line)
                if m and m.group(2).strip():
                    self.owl[m.group(1)] = m.group(2).strip()
            elif section == "triples":
                m = re.match(r"^-\s*predicate:\s*(.*)$", line)
                if m:
                    current_triple = (m.group(1).strip(), [])
                    self.triples.append(current_triple)
                    continue
                m = re.match(r"^object:\s*(.*)$", line)
                if m and current_triple is not None:
                    if m.group(1).strip():
                        current_triple[1].append(m.group(1).strip())
                    continue
                m = re.match(r"^-\s*(\S.*)$", line)
                if m and current_triple is not None:
                    current_triple[1].append(m.group(1).strip())
            elif section == "meta":
                m = re.match(r"^([A-Za-z0-9_-]+):\s*(.*)$", line)
                if m and m.group(2).strip():
                    self.meta[m.group(1)] = m.group(2).strip()
        return True

    def first_heading(self):
        for line in self.body.split("\n"):
            if line.startswith("## "):
                return line[3:].strip()
            if line.startswith("# "):
                return line[2:].strip()
        return None


def is_subsequence(observed, canonical):
    it = iter(canonical)
    return all(any(key == c for c in it) for key in observed)


def check_format(doc, findings):
    path = doc.path
    if not doc.fm_ok:
        findings.append(Finding(path, "FMT-001", "error",
                                "frontmatter must open with '---' on line 1 and be closed"))
        return
    if doc.fm_lines and doc.fm_lines[0].strip() == "":
        findings.append(Finding(path, "FMT-002", "error",
                                "blank line directly after the opening '---'"))
    order, _ = FAMILIES[doc.family][1], None
    known = [k for k in doc.top_keys if k in order]
    unknown = [k for k in doc.top_keys if k not in order]
    for key in unknown:
        findings.append(Finding(path, "FMT-003", "error",
                                "unknown top-level key %r for family %s" % (key, doc.family)))
    if not is_subsequence(known, order):
        findings.append(Finding(path, "FMT-003", "error",
                                "top-level keys out of canonical order: %s (expected order %s)"
                                % (known, order)))
    for i, raw in enumerate(doc.lines, start=1):
        if raw != raw.rstrip():
            findings.append(Finding(path, "FMT-004", "error",
                                    "trailing whitespace on line %d" % i))
            break
    if not doc.text.endswith("\n") or doc.text.endswith("\n\n"):
        findings.append(Finding(path, "FMT-004", "error",
                                "file must end with exactly one newline"))
    created, updated = doc.meta.get("created"), doc.meta.get("updated")
    for key, value in (("created", created), ("updated", updated)):
        if value and not ISO_DATE.match(value):
            findings.append(Finding(path, "FMT-005", "error",
                                    "meta.%s %r is not ISO YYYY-MM-DD" % (key, value)))
    if created and updated and ISO_DATE.match(created) and ISO_DATE.match(updated):
        if created > updated:
            findings.append(Finding(path, "FMT-005", "error",
                                    "meta.created %s is after meta.updated %s" % (created, updated)))
    if doc.meta.get("lang") and doc.meta["lang"] != "en":
        findings.append(Finding(path, "FMT-006", "error",
                                "meta.lang %r, expected 'en'" % doc.meta["lang"]))


def check_frontmatter(doc, findings):
    path = doc.path
    _, order, required = FAMILIES[doc.family][0], FAMILIES[doc.family][1], FAMILIES[doc.family][2]
    for key in required:
        if not doc.scalars.get(key):
            findings.append(Finding(path, "FM-001", "error",
                                    "required key %r missing or empty" % key))
    if not doc.rdf_types:
        findings.append(Finding(path, "FM-001", "error", "rdf.type missing or empty"))
    for key in REQUIRED_META:
        if key not in doc.meta:
            findings.append(Finding(path, "FM-001", "error",
                                    "meta.%s missing or empty" % key))

    filename = Path(path).name
    doc_id = doc.scalars.get("id", "")
    iri = doc.scalars.get("iri", "")

    if doc.family in ARCH_NS:
        prefix, ns = ARCH_NS[doc.family]
        if doc_id:
            if not doc_id.startswith(prefix) or not filename.startswith(doc_id + "-"):
                findings.append(Finding(path, "FM-002", "error",
                                        "id %r is not a %s* prefix of the filename" % (doc_id, prefix)))
            expected_iri = ns + doc_id.lower()
            if iri and iri != expected_iri:
                findings.append(Finding(path, "FM-003", "error",
                                        "iri %r, expected %r" % (iri, expected_iri)))
            slug = doc.scalars.get("slug", "")
            if slug and filename.startswith(doc_id + "-"):
                tail = filename[len(doc_id) + 1:-3]
                if slug != tail:
                    findings.append(Finding(path, "FM-004", "error",
                                            "slug %r does not match filename tail %r" % (slug, tail)))
        expected_heading = "Requirement" if doc.family == "req" else doc.scalars.get("title", "")
    else:
        label = doc.scalars.get("label", "")
        id_prefix = ONT_ID_PREFIX[doc.family]
        if label:
            if filename != label + ".md":
                findings.append(Finding(path, "FM-002", "error",
                                        "filename %r, expected %r" % (filename, label + ".md")))
            if doc_id != id_prefix + label:
                findings.append(Finding(path, "FM-002", "error",
                                        "id %r, expected %r" % (doc_id, id_prefix + label)))
            expected_iri = ONT_NS[doc.family] + label
            if iri and iri != expected_iri:
                findings.append(Finding(path, "FM-003", "error",
                                        "iri %r, expected %r" % (iri, expected_iri)))
        expected_heading = label

    heading = doc.first_heading()
    if expected_heading:
        if heading is None or heading.lower() != expected_heading.lower():
            findings.append(Finding(path, "FM-005", "error",
                                    "first heading %r, expected %r" % (heading, expected_heading)))


def check_vocabulary(doc, vocab, findings, allow_undefined_inverse):
    path = doc.path
    classes, properties, all_iris = vocab
    for predicate, objects in doc.triples:
        if predicate.startswith("arqix:") and predicate not in properties:
            findings.append(Finding(path, "ONT-001", "error",
                                    "predicate %s is not a defined ontology property" % predicate))
        for obj in objects:
            if obj.startswith("arqix:") and obj not in all_iris:
                findings.append(Finding(path, "ONT-003", "error",
                                        "triple object %s does not resolve to a scanned document" % obj))
    for rdf_type in doc.rdf_types:
        if rdf_type.startswith("arqix:"):
            if rdf_type not in classes:
                findings.append(Finding(path, "ONT-002", "error",
                                        "rdf.type %s is not a defined ontology class" % rdf_type))
        elif rdf_type not in NON_ARQIX_TYPES:
            findings.append(Finding(path, "ONT-002", "error",
                                    "rdf.type %s is neither an arqix class nor an allowed external type" % rdf_type))
    for subkey in ("sub-class-of", "domain", "range"):
        for target in doc.rdfs.get(subkey, []):
            if target.startswith("arqix:") and target not in classes:
                findings.append(Finding(path, "ONT-004", "error",
                                        "rdfs.%s target %s is not a defined class" % (subkey, target)))
    inverse = doc.owl.get("inverse-of")
    if inverse and inverse.startswith("arqix:") and inverse not in properties:
        if not allow_undefined_inverse:
            findings.append(Finding(path, "ONT-005", "warning",
                                    "owl.inverse-of %s has no property document" % inverse))


def check_index(root, classes_by_label, findings):
    path = root / "docs/ontology/index.md"
    if not path.is_file():
        return
    text = path.read_text(encoding="utf-8")
    body = text.split("---\n", 2)[-1]
    for m in re.finditer(r"^- ([a-z0-9-]+)$", body, re.MULTILINE):
        name = m.group(1)
        if name not in classes_by_label:
            findings.append(Finding(path, "ONT-006", "error",
                                    "index lists class %r which is not defined" % name))


def load_docs(root):
    docs = []
    for family, (rel_dir, _, _) in FAMILIES.items():
        d = root / rel_dir
        if not d.is_dir():
            continue
        for path in sorted(d.glob("*.md")):
            docs.append(Doc(path, path.read_text(encoding="utf-8"), family))
    return docs


def run_checks(root, allow_undefined_inverse):
    findings = []
    docs = load_docs(root)

    classes = {d.scalars.get("iri") for d in docs if d.family == "ont-class"}
    properties = {d.scalars.get("iri") for d in docs if d.family == "ont-property"}
    all_iris = {d.scalars.get("iri") for d in docs if d.scalars.get("iri")}
    classes_by_label = {d.scalars.get("label") for d in docs if d.family == "ont-class"}
    vocab = (classes, properties, all_iris)

    seen_ids, seen_iris = {}, {}
    for doc in docs:
        check_format(doc, findings)
        if not doc.fm_ok:
            continue
        check_frontmatter(doc, findings)
        check_vocabulary(doc, vocab, findings, allow_undefined_inverse)
        for kind, seen, value in (("id", seen_ids, doc.scalars.get("id")),
                                  ("iri", seen_iris, doc.scalars.get("iri"))):
            if not value:
                continue
            if value in seen:
                findings.append(Finding(doc.path, "FM-006", "error",
                                        "duplicate %s %r (also in %s)" % (kind, value, seen[value])))
            else:
                seen[value] = Path(doc.path).name

    check_index(root, classes_by_label, findings)
    findings.sort(key=lambda f: (f.path, f.rule, f.message))
    return findings


def report(findings, as_json):
    errors = sum(1 for f in findings if f.level == "error")
    warnings = len(findings) - errors
    if as_json:
        print(json.dumps({"findings": [f.as_dict() for f in findings],
                          "summary": {"errors": errors, "warnings": warnings}}, indent=2))
    else:
        for f in findings:
            print("%s: [%s] %s: %s" % (f.level, f.rule, f.path, f.message))
        print("checked: %d error(s), %d warning(s)" % (errors, warnings))
    return 1 if findings else 0


# --- selftest -----------------------------------------------------------

GOOD_CLASS = """---
id: class-widget
label: widget
iri: arqix:classes/widget

rdf:
  type:
    - rdfs:Class

rdfs:
  sub-class-of:
    - arqix:classes/widget

triples: []

properties: {}

external-references: []

owl: {}

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-02
  updated: 2026-07-02
  lang: en
  generated: false
---

## Widget

A selftest fixture class.
"""

GOOD_STORY = """---
id: US-01-01-01
title: Test Story
slug: test-story
iri: arqix:user-stories/us-01-01-01

rdf:
  type:
    - arqix:classes/widget

triples:
  - predicate: arqix:properties/points-at
    object: arqix:classes/widget

properties:
  priority: high

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-01
  updated: 2026-07-02
  lang: en
  generated: false
---

## Test Story

As a tester, I want fixtures, so that the selftest is honest.
"""


def selftest():
    failures = []
    vocab = ({"arqix:classes/widget"}, {"arqix:properties/points-at"},
             {"arqix:classes/widget", "arqix:properties/points-at",
              "arqix:user-stories/us-01-01-01"})

    def run(name, text, family, expected_rules, mutate=None):
        if mutate:
            text = mutate(text)
        doc = Doc(Path(name), text, family)
        findings = []
        check_format(doc, findings)
        if doc.fm_ok:
            check_frontmatter(doc, findings)
            check_vocabulary(doc, vocab, findings, allow_undefined_inverse=False)
        rules = sorted({f.rule for f in findings})
        if rules != sorted(expected_rules):
            failures.append("%s: got %s, expected %s" % (name, rules, sorted(expected_rules)))

    run("widget.md", GOOD_CLASS, "ont-class", [])
    run("US-01-01-01-test-story.md", GOOD_STORY, "story", [])
    run("US-01-01-01-test-story.md", GOOD_STORY, "story", [],
        lambda t: t.replace("title: Test Story", 'title: "Test Story"')
                   .replace("## Test Story", "## Test Story"))
    run("US-01-01-01-test-story.md", GOOD_STORY, "story", ["FMT-001"],
        lambda t: "\n" + t)
    run("US-01-01-01-test-story.md", GOOD_STORY, "story", ["FMT-002"],
        lambda t: t.replace("---\nid:", "---\n\nid:", 1))
    run("US-01-01-01-test-story.md", GOOD_STORY, "story", ["FMT-003"],
        lambda t: t.replace("title: Test Story\nslug: test-story",
                            "slug: test-story\ntitle: Test Story", 1))
    run("US-01-01-01-test-story.md", GOOD_STORY, "story", ["FMT-004"],
        lambda t: t.replace("priority: high", "priority: high "))
    run("US-01-01-01-test-story.md", GOOD_STORY, "story", ["FMT-005"],
        lambda t: t.replace("created: 2026-07-01", "created: 2026-07-03"))
    run("US-01-01-01-test-story.md", GOOD_STORY, "story", ["FMT-006"],
        lambda t: t.replace("lang: en", "lang: de"))
    run("US-01-01-01-test-story.md", GOOD_STORY, "story", ["FM-001"],
        lambda t: t.replace("slug: test-story", "slug:"))
    run("US-01-01-99-test-story.md", GOOD_STORY, "story", ["FM-002"])
    run("US-01-01-01-test-story.md", GOOD_STORY, "story", ["FM-003"],
        lambda t: t.replace("iri: arqix:user-stories/us-01-01-01",
                            "iri: arqix:user-stories/wrong"))
    run("US-01-01-01-other-slug.md", GOOD_STORY, "story", ["FM-004"])
    run("US-01-01-01-test-story.md", GOOD_STORY, "story", ["FM-005"],
        lambda t: t.replace("## Test Story", "## Something Else"))
    run("US-01-01-01-test-story.md", GOOD_STORY, "story", ["ONT-001"],
        lambda t: t.replace("arqix:properties/points-at", "arqix:properties/undefined"))
    run("US-01-01-01-test-story.md", GOOD_STORY, "story", ["ONT-002"],
        lambda t: t.replace("- arqix:classes/widget\n\ntriples",
                            "- arqix:classes/undefined\n\ntriples", 1))
    run("US-01-01-01-test-story.md", GOOD_STORY, "story", ["ONT-003"],
        lambda t: t.replace("object: arqix:classes/widget", "object: arqix:classes/nowhere"))
    run("widget.md", GOOD_CLASS, "ont-class", ["ONT-004"],
        lambda t: t.replace("sub-class-of:\n    - arqix:classes/widget",
                            "sub-class-of:\n    - arqix:classes/nowhere"))
    run("widget.md", GOOD_CLASS, "ont-class", ["ONT-005"],
        lambda t: t.replace("owl: {}", "owl:\n  inverse-of: arqix:properties/nowhere"))
    run("wrong-name.md", GOOD_CLASS, "ont-class", ["FM-002"])

    if failures:
        for f in failures:
            print("selftest FAIL: %s" % f)
        return 1
    print("selftest OK: 20 fixture cases")
    return 0


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    parser.add_argument("--root", default=".", help="repository root (default: .)")
    parser.add_argument("--json", action="store_true", help="machine-readable output")
    parser.add_argument("--allow-undefined-inverse", action="store_true",
                        help="suppress ONT-005 warnings for owl.inverse-of names "
                             "that have no property document yet")
    parser.add_argument("--selftest", action="store_true", help="run built-in examples")
    args = parser.parse_args(argv)

    if args.selftest:
        return selftest()

    root = Path(args.root)
    if not (root / "docs/ontology").is_dir():
        print("error: docs/ontology not found under %s" % root, file=sys.stderr)
        return 2
    findings = run_checks(root, args.allow_undefined_inverse)
    return report(findings, args.json)


if __name__ == "__main__":
    sys.exit(main())
