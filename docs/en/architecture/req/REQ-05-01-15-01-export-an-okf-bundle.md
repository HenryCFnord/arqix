---
id: REQ-05-01-15-01
title: Export an OKF Bundle
slug: export-an-okf-bundle
iri: arqix:requirements/req-05-01-15-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-05-01-15
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: One command exports the corpus as a directory of OKF concept documents with includes expanded and directives stripped.

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

The arqix CLI SHALL export the corpus as an Open Knowledge Format bundle of artefact-ready Markdown concept documents.

### Notes

Derived from US-05-01-15.
