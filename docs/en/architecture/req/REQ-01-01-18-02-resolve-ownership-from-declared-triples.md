---
id: REQ-01-01-18-02
title: Resolve Ownership from Declared Triples
slug: resolve-ownership-from-declared-triples
iri: arqix:requirements/req-01-01-18-02

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-18
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: With a group-free ID pattern, the us-req matrix and the story-progress projection are complete and identical to the triple-declared ownership.

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

When the trace graph resolves the owning story of a requirement, arqix SHALL read it from the requirement's first `derived-from` triple.

### Notes

Derived from US-01-01-18.
Declared triples are the source of truth for relations; the ID is an opaque label (ADR-0012).
This replaces the ID-slice derivation in the trace engine and the oracle; both sides move together under the conformance suite.
