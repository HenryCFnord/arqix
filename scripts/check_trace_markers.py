#!/usr/bin/env python3
"""Check trace markers in Rust tests against the requirements corpus.

Reference checker for the TDD gate (AGENTS.md, "Test-driven
implementation"): every test function carries either an
``// arqix:verifies REQ-…`` marker or an explicit ``// arqix:no-requirement``
annotation, markers reference existing requirements, and ignored tests name
their owning story. Like its siblings (``check_requirements.py``,
``check_frontmatter.py``) this script is stdlib-only and serves as the
behavioural oracle for the later Rust implementation of ``trace scan`` /
``trace check``.

Rules:
  TRC-001  error    marker references a requirement ID that does not exist
  TRC-002  error    test function has neither a verifies marker nor an
                    explicit arqix:no-requirement annotation
  TRC-003  error    ignored test reason does not match
                    'US-XX-YY-ZZ: <text>' or names an unknown story
  TRC-004  error    malformed marker payload (not REQ-XX-YY-ZZ-NN)
  TRC-005  error    test carries both a verifies marker and
                    arqix:no-requirement
  TRC-006  error    derived-from and has-requirement backlinks are
                    asymmetric between a requirement and a story

Exit codes: 0 = clean, 1 = findings, 2 = usage/internal error.
"""

import argparse
import json
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from arqix_trace import build_model, read_corpus  # noqa: E402

REPO_ROOT = Path(__file__).resolve().parent.parent
REQ_DIR = REPO_ROOT / "docs/en/architecture/req"
STORY_DIR = REPO_ROOT / "docs/en/architecture/stories"
TEST_DIR = REPO_ROOT / "tests"
SRC_DIR = REPO_ROOT / "src"

REQ_ID_RE = re.compile(r"^REQ-\d{2}-\d{2}-\d{2}-\d{2}$")
MARKER_RE = re.compile(r"//\s*arqix:verifies\s+(\S+)")
NO_REQ_RE = re.compile(r"//\s*arqix:no-requirement\b")
TEST_ATTR_RE = re.compile(r"^\s*#\[test\]")
IGNORE_ATTR_RE = re.compile(r"^\s*#\[ignore(?:\s*=\s*\"([^\"]*)\")?\]")
FN_RE = re.compile(r"^\s*(?:pub\s+)?fn\s+\w+")
IGNORE_REASON_RE = re.compile(r"^(US-\d{2}-\d{2}-\d{2}): .+")
COMMENT_OR_ATTR_RE = re.compile(r"^\s*(//|#\[)")


def known_requirement_ids():
    return set(known_requirement_kinds())


KIND_RE = re.compile(
    r"arqix:classes/(functional-requirement|quality-requirement|constraint)"
)
KIND_SHORT = {
    "functional-requirement": "functional",
    "quality-requirement": "quality",
    "constraint": "constraint",
}


def known_requirement_kinds():
    """Map requirement ID -> {kind, declared, file}.

    A requirement without a declared kind is treated as functional (the
    strictest default, ADR-0006) and reported as a TRC-KIND-001 warning.
    """
    kinds = {}
    for p in sorted(REQ_DIR.glob("REQ-*.md")):
        m = re.match(r"REQ-\d{2}-\d{2}-\d{2}-\d{2}", p.name)
        if not m:
            continue
        kind_match = KIND_RE.search(p.read_text(encoding="utf-8"))
        kinds[m.group(0)] = {
            "kind": KIND_SHORT[kind_match.group(1)] if kind_match else "functional",
            "declared": kind_match is not None,
            "file": str(p.relative_to(REPO_ROOT)),
        }
    return kinds


def known_story_ids():
    return {
        m.group(0)
        for p in sorted(STORY_DIR.glob("US-*.md"))
        if (m := re.match(r"US-\d{2}-\d{2}-\d{2}", p.name))
    }


