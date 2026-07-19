---
id: REQ-08-01-40-04
title: Export Claims as Data
slug: export-claims-as-data
iri: arqix:requirements/req-08-01-40-04

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-08-01-40
      - arqix:user-stories/us-08-01-41
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: report claims prints file, supported-by, confidence, and anchor per marker in deterministic order; the committed claims.csv is freshness-checked by report snapshot --check; a markerless corpus prints the header only.

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

When `arqix report claims` runs, arqix SHALL print one deterministic CSV row per claim marker — file, supported-by target, confidence, and anchor — with the committed export held fresh by the snapshot check.

### Notes

The export mirrors the normative-statements projection; empty stays the header-only file.
Derived from US-08-01-40.
