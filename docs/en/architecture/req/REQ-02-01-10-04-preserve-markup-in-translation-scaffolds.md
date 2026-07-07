---
id: REQ-02-01-10-04
title: Preserve Markup in Translation Scaffolds
slug: preserve-markup-in-translation-scaffolds
iri: arqix:requirements/req-02-01-10-04

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-02-01-10
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Directives and structural elements of the source survive scaffolding as the strategy defines.

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

When a translation is scaffolded, arqix SHALL preserve arqix markup directives and structural elements according to the scaffold strategy.

### Notes

Derived from the acceptance criteria of US-02-01-10 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10).
Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
