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
  updated: 2026-07-06
  lang: en
  translation-of:
  generated: false
---

## Risks and Technical Debt

Maintained as implementation proceeds.
The Rust core landed in Phase 4 (parser, store, linter, rewriter, trace engine, verify, templates, assembler); the items below track what that left open.

Known at specification time:

- Python/Rust checker drift: the oracle policy is decided (chapter 8 — scripts stay the oracle until the Rust port passes conformance, then cross-check, then removal).
  The **trace** family has now passed conformance (the `cli_trace` suite is green with the Rust binary and its output is value-equal to the oracle on the real corpus), so the trace oracle is in its cross-check phase.
  Remaining debt: `check_requirements.py`/`check_frontmatter.py` are still the active oracle; their Rust ports and conformance suites are Phase 5 work.
- Performance budgets are unvalidated guesses until a real 1000-document corpus exists; decide when the corpus reaches ~500 documents or the first performance measurement exists.
- Mermaid views are hand-derived from `workspace.dsl` until the structurizr-cli export runs in CI (ADR-0002); implement with the first CI workflow PR.
- `doc search` is a linear full-text scan in v1 (`src/store.rs`); no index yet.
  The one-second budget (REQ-00-00-00-11) may later force an index, which brings state and invalidation questions.
  Decide with a dedicated search story, after measuring the linear scan on the real corpus.
- The assembler fingerprints fragments with a self-contained SHA-256 (`src/sha256.rs`), chosen to keep the dependency graph minimal and the output deterministic; it is a content-identity marker, not a security primitive, and is pinned to the NIST test vectors.
  Debt: if a vetted hash dependency is later adopted for other reasons, this module can be retired in its favour.
- The assembly log path is a fixed v1 default (`pages/assembly.jsonl`); REQ-04-01-01-03 asks for it to be configurable, which lands with the render/publish configuration story in Phase 5.
  A missing include target is currently left verbatim by the assembler (the linter's LNT-001 is the tool that flags it); folding that signal into `assemble build` is a later refinement.
