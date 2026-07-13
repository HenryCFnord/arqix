---
id: REQ-01-01-11-04
title: C4-Oriented Architecture Views
slug: c4-oriented-architecture-views
iri: arqix:requirements/req-01-01-11-04

rdf:
  type:
    - arqix:classes/quality-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-11
      - arqix:user-stories/us-06-01-07
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: The architecture views present the C4 model at its levels (context, containers) and are generated from workspace.dsl, not hand-authored.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-02
  updated: 2026-07-13
  lang: en
  translation-of:
  generated: false
---

## Requirement

Architecture views SHOULD present the C4 model in a C4-oriented modelling style.

### Notes

Derived from the acceptance criteria of US-01-01-11 during the group-01 pilot derivation (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`).
The views were originally hand-authored C4 Mermaid; they are now rendered from `workspace.dsl` as embedded images (ADR-0016, US-04-01-18), which keeps the C4-oriented style while removing the drift the hand-authored diagrams invited.
Cross-cutting behaviour is linked via the story's `has-requirement`, not restated here.