def check_test_file(text, known_reqs, known_stories, path):
    """Scan one Rust test file; return a sorted list of findings."""
    findings = []
    markers = []  # (line_no, payload)
    no_req_lines = []
    is_test = False
    ignore = None  # (line_no, reason or None)

    def reset():
        nonlocal markers, no_req_lines, is_test, ignore
        markers, no_req_lines, is_test, ignore = [], [], False, None

    for line_no, line in enumerate(text.splitlines(), start=1):
        stripped = line.strip()
        if not stripped:
            reset()
            continue
        if m := MARKER_RE.search(line):
            markers.append((line_no, m.group(1)))
            continue
        if NO_REQ_RE.search(line):
            no_req_lines.append(line_no)
            continue
        if TEST_ATTR_RE.match(line):
            is_test = True
            continue
        if m := IGNORE_ATTR_RE.match(line):
            ignore = (line_no, m.group(1))
            continue
        if COMMENT_OR_ATTR_RE.match(line):
            continue
        if FN_RE.match(line) and is_test:
            for marker_line, payload in markers:
                if not REQ_ID_RE.match(payload):
                    findings.append((path, marker_line, "TRC-004",
                                     f"malformed marker payload '{payload}' "
                                     "(expected REQ-XX-YY-ZZ-NN)"))
                elif payload not in known_reqs:
                    findings.append((path, marker_line, "TRC-001",
                                     f"marker references unknown requirement {payload}"))
            if not markers and not no_req_lines:
                findings.append((path, line_no, "TRC-002",
                                 "test has neither an arqix:verifies marker nor "
                                 "an arqix:no-requirement annotation"))
            if markers and no_req_lines:
                findings.append((path, line_no, "TRC-005",
                                 "test carries both a verifies marker and "
                                 "arqix:no-requirement"))
            if ignore is not None:
                ignore_line, reason = ignore
                m = IGNORE_REASON_RE.match(reason or "")
                if not m:
                    findings.append((path, ignore_line, "TRC-003",
                                     "ignore reason must be 'US-XX-YY-ZZ: <text>', "
                                     f"got {reason!r}"))
                elif m.group(1) not in known_stories:
                    findings.append((path, ignore_line, "TRC-003",
                                     f"ignore reason names unknown story {m.group(1)}"))
            reset()
        elif FN_RE.match(line):
            reset()
        else:
            reset()

    return sorted(findings)


def check_marker_targets(text, known_reqs, path):
    """Validate marker payloads in non-test sources (src/)."""
    findings = []
    for line_no, line in enumerate(text.splitlines(), start=1):
        if m := MARKER_RE.search(line):
            payload = m.group(1)
            if not REQ_ID_RE.match(payload):
                findings.append((path, line_no, "TRC-004",
                                 f"malformed marker payload '{payload}' "
                                 "(expected REQ-XX-YY-ZZ-NN)"))
            elif payload not in known_reqs:
                findings.append((path, line_no, "TRC-001",
                                 f"marker references unknown requirement {payload}"))
    return sorted(findings)


def check_backlinks(edges):
    """TRC-006: the canonical-owner model keeps derived-from (REQ -> US)
    and has-requirement (US -> REQ) as double bookkeeping; the pairs must
    stay symmetric. Reports the missing counterpart at the location of the
    existing edge."""
    derived = {
        (e["from"], e["to"]): e
        for e in edges
        if e["kind"] == "derived-from" and str(e["to"]).startswith("US-")
    }
    backlinks = {
        (e["to"], e["from"]): e
        for e in edges
        if e["kind"] == "has-requirement" and str(e["from"]).startswith("US-")
    }
    findings = []
    for pair, e in derived.items():
        if pair not in backlinks:
            findings.append((e["file"], e["line"], "TRC-006",
                             f"{pair[0]} is derived-from {pair[1]}, but the story "
                             "has no has-requirement backlink"))
    for pair, e in backlinks.items():
        if pair not in derived:
            findings.append((e["file"], e["line"], "TRC-006",
                             f"{pair[1]} lists {pair[0]} via has-requirement, but the "
                             "requirement has no derived-from counterpart"))
    return sorted(findings)


def collect_referenced_reqs(paths):
    refs = set()
    for p in paths:
        for m in MARKER_RE.finditer(p.read_text(encoding="utf-8")):
            if REQ_ID_RE.match(m.group(1)):
                refs.add(m.group(1))
    return refs


def run_checks(emit_json):
    kinds = known_requirement_kinds()
    known_reqs = set(kinds)
    known_stories = known_story_ids()

    test_files = sorted(
        p for p in TEST_DIR.rglob("*.rs") if "fixtures" not in p.parts
    )
    src_files = sorted(SRC_DIR.rglob("*.rs")) if SRC_DIR.exists() else []

    findings = []
    for p in test_files:
        rel = str(p.relative_to(REPO_ROOT))
        findings.extend(check_test_file(p.read_text(encoding="utf-8"),
                                        known_reqs, known_stories, rel))
    for p in src_files:
        rel = str(p.relative_to(REPO_ROOT))
        findings.extend(check_marker_targets(p.read_text(encoding="utf-8"),
                                             known_reqs, rel))
    _, corpus_edges, _ = build_model(read_corpus(REPO_ROOT))
    findings.extend(check_backlinks(corpus_edges))
    findings.sort()

    referenced = collect_referenced_reqs(test_files) & known_reqs
    coverage = {}
    for kind in ("functional", "quality", "constraint"):
        total = sum(1 for k in kinds.values() if k["kind"] == kind)
        hit = sum(1 for r in referenced if kinds[r]["kind"] == kind)
        coverage[kind] = {"total": total, "referenced": hit}

    warnings = [
        (info["file"], "TRC-KIND-001",
         f"requirement {req_id} declares no kind; treated as functional")
        for req_id, info in sorted(kinds.items())
        if not info["declared"]
    ]

    if emit_json:
        print(json.dumps({
            "findings": [
                {"file": f, "line": l, "rule": r, "message": m}
                for f, l, r, m in findings
            ],
            "warnings": [
                {"file": f, "rule": r, "message": m}
                for f, r, m in warnings
            ],
            "tests_files": len(test_files),
            "coverage_by_kind": coverage,
        }, indent=2, sort_keys=True))
    else:
        for f, l, r, m in findings:
            print(f"{f}:{l}: {r}: {m}")
        for f, r, m in warnings:
            print(f"{f}: {r}: warning: {m}")
        by_kind = ", ".join(
            f"{kind} {c['referenced']}/{c['total']}"
            for kind, c in coverage.items()
        )
        print(f"checked: {len(findings)} error(s), {len(warnings)} warning(s) — "
              f"referenced by verifies markers: {by_kind}")
    return 1 if findings else 0


