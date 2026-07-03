---
id: REQ-02-01-10-01
title: Create Translation Files at Layout Location
slug: create-translation-files-at-layout-location
iri: arqix:requirements/req-02-01-10-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-02-01-10
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: The new translation file appears at the layout-defined path for the chosen i18n layout.

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

When a translation is scaffolded, arqix SHALL create the translation file at the location the chosen i18n layout defines.

### Notes

Derived from the acceptance criteria of US-02-01-10 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10). Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
