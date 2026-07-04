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
  updated: 2026-07-04
  lang: en
  translation-of:
  generated: false
---

## Risks and Technical Debt

TODO — to be maintained as implementation starts.

Known at specification time:

- Python/Rust checker drift: the oracle policy is decided (chapter 8 — scripts stay the oracle until the Rust port passes conformance, then cross-check, then removal). Remaining debt: the conformance suite itself must be built when the port starts.
- Performance budgets are unvalidated guesses until a real 1000-document corpus exists; decide when the corpus reaches ~500 documents or the first performance measurement exists.
- Mermaid views are hand-derived from `workspace.dsl` until the structurizr-cli export runs in CI (ADR-0002); implement with the first CI workflow PR.
- `doc search` has no decided implementation strategy yet (linear scan vs index); the one-second budget (REQ-00-00-00-11) may force an index, which brings state and invalidation questions. Decide with the doc search implementation story, after measuring a linear scan on the real corpus.
