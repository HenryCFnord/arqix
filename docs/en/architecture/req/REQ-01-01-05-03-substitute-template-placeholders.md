---
id: REQ-01-01-05-03
title: Substitute Template Placeholders
slug: substitute-template-placeholders
iri: arqix:requirements/req-01-01-05-03

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-05
      - arqix:user-stories/us-02-01-05
      - arqix:user-stories/us-06-01-03
      - arqix:user-stories/us-08-01-05
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: A template containing the three placeholders yields a document with all of them replaced by the actual values.

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

When a document is created from a template, arqix SHALL substitute the placeholders `{title}`, `{slug}`, and `{id}`.

### Notes

Derived from the acceptance criteria of US-01-01-05 during the group-01 pilot derivation (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`).
Cross-cutting behaviour is linked via the story's `has-requirement`, not restated here.
