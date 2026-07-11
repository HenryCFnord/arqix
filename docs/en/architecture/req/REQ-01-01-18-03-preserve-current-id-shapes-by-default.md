---
id: REQ-01-01-18-03
title: Preserve Current ID Shapes by Default
slug: preserve-current-id-shapes-by-default
iri: arqix:requirements/req-01-01-18-03

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
  fit-criterion: With an empty `arqix.toml`, ID generation, ID validation, and the trace graph match the pre-policy behaviour exactly.

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

Where no ID policy is configured, arqix SHALL apply default patterns that reproduce the built-in ID shapes.

### Notes

Derived from US-01-01-18.
This is the defaults-preserve-the-present rule of ADR-0011 applied to the heaviest configured value: shipping the policy surface must not change an existing corpus.
