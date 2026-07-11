---
id: REQ-01-01-18-01
title: Apply Configured ID Patterns
slug: apply-configured-id-patterns
iri: arqix:requirements/req-01-01-18-01

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
  fit-criterion: Changing a family's configured `id-pattern` changes ID generation and ID validation together, without a code change.

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

When a document ID is generated or validated, arqix SHALL apply the ID pattern configured for the document's family.

### Notes

Derived from US-01-01-18.
The one-source rule (ADR-0011) applies: the Python reference tools read the same configured policy, so the conformance suite pins parity.
