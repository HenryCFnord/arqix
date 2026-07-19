---
id: REQ-08-01-40-07
title: Resolve Record References to Claim Documents
slug: resolve-record-references-to-claim-documents
iri: arqix:requirements/req-08-01-40-07

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-08-01-40
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: A marker with record= naming an existing claim document passes; a record value that resolves to no document, or to a document outside the claim class, is CLM-004 naming the value; several markers may share one record.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-19
  updated: 2026-07-19
  lang: en
  translation-of:
  generated: false
---

## Requirement

When a claim marker carries a `record=` reference, arqix SHALL report the marker unless the reference resolves to a document of the claim class.

### Notes

Rule CLM-004; the record is the fullest provenance carrier (ADR-0019 carrier three), shared by any number of markers.
Derived from US-08-01-40.
