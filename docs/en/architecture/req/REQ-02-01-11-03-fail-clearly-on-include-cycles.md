---
id: REQ-02-01-11-03
title: Fail Clearly on Include Cycles
slug: fail-clearly-on-include-cycles
iri: arqix:requirements/req-02-01-11-03

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-02-01-11
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: A cyclic include chain aborts assembly with an error naming the cycle.

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

If an include cycle is detected, then arqix SHALL fail with a clear error message.

### Notes

Derived from the acceptance criteria of US-02-01-11 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10).
Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
