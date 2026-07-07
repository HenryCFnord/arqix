---
id: REQ-03-01-02-04
title: Keep Empty-Link Cases Reviewer-Friendly
slug: keep-empty-link-cases-reviewer-friendly
iri: arqix:requirements/req-03-01-02-04

rdf:
  type:
    - arqix:classes/quality-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-03-01-02
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: A requirement without links still appears as a row that a reviewer can spot without post-processing.

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

Empty-link cases SHOULD remain visible in the exported matrix in a reviewer-friendly form.

### Notes

Derived from the acceptance criteria of US-03-01-02 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10).
Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