SELFTEST_REQS = {"REQ-01-01-16-01", "REQ-01-01-16-02"}
SELFTEST_STORIES = {"US-01-01-16"}

SELFTEST_CASES = [
    ("clean marked test", """\
// arqix:verifies REQ-01-01-16-01
#[test]
#[ignore = "US-01-01-16: not implemented"]
fn a() {
""", []),
    ("clean no-requirement test", """\
// arqix:no-requirement
#[test]
fn a() {
""", []),
    ("unknown requirement", """\
// arqix:verifies REQ-99-99-99-99
#[test]
fn a() {
""", ["TRC-001"]),
    ("missing marker", """\
#[test]
fn a() {
""", ["TRC-002"]),
    ("bad ignore reason", """\
// arqix:verifies REQ-01-01-16-01
#[test]
#[ignore = "todo"]
fn a() {
""", ["TRC-003"]),
    ("unknown story in ignore reason", """\
// arqix:verifies REQ-01-01-16-01
#[test]
#[ignore = "US-99-99-99: not implemented"]
fn a() {
""", ["TRC-003"]),
    ("ignore without reason", """\
// arqix:verifies REQ-01-01-16-01
#[test]
#[ignore]
fn a() {
""", ["TRC-003"]),
    ("malformed marker payload", """\
// arqix:verifies REQ-1-2-3
#[test]
fn a() {
""", ["TRC-004"]),
    ("contradictory annotations", """\
// arqix:verifies REQ-01-01-16-01
// arqix:no-requirement
#[test]
fn a() {
""", ["TRC-005"]),
    ("marker separated by blank line does not attach", """\
// arqix:verifies REQ-01-01-16-01

#[test]
fn a() {
""", ["TRC-002"]),
    ("two markers on one test", """\
// arqix:verifies REQ-01-01-16-01
// arqix:verifies REQ-01-01-16-02
#[test]
fn a() {
""", []),
    ("helper fn is not a test", """\
fn helper() {
""", []),
]


DERIVED_EDGE = {"from": "REQ-01-01-16-01", "to": "US-01-01-16",
                "kind": "derived-from", "file": "r.md", "line": 5}
BACKLINK_EDGE = {"from": "US-01-01-16", "to": "REQ-01-01-16-01",
                 "kind": "has-requirement", "file": "s.md", "line": 7}

BACKLINK_CASES = [
    ("symmetric backlinks are clean", [DERIVED_EDGE, BACKLINK_EDGE], []),
    ("missing has-requirement backlink", [DERIVED_EDGE], ["TRC-006"]),
    ("missing derived-from counterpart", [BACKLINK_EDGE], ["TRC-006"]),
]


def selftest():
    failed = 0
    for name, text, expected_rules in SELFTEST_CASES:
        findings = check_test_file(text, SELFTEST_REQS, SELFTEST_STORIES, "t.rs")
        rules = [r for _, _, r, _ in findings]
        if rules != expected_rules:
            print(f"FAIL {name}: expected {expected_rules}, got {rules}")
            failed += 1
        else:
            print(f"ok   {name}")
    for name, edges, expected_rules in BACKLINK_CASES:
        rules = [r for _, _, r, _ in check_backlinks(edges)]
        if rules != expected_rules:
            print(f"FAIL {name}: expected {expected_rules}, got {rules}")
            failed += 1
        else:
            print(f"ok   {name}")
    total = len(SELFTEST_CASES) + len(BACKLINK_CASES)
    print(f"selftest: {total - failed}/{total} passed")
    return 1 if failed else 0


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--json", action="store_true", help="emit findings as JSON")
    parser.add_argument("--selftest", action="store_true", help="run embedded selftest")
    args = parser.parse_args()

    if args.selftest:
        return selftest()
    return run_checks(args.json)


if __name__ == "__main__":
    sys.exit(main())
