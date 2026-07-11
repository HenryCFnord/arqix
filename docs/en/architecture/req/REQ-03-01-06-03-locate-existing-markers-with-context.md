---
id: REQ-03-01-06-03
title: Locate Existing Markers with Context
slug: locate-existing-markers-with-context
iri: arqix:requirements/req-03-01-06-03

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
  fit-criterion: Every reported marker carries its file path and line number.

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

The trace check report SHALL include the locations of existing markers with path and line context.

### Notes

Derived from the acceptance criteria of US-03-01-06 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10).
Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
