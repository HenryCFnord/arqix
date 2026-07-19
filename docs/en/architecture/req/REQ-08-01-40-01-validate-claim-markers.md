---
id: REQ-08-01-40-01
title: Validate Claim Markers
slug: validate-claim-markers
iri: arqix:requirements/req-08-01-40-01

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
  priority: high
  fit-criterion: A claim marker without supported-by or with an unknown key is CLM-001 naming the defect; confidence outside the effective vocabulary is CLM-002; a well-formed marker with a default-vocabulary confidence passes.

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

When `arqix lint frontmatter` checks a document body, arqix SHALL report every claim marker that misses `supported-by` or carries an unknown key, and every `confidence` value outside the effective vocabulary.

### Notes

Rules CLM-001 and CLM-002; the effective vocabulary is `[frontmatter].claim-confidence`, defaulting to high, inferred, estimated.
Derived from US-08-01-40.
