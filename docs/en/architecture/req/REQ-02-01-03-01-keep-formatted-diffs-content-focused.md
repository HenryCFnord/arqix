---
id: REQ-02-01-03-01
title: Keep Formatted Diffs Content-Focused
slug: keep-formatted-diffs-content-focused
iri: arqix:requirements/req-02-01-03-01

rdf:
  type:
    - arqix:classes/quality-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-02-01-03
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: A content-only edit followed by formatting produces a diff without unrelated style churn.

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

The formatting output SHOULD keep document diffs focused on content rather than incidental style changes.

### Notes

Derived from the acceptance criteria of US-02-01-03 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10).
Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
