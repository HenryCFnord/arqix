---
id: REQ-01-01-19-02
title: Share One Frontmatter Contract Source
slug: share-one-frontmatter-contract-source
iri: arqix:requirements/req-01-01-19-02

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
  fit-criterion: One edit to the configured contract changes formatter and validation behaviour together; no second copy of the contract lives in code.

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

arqix SHALL validate frontmatter contracts against the same configured source the formatter reads.

### Notes

Derived from US-01-01-19.
The one-source rule of ADR-0011: `fmt` must never format what validation then flags.
The Python reference checker reads the same source (audit rows C1, C2, C6), keeping the reference and the engine conformance-comparable.
