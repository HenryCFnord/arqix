---
id: REQ-03-01-02-01
title: Export Trace Matrices as CSV
slug: export-trace-matrices-as-csv
iri: arqix:requirements/req-03-01-02-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-03-01-02
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: A matrix export produces a CSV file for the selected matrix type.

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

When `arqix trace matrix` runs, arqix SHALL export the selected matrix as CSV.

### Notes

Derived from the acceptance criteria of US-03-01-02 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10).
Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
