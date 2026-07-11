---
id: REQ-08-01-02-01
title: Make Assembly Outcomes Machine-Interpretable
slug: make-assembly-outcomes-machine-interpretable
iri: arqix:requirements/req-08-01-02-01

rdf:
  type:
    - arqix:classes/quality-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-08-01-02
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Success and failure are decidable from the log and exit status alone.

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

Assembly outcomes SHOULD be interpretable from the log and command result without human guesswork.

### Notes

Derived from the acceptance criteria of US-08-01-02 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10).
Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
