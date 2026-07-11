---
id: REQ-04-01-05-01
title: Run Configured Verification Sub-Steps
slug: run-configured-verification-sub-steps
iri: arqix:requirements/req-04-01-05-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-04-01-05
      - arqix:user-stories/us-08-01-13
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Each configured sub-step runs; unconfigured sub-steps are skipped.

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

When `arqix verify` runs, arqix SHALL execute the configured sub-steps of format, lint, trace scan, and coverage.

### Notes

Derived from the acceptance criteria of US-04-01-05, US-08-01-13 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10).
Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
