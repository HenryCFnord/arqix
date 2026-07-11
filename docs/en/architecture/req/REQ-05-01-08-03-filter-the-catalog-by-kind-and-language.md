---
id: REQ-05-01-08-03
title: Filter the Catalog by Kind and Language
slug: filter-the-catalog-by-kind-and-language
iri: arqix:requirements/req-05-01-08-03

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-05-01-08
      - arqix:user-stories/us-08-01-07
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Each filter narrows the catalog to matching documents only.

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

When `arqix doc list` is invoked with kind or language filters, arqix SHALL filter the catalog accordingly.

### Notes

Derived from the acceptance criteria of US-05-01-08, US-08-01-07 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10).
Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
