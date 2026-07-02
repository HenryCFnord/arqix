---
id: REQ-01-01-03-01
title: Sort Frontmatter Keys Canonically
slug: sort-frontmatter-keys-canonically
iri: arqix:requirements/req-01-01-03-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object: arqix:user-stories/us-01-01-03
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Frontmatter keys of formatted documents appear in the configured `key_order`.

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

When `arqix fmt` runs, arqix SHALL sort frontmatter keys according to the configured `key_order`.

### Notes

Derived from the acceptance criteria of US-01-01-03 during the group-01 pilot derivation (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`). Cross-cutting behaviour is linked via the story's `has-requirement`, not restated here.
