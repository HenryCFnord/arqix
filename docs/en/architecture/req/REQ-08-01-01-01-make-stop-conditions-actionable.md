---
id: REQ-08-01-01-01
title: Make Stop Conditions Actionable
slug: make-stop-conditions-actionable
iri: arqix:requirements/req-08-01-01-01

rdf:
  type:
    - arqix:classes/quality-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-08-01-01
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: An agent or user can resolve the failure from the diagnostic alone.

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

Failure diagnostics SHOULD make the stop condition clear enough to act on without reading source code.

### Notes

Derived from the acceptance criteria of US-08-01-01 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10).
Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
