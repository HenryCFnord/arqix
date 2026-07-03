---
id: REQ-04-01-05-02
title: Support Fail-Fast and Aggregate Modes
slug: support-fail-fast-and-aggregate-modes
iri: arqix:requirements/req-04-01-05-02

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-04-01-05
      - arqix:user-stories/us-08-01-13
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Fail-fast stops at the first failing sub-step; aggregate mode reports all sub-step results.

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

The verification loop SHALL support fail-fast and aggregate result modes, selected by configuration.

### Notes

Derived from the acceptance criteria of US-04-01-05, US-08-01-13 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10). Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
