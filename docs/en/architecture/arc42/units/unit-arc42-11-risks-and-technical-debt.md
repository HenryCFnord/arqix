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
- Performance budgets have their first real measurements: at ~480 corpus documents (roughly half the 1000-document reference corpus), `arqix verify` completes in ~5.5s warm against the ten-second budget (REQ-00-00-00-12; the git-history marker-freshness sub-step dominates at ~2.7s) and `doc search` in ~0.15s against the one-second budget (REQ-00-00-00-11).
  Revisit when the corpus approaches 1000 documents or a budget is exceeded; the freshness sub-step is the first optimization candidate.
- The C4 views are rendered from `workspace.dsl` via the digest-pinned Kroki image (ADR-0016) and committed under `model/generated/`.
  Remaining debt: the freshness gate (`just render-views-check`, the architecture-diagrams workflow) is manual-dispatch only and not yet part of the blocking CI gate; promoting it once the Kroki invocation is confirmed on a runner is open.
- `doc search` is a linear full-text scan in v1 (`src/store.rs`); no index yet.
  The one-second budget (REQ-00-00-00-11) may later force an index, which brings state and invalidation questions.
  Decide with a dedicated search story, after measuring the linear scan on the real corpus.
- The assembly log path is a fixed v1 default (`pages/assembly.jsonl`); REQ-04-01-01-03 asks for it to be configurable, which lands with the render/publish configuration story in Phase 5.
  A missing include target is currently left verbatim by the assembler (the linter's LNT-001 is the tool that flags it); folding that signal into `assemble build` is a later refinement.
- A future documentation consistency check (drift between the C4 model, the Mermaid views, and the prose) is recorded as an extension path of the verification pipeline (REQ-01-01-11-05); no story schedules it yet.
