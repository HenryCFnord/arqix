---
id: REQ-01-01-04-04
title: Report Lint Findings with File and Line Context
slug: report-lint-findings-with-file-and-line-context
iri: arqix:requirements/req-01-01-04-04

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-04
      - arqix:user-stories/us-02-01-04
      - arqix:user-stories/us-03-01-01
      - arqix:user-stories/us-08-01-04
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Every lint finding names the file and line it originates from.

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

When a lint check reports a finding, the diagnostic SHALL include precise file and line context.

### Notes

Derived from the acceptance criteria of US-01-01-04 during the group-01 pilot derivation (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`).
Cross-cutting behaviour is linked via the story's `has-requirement`, not restated here.
