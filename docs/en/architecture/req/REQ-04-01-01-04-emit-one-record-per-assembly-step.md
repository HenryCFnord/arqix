---
id: REQ-04-01-01-04
title: Emit One Record per Assembly Step
slug: emit-one-record-per-assembly-step
iri: arqix:requirements/req-04-01-01-04

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-04-01-01
      - arqix:user-stories/us-05-01-02
      - arqix:user-stories/us-06-01-02
      - arqix:user-stories/us-08-01-02
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: The number of records equals the number of assembly steps, with a stable record shape.

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

When an assembly step executes, arqix SHALL emit exactly one stable JSONL record for it.

### Notes

Derived from the acceptance criteria of US-04-01-01, US-05-01-02, US-06-01-02, US-08-01-02 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10).
Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
