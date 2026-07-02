---
id: REQ-04-01-08-01
title: Pin the Exit-Code Contract
slug: pin-the-exit-code-contract
iri: arqix:requirements/req-04-01-08-01

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
  fit-criterion: Each of the three outcome classes maps to its documented code and nothing else.

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

The arqix CLI SHALL use exit code `0` for success, `1` for lint or quality-gate failure, and `2` for usage error.

### Notes

Derived from the acceptance criteria of US-04-01-08, US-08-01-15 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10). Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
