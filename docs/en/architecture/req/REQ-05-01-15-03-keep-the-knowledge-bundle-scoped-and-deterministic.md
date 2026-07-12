---
id: REQ-05-01-15-03
title: Keep the Knowledge Bundle Scoped and Deterministic
slug: keep-the-knowledge-bundle-scoped-and-deterministic
iri: arqix:requirements/req-05-01-15-03

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-05-01-15
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: Excluded subtrees and retired documents never reach the bundle, and two exports over identical inputs are identical.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-12
  updated: 2026-07-12
  lang: en
  translation-of:
  generated: false
---

## Requirement

When exporting an OKF bundle, arqix SHALL produce a bundle that honours the publish scope and the document lifecycle and is identical for identical inputs.

### Notes

Derived from US-05-01-15.
The publish scope is the configured exclude list; retired documents leave living knowledge exactly as they leave progress denominators (ADR-0010).
