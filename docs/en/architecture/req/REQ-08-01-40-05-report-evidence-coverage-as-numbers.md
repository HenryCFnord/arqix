---
id: REQ-08-01-40-05
title: Report Evidence Coverage as Numbers
slug: report-evidence-coverage-as-numbers
iri: arqix:requirements/req-08-01-40-05

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
  fit-criterion: report snapshot renders evidence-coverage.md with the claim total, the count of documents carrying claims, and the distinct sources cited; zero everywhere on a markerless corpus.

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

When `arqix report snapshot` runs, arqix SHALL render the evidence-coverage unit with the claim total, the documents carrying claims, and the distinct sources cited.

### Notes

Question Q-12; coverage stays a report number, never a gate (ADR-0018).
Derived from US-08-01-40.
