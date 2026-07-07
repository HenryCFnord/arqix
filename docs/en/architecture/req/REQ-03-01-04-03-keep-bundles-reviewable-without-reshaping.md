---
id: REQ-03-01-04-03
title: Keep Bundles Reviewable without Reshaping
slug: keep-bundles-reviewable-without-reshaping
iri: arqix:requirements/req-03-01-04-03

rdf:
  type:
    - arqix:classes/quality-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-03-01-04
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: A reviewer can work through the bundle as exported, without converting or reorganising files first.

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

The exported bundle SHOULD be reviewable without manual reshaping of the source evidence.

### Notes

Derived from the acceptance criteria of US-03-01-04 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10).
Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
