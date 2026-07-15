---
id: unit-arc42-11
title: Risks and Technical Debt
slug: risks-and-technical-debt
iri: arqix:units/unit-arc42-11

rdf:
  type:
    - arqix:classes/unit

triples: []

properties:
  section-kind: arc42-chapter

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-03
  updated: 2026-07-15
  lang: en
  translation-of:
  generated: false
---

## Risks and Technical Debt

Maintained as implementation proceeds.
The Rust core landed in Phase 4 (parser, store, linter, rewriter, trace engine, verify, templates, assembler); the items below track what that left open.

Known at specification time:

- Python/Rust checker drift: **closed** (chapter 8 — the oracle policy ran its course: oracle until conformance, then retirement).
  Every family passed conformance — trace (`cli_trace` green with the Rust binary, value-equal output on the real corpus), the marker gate, the report units, and the frontmatter/requirements checkers (JSON value-equal) — and the owner retired all five Python scripts directly on 2026-07-15 (task #78), closing the grace period.
  The drift risk is gone with the second implementation; what guards the contracts now is the Rust test suite, which mirrors the retired oracles' selftest fixtures (`selftest_cases_match_the_oracle` and siblings), so a regression against the pinned behaviours fails the gate.
- Performance budgets are unvalidated guesses until a real 1000-document corpus exists; decide when the corpus reaches ~500 documents or the first performance measurement exists.
- Mermaid views are hand-derived from `workspace.dsl` (ADR-0002).
  The first CI PR landed the gate without the structurizr-cli export: switching the views from hand-derived to generated is a diagram-pipeline change of its own, and joins CI as a separate slice.
- `doc search` is a linear full-text scan in v1 (`src/store.rs`); no index yet.
  The one-second budget (REQ-00-00-00-11) may later force an index, which brings state and invalidation questions.
  Decide with a dedicated search story, after measuring the linear scan on the real corpus.
- The assembly log path is a fixed v1 default (`pages/assembly.jsonl`); REQ-04-01-01-03 asks for it to be configurable, which lands with the render/publish configuration story in Phase 5.
  A missing include target is currently left verbatim by the assembler (the linter's LNT-001 is the tool that flags it); folding that signal into `assemble build` is a later refinement.
- A future documentation consistency check (drift between the C4 model, the Mermaid views, and the prose) is recorded as an extension path of the verification pipeline (REQ-01-01-11-05); no story schedules it yet.
