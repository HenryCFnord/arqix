---
id: REQ-03-01-06-02
title: Report Verifies Markers per Requirement
slug: report-verifies-markers-per-requirement
iri: arqix:requirements/req-03-01-06-02

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-03-01-06
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: The command answers the verifies question for the given requirement ID.

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

When `arqix trace check` is invoked for a requirement, arqix SHALL report whether `verifies` markers exist for it.

### Notes

Derived from the acceptance criteria of US-03-01-06 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10).
Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
