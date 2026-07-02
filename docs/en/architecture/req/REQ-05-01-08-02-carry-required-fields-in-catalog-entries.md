---
id: REQ-05-01-08-02
title: Carry Required Fields in Catalog Entries
slug: carry-required-fields-in-catalog-entries
iri: arqix:requirements/req-05-01-08-02

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
  fit-criterion: Every entry carries all five fields.

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

Each catalog entry SHALL include at least `id`, `kind`, `title`, `lang`, and the source path.

### Notes

Derived from the acceptance criteria of US-05-01-08, US-08-01-07 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10). Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
