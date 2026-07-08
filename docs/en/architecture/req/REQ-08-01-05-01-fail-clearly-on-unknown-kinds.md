---
id: REQ-08-01-05-01
title: Fail Clearly on Unknown Kinds
slug: fail-clearly-on-unknown-kinds
iri: arqix:requirements/req-08-01-05-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-08-01-05
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Requesting an unconfigured kind yields a failing status and an error naming the kind.

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

If an unknown document kind is requested, then arqix SHALL fail with a clear, actionable error.

### Notes

Derived from the acceptance criteria of US-08-01-05 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10).
Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
