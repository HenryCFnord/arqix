---
id: REQ-03-01-02-02
title: Support REQ-Test and US-REQ Matrices
slug: support-req-test-and-us-req-matrices
iri: arqix:requirements/req-03-01-02-02

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-03-01-02
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Both matrix types are selectable and populated from the trace graph.

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

The arqix CLI SHALL support at least the `REQ×Test` and `US×REQ` matrix types.

### Notes

Derived from the acceptance criteria of US-03-01-02 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10).
Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
