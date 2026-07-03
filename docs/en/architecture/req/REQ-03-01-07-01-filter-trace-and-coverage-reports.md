---
id: REQ-03-01-07-01
title: Filter Trace and Coverage Reports
slug: filter-trace-and-coverage-reports
iri: arqix:requirements/req-03-01-07-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-03-01-07
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Each filter dimension (kind, status, missing-link category) narrows the report; filtered outputs remain exportable.

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

When a trace or coverage report is generated, arqix SHALL support filtering by document kind, status, and missing-link category.

### Notes

Derived from the acceptance criteria of US-03-01-07 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10). Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
