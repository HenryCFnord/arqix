---
id: REQ-01-01-04-05
title: Fail on Invalid Lint Input
slug: fail-on-invalid-lint-input
iri: arqix:requirements/req-01-01-04-05

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-04
      - arqix:user-stories/us-02-01-04
      - arqix:user-stories/us-03-01-01
      - arqix:user-stories/us-08-01-04
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Invalid input yields a non-success exit status instead of a silent pass.

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

If lint input is invalid, then arqix SHALL return a failing status.

### Notes

Derived from the acceptance criteria of US-01-01-04 during the group-01 pilot derivation (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`).
Cross-cutting behaviour is linked via the story's `has-requirement`, not restated here.
