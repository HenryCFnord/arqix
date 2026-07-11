---
id: REQ-02-01-09-03
title: Expand Glob Includes with Configured Sorting
slug: expand-glob-includes-with-configured-sorting
iri: arqix:requirements/req-02-01-09-03

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-02-01-09
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: The expansion order matches the configured sorting for the same file set.

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

When a glob include is expanded, arqix SHALL apply the configured sorting.

### Notes

Derived from the acceptance criteria of US-02-01-09 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10).
Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
