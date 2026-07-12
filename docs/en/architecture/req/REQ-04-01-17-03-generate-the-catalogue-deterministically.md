---
id: REQ-04-01-17-03
title: Generate the Catalogue Deterministically
slug: generate-the-catalogue-deterministically
iri: arqix:requirements/req-04-01-17-03

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-04-01-17
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: Two stagings of the same corpus produce byte-identical catalogue pages.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-12
  updated: 2026-07-12
  lang: en
  translation-of:
  generated: false
---

## Requirement

When the corpus is unchanged, repeated staging SHALL produce byte-identical catalogue pages.

### Notes

Derived from US-04-01-17.
Groups, stories, and requirements are ordered by ID; no wall-clock values enter the pages (the injected-clock discipline).
