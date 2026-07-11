---
id: REQ-04-01-12-01
title: Support Audit Export Formats
slug: support-audit-export-formats
iri: arqix:requirements/req-04-01-12-01

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
  fit-criterion: Each applicable report is exportable in all three formats.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-02
  updated: 2026-07-10
  lang: en
  translation-of:
  generated: false
---

## Requirement

Audit-oriented report exports SHALL support at least Markdown, CSV, and JSON where applicable.

### Notes

Derived from the acceptance criteria of US-04-01-12, US-07-01-07 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10).
Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
