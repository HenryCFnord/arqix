---
id: REQ-01-01-19-01
title: Order Frontmatter Keys from Configuration
slug: order-frontmatter-keys-from-configuration
iri: arqix:requirements/req-01-01-19-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-19
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: "`fmt` orders frontmatter keys exactly as the family's configured key order lists them; the default configuration leaves the corpus byte-identical."

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-11
  updated: 2026-07-11
  lang: en
  translation-of:
  generated: false
---

## Requirement

When `arqix fmt` canonicalises frontmatter, arqix SHALL order keys by the per-family key order in the effective configuration.

### Notes

Derived from US-01-01-19.
Replaces the per-family key-order tables hardcoded in the rewriter (audit row C1).
