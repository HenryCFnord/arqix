---
id: REQ-01-01-08-02
title: Identify Requirements without Implementing Code
slug: identify-requirements-without-implementing-code
iri: arqix:requirements/req-01-01-08-02

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-08
      - arqix:user-stories/us-03-01-03
      - arqix:user-stories/us-07-01-01
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: A requirement with no code marker linking to it appears in the coverage report.

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

When `arqix trace coverage` runs, arqix SHALL identify requirements without `implements` code.

### Notes

Derived from the acceptance criteria of US-01-01-08 during the group-01 pilot derivation (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`).
Cross-cutting behaviour is linked via the story's `has-requirement`, not restated here.
