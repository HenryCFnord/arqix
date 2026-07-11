---
id: REQ-05-01-10-01
title: Retrieve Sections by Slug or Anchor
slug: retrieve-sections-by-slug-or-anchor
iri: arqix:requirements/req-05-01-10-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-05-01-10
      - arqix:user-stories/us-08-01-09
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: A slug or anchor selector yields exactly the selected section.

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

When `arqix doc read` is invoked with a heading slug or explicit anchor, arqix SHALL return the selected section.

### Notes

Derived from the acceptance criteria of US-05-01-10, US-08-01-09 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10).
Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
