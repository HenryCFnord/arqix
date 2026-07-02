---
id: REQ-04-01-13-03
title: Report Broken Architecture Navigation
slug: report-broken-architecture-navigation
iri: arqix:requirements/req-04-01-13-03

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-04-01-13
      - arqix:user-stories/us-06-01-11
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: A broken navigation path produces a validation finding naming the path.

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

When publish validation runs, arqix SHALL report broken architecture navigation paths.

### Notes

Derived from the acceptance criteria of US-04-01-13, US-06-01-11 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10). Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
