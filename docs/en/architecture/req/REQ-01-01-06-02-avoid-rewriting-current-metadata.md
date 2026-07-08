---
id: REQ-01-01-06-02
title: Avoid Rewriting Current Metadata
slug: avoid-rewriting-current-metadata
iri: arqix:requirements/req-01-01-06-02

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-06
      - arqix:user-stories/us-02-01-08
      - arqix:user-stories/us-08-01-06
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Repeated finalise runs on an unchanged document produce no file modification.

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

If a metadata value is already current, then `arqix finalise` SHALL NOT rewrite it.

### Notes

Derived from the acceptance criteria of US-01-01-06 during the group-01 pilot derivation (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`).
Cross-cutting behaviour is linked via the story's `has-requirement`, not restated here.
