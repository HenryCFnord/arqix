#!/usr/bin/env python3
"""Generate question-driven report units from the trace graph (ADR-0008).

Every unit answers exactly one named question from the living catalog in
docs/en/reports/QUESTIONS.md; the presentation follows the question, not
the data structure. Units are deterministic projections of the model built
by arqix_trace; the snapshot stamp (commit + date) is injected via
--snapshot, never taken from the wall clock.

Usage:
  python3 scripts/arqix_report.py --snapshot "<sha>, <date>" \
      [--out docs/en/reports/units]
  python3 scripts/arqix_report.py --selftest
"""

import argparse
import sys
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(SCRIPT_DIR))
from arqix_trace import build_model, coverage, read_corpus  # noqa: E402


def header(question, qid, snapshot):
    return (
        f"<!-- GENERATED SNAPSHOT — do not edit by hand.\n"
        f"     Question: {qid} (see docs/en/reports/QUESTIONS.md)\n"
        f"     Snapshot: {snapshot}\n"
        f"     Regenerate: python3 scripts/arqix_report.py "
        f'--snapshot "<sha>, <date>" -->\n'
        f"\n# {question}\n"
    )


def title_of(documents, doc_id):
    info = documents.get(doc_id)
    return (info.get("title") or doc_id) if info else doc_id


def marker_rows(edges, kind):
    """Sorted (test, location, requirement, ignored) rows for marker edges."""
    rows = []
    for e in edges:
        if e["kind"] == kind and e["test"] is not None:
            rows.append((e["test"], f"{e['from']}:{e['line']}", e["to"], e["ignored"]))
    return sorted(rows)


def stories_of_requirement(edges, req_id):
    return sorted(
        e["to"] for e in edges
        if e["from"] == req_id and e["kind"] == "derived-from"
        and str(e["to"]).startswith("US-")
    )


def workflows_of_story(edges, story_id):
    return sorted(
        e["to"] for e in edges
        if e["from"] == story_id and e["kind"] == "is-part-of-workflow"
    )


