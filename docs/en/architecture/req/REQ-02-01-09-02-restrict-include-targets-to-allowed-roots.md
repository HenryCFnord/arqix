---
id: REQ-02-01-09-02
title: Restrict Include Targets to Allowed Roots
slug: restrict-include-targets-to-allowed-roots
iri: arqix:requirements/req-02-01-09-02

rdf:
  type:
    - arqix:classes/constraint

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-02-01-09
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: An include pointing outside the allowed roots is rejected with a diagnostic.

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

The arqix CLI SHALL NOT resolve include targets outside the configured allowed roots.

### Notes

Derived from the acceptance criteria of US-02-01-09 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10).
Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
