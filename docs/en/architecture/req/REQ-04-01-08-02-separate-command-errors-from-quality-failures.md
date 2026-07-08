---
id: REQ-04-01-08-02
title: Separate Command Errors from Quality Failures
slug: separate-command-errors-from-quality-failures
iri: arqix:requirements/req-04-01-08-02

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-04-01-08
      - arqix:user-stories/us-08-01-15
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: A CI script can classify a run as command error or quality failure from stderr alone.

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

The stderr messaging SHALL let CI distinguish command errors from quality failures.

### Notes

Derived from the acceptance criteria of US-04-01-08, US-08-01-15 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10).
Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