def unit_story_progress(requirements, edges, documents, snapshot):
    """Q-01: How far along is each user story?"""
    report, _ = coverage(requirements, edges)
    per_story = {}
    for row in report["requirements"]:
        for story in stories_of_requirement(edges, row["id"]) or [row["story"]]:
            if story is None:
                continue
            bucket = per_story.setdefault(
                story, {"verified": 0, "planned": 0, "uncovered": 0}
            )
            if row["verified_by"]:
                bucket["verified"] += 1
            elif row["planned_by"]:
                bucket["planned"] += 1
            else:
                bucket["uncovered"] += 1

    lines = [header("How far along is each user story?", "Q-01", snapshot)]
    lines.append("A requirement counts for every story that demands it "
                 "(`derived-from`), so shared requirements advance several "
                 "stories at once.\n")
    lines.append("| story | title | verified | planned | uncovered | progress |")
    lines.append("| --- | --- | ---: | ---: | ---: | --- |")
    for story in sorted(per_story):
        b = per_story[story]
        total = b["verified"] + b["planned"] + b["uncovered"]
        done = round(100 * b["verified"] / total) if total else 0
        bar = "█" * (done // 10) + "░" * (10 - done // 10)
        lines.append(
            f"| {story} | {title_of(documents, story)} | {b['verified']} "
            f"| {b['planned']} | {b['uncovered']} | `{bar}` {done}% |"
        )
    return "\n".join(lines) + "\n"


def unit_scoreboard(requirements, edges, documents, snapshot):
    """Q-03: What share of requirements is verifiably implemented?"""
    report, _ = coverage(requirements, edges)
    lines = [header("What share of the requirements is verifiably implemented?",
                    "Q-03", snapshot)]
    lines.append("| kind | verified | planned | uncovered | total | verified % |")
    lines.append("| --- | ---: | ---: | ---: | ---: | ---: |")
    for kind in ("functional", "quality", "constraint"):
        s = report["summary"].get(
            kind, {"verified": 0, "planned": 0, "uncovered": 0, "total": 0}
        )
        pct = round(100 * s["verified"] / s["total"]) if s["total"] else 0
        lines.append(
            f"| {kind} | {s['verified']} | {s['planned']} | {s['uncovered']} "
            f"| {s['total']} | {pct}% |"
        )
    lines.append("")
    lines.append("*Verified* means a `verifies` marker on an active test; "
                 "markers on `#[ignore]`d tests only *plan* verification "
                 "(red-skeleton lifecycle, ADR-0006).")
    return "\n".join(lines) + "\n"


def unit_test_to_requirement(requirements, edges, documents, snapshot):
    """Q-02: Which tests verify which requirements?"""
    lines = [header("Which tests verify which requirements?", "Q-02", snapshot)]
    lines.append("| test | location | requirement | status |")
    lines.append("| --- | --- | --- | --- |")
    for test, loc, req, ignored in marker_rows(edges, "verifies"):
        status = "planned (ignored)" if ignored else "active"
        lines.append(f"| `{test}` | {loc} | {req} | {status} |")
    return "\n".join(lines) + "\n"


def unit_test_to_story(requirements, edges, documents, snapshot):
    """Q-05: Which user story belongs to which integration test?"""
    pairs = set()
    for test, loc, req, _ in marker_rows(edges, "verifies"):
        for story in stories_of_requirement(edges, req):
            pairs.add((test, story))
    lines = [header("Which user story belongs to which integration test?",
                    "Q-05", snapshot)]
    lines.append("Joined test → requirement (`verifies`) → story "
                 "(`derived-from`).\n")
    lines.append("| test | story | title |")
    lines.append("| --- | --- | --- |")
    for test, story in sorted(pairs):
        lines.append(f"| `{test}` | {story} | {title_of(documents, story)} |")
    return "\n".join(lines) + "\n"


def unit_test_to_workflow(requirements, edges, documents, snapshot):
    """Q-06: Which workflow belongs to which integration test?"""
    pairs = set()
    for test, loc, req, _ in marker_rows(edges, "verifies"):
        for story in stories_of_requirement(edges, req):
            for wf in workflows_of_story(edges, story):
                pairs.add((test, wf))
    lines = [header("Which workflow belongs to which integration test?",
                    "Q-06", snapshot)]
    lines.append("Joined test → requirement → story → workflow "
                 "(`is-part-of-workflow`).\n")
    lines.append("| test | workflow | title |")
    lines.append("| --- | --- | --- |")
    for test, wf in sorted(pairs):
        lines.append(f"| `{test}` | {wf} | {title_of(documents, wf)} |")
    return "\n".join(lines) + "\n"


def unit_adr_to_requirement(requirements, edges, documents, snapshot):
    """Q-07: Which ADRs are linked to which requirements?"""
    lines = [header("Which ADRs are linked to which requirements?",
                    "Q-07", snapshot)]
    lines.append("| adr | title | requirement |")
    lines.append("| --- | --- | --- |")
    for e in sorted(
        (e for e in edges if e["kind"] == "guides-implementation-of"),
        key=lambda e: (e["from"], str(e["to"])),
    ):
        lines.append(f"| {e['from']} | {title_of(documents, e['from'])} | {e['to']} |")
    return "\n".join(lines) + "\n"


def unit_code_to_requirement(requirements, edges, documents, snapshot):
    """Q-04: Which code implements which requirement?"""
    rows = marker_rows(edges, "implements")
    lines = [header("Which code implements which requirement?", "Q-04", snapshot)]
    if not rows:
        lines.append("No `implements` markers exist yet — the Rust "
                     "implementation phase has not started. This unit fills "
                     "up as stories move from red to green.")
    else:
        lines.append("| code | location | requirement |")
        lines.append("| --- | --- | --- |")
        for test, loc, req, _ in rows:
            lines.append(f"| `{test}` | {loc} | {req} |")
    return "\n".join(lines) + "\n"


def unit_doc_to_code(requirements, edges, documents, snapshot):
    """Q-08: Where is the documentation for a given piece of code?"""
    lines = [header("Where is the documentation for a given piece of code?",
                    "Q-08", snapshot)]
    lines.append(
        "Partially answerable today: code → requirement (`implements` "
        "markers, see Q-04) → story/unit via the ontology. A direct "
        "code → document convention (e.g. an `arqix:documented-by` marker "
        "or unit frontmatter listing source paths) is an open design "
        "decision; until it is made, this unit stays a placeholder naming "
        "that gap."
    )
    return "\n".join(lines) + "\n"


UNITS = [
    ("story-progress.md", unit_story_progress),
    ("scoreboard.md", unit_scoreboard),
    ("test-to-requirement.md", unit_test_to_requirement),
    ("test-to-story.md", unit_test_to_story),
    ("test-to-workflow.md", unit_test_to_workflow),
    ("adr-to-requirement.md", unit_adr_to_requirement),
    ("code-to-requirement.md", unit_code_to_requirement),
    ("doc-to-code.md", unit_doc_to_code),
]


def generate(out_dir, snapshot, root="."):
    requirements, edges, documents = build_model(read_corpus(root))
    out = Path(out_dir)
    out.mkdir(parents=True, exist_ok=True)
    for filename, unit in UNITS:
        (out / filename).write_text(
            unit(requirements, edges, documents, snapshot), encoding="utf-8"
        )
        print(f"wrote {out / filename}")
    return 0


SELFTEST_CORPUS = {
    "docs/r.md": """---
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
""",
    "docs/s.md": """---
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
""",
    "docs/w.md": """---
id: WF-22-22
title: Example Workflow
iri: arqix:workflows/wf-22-22
rdf:
  type:
    - arqix:classes/workflow
---
body
""",
    "a.rs": "// arqix:verifies REQ-11-11-11-01\n#[test]\nfn covers() {}\n",
    "b.rs": "// arqix:implements REQ-11-11-11-01\nconst SCHEMA: u32 = 1;\n",
}


def selftest():
    requirements, edges, documents = build_model(SELFTEST_CORPUS)
    snapshot = "test, 2026-01-01"
    failed = 0

    def expect(name, condition):
        nonlocal failed
        if condition:
            print(f"ok   {name}")
        else:
            print(f"FAIL {name}")
            failed += 1

    progress = unit_story_progress(requirements, edges, documents, snapshot)
    expect("story progress counts the verified requirement",
           "| US-22-22-22 | Provide a Linked Example | 1 | 0 | 0 |" in progress)

    board = unit_scoreboard(requirements, edges, documents, snapshot)
    expect("scoreboard shows 100% verified functional",
           "| functional | 1 | 0 | 0 | 1 | 100% |" in board)

    t2r = unit_test_to_requirement(requirements, edges, documents, snapshot)
    expect("test-to-requirement lists the active test",
           "| `covers` | a.rs:1 | REQ-11-11-11-01 | active |" in t2r)

    t2s = unit_test_to_story(requirements, edges, documents, snapshot)
    expect("test-to-story joins through the requirement",
           "| `covers` | US-22-22-22 | Provide a Linked Example |" in t2s)

    t2w = unit_test_to_workflow(requirements, edges, documents, snapshot)
    expect("test-to-workflow joins through the story",
           "| `covers` | WF-22-22 | Example Workflow |" in t2w)

    q4 = unit_code_to_requirement(requirements, edges, documents, snapshot)
    expect("code unit lists implements markers not attached to a fn",
           "| — | b.rs:1 | REQ-11-11-11-01 |" in q4)

    again = unit_story_progress(requirements, edges, documents, snapshot)
    expect("units are deterministic", progress == again)

    total = 7
    print(f"selftest: {total - failed}/{total} passed")
    return 1 if failed else 0


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--snapshot", help='snapshot stamp, e.g. "abc1234, 2026-07-05"')
    parser.add_argument("--out", default="docs/en/reports/units",
                        help="output directory for the unit files")
    parser.add_argument("--selftest", action="store_true", help="run embedded selftest")
    args = parser.parse_args()

    if args.selftest:
        return selftest()
    if not args.snapshot:
        print("error: --snapshot is required (injected, never ambient)", file=sys.stderr)
        return 2
    return generate(args.out, args.snapshot)


if __name__ == "__main__":
    sys.exit(main())
