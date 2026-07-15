---
id: REQ-01-01-22-01
title: Create Documents in the Declared Directory
slug: create-documents-in-the-declared-directory
iri: arqix:requirements/req-01-01-22-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-22
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: With [kinds.adr] dir = "docs/decisions" configured, doc new adr plans and writes the document under docs/decisions/; without a contract the target stays <first-root>/adr/.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-15
  updated: 2026-07-15
  lang: en
  translation-of:
  generated: false
---

## Requirement

When a `[kinds.<family>]` contract declares a `dir`, arqix SHALL create documents of that family under the declared directory.

### Notes

Derived from US-01-01-22 (authoring-ergonomics band, knowledge-repository intake gap G2).
The declared `dir` already governs the family's validation (US-01-01-19); this requirement binds creation to the same single source (ADR-0011), and an unconfigured family keeps the `<first-root>/<kind>/` default, so present behaviour is preserved exactly.
