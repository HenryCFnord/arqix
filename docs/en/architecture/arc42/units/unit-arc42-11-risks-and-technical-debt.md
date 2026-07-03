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
  updated: 2026-07-03
  lang: en
  translation-of:
  generated: false
---

## Risks and Technical Debt

TODO — to be maintained as implementation starts.

Known at specification time:

- The Python checker scripts and the future Rust implementation can drift; the scripts are the oracle until ported, then must be retired or demoted to cross-checks.
- Performance budgets are unvalidated guesses until a real 1000-document corpus exists.
- Mermaid views are hand-derived from `workspace.dsl` until the structurizr-cli export runs in CI (ADR-0002).
