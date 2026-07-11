---
id: REQ-04-01-13-01
title: Assemble Architecture Units in Predictable Order
slug: assemble-architecture-units-in-predictable-order
iri: arqix:requirements/req-04-01-13-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-04-01-13
      - arqix:user-stories/us-06-01-11
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: The chapter order follows the declared structure and does not vary between runs.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-02
  updated: 2026-07-10
  lang: en
  translation-of:
  generated: false
---

## Requirement

When architecture source units are assembled, arqix SHALL produce a predictable chapter order.

### Notes

Derived from the acceptance criteria of US-04-01-13, US-06-01-11 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10).
Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
