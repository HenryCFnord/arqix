---
id: REQ-04-01-12-03
title: Record Report Generation Metadata
slug: record-report-generation-metadata
iri: arqix:requirements/req-04-01-12-03

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-04-01-12
      - arqix:user-stories/us-07-01-07
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Every export carries all three metadata items.

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

Report metadata SHALL record the generation time, scope, and source inputs.

### Notes

Derived from the acceptance criteria of US-04-01-12, US-07-01-07 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10).
Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
