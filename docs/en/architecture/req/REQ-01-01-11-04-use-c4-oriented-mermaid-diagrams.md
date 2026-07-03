---
id: REQ-01-01-11-04
title: Use C4-Oriented Mermaid Diagrams
slug: use-c4-oriented-mermaid-diagrams
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
  fit-criterion: Architecture diagrams are Mermaid-based and follow C4 levels where applicable.

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-02
  updated: 2026-07-02
  lang: en
  translation-of:
  generated: false
---

## Requirement

Architecture views SHOULD use Mermaid diagrams in a C4-oriented modelling style.

### Notes

Derived from the acceptance criteria of US-01-01-11 during the group-01 pilot derivation (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`). Cross-cutting behaviour is linked via the story's `has-requirement`, not restated here.
