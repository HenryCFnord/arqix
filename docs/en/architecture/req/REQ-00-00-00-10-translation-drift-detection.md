---
id: REQ-00-00-00-10
title: Translation Drift Detection
slug: translation-drift-detection
iri: arqix:requirements/req-00-00-00-10

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-14
      - arqix:user-stories/us-04-01-04
      - arqix:user-stories/us-05-01-05
      - arqix:user-stories/us-08-01-11
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: The i18n lint profile flags missing required translations, unresolvable `translation_of` references, and translations whose `source_updated` is older than the source `updated`, and can gate CI with exit code 1.

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

When translations exist for a document, arqix SHALL detect missing and outdated translations deterministically.

### Notes

Curated to the i18n lint profile stories.
Translation scaffolding, language filtering, and i18n publishing configuration were excluded as story-bound concerns.

Contributing stories: 4 (see `derived-from`).
Approved via `docs/en/plans/requirements-derivation-2026-07-02/CROSS-CONCERNS.md`.
