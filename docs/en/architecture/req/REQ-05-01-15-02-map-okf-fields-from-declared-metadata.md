---
id: REQ-05-01-15-02
title: Map OKF Fields from Declared Metadata
slug: map-okf-fields-from-declared-metadata
iri: arqix:requirements/req-05-01-15-02

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
  fit-criterion: An exported concept document carries type from the declared class, title verbatim, and timestamp from the declared update date where present; absent metadata is omitted, never fabricated.

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

When exporting an OKF bundle, arqix SHALL map each concept document's OKF fields from the source document's declared metadata, omitting fields whose source metadata is absent.

### Notes

Derived from US-05-01-15.
OKF v0.1 requires only `type`; a document without a declared class exports as the generic document type.
