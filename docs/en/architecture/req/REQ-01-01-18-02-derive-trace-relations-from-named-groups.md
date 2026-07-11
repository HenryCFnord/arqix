---
id: REQ-01-01-18-02
title: Derive Trace Relations from Named Groups
slug: derive-trace-relations-from-named-groups
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
  fit-criterion: Owner-story slice, cross-cutting domain, and sequence positions come from the pattern's named groups, not from fixed character offsets.

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

When the trace graph derives owner-story or sequencing relations from a document ID, arqix SHALL consume the named groups declared in the configured ID pattern.

### Notes

Derived from US-01-01-18.
The group semantics — `story` as the owner slice, `seq` as per-story sequencing, the configured cross-cutting domain marking ownerless requirements — are decided in ADR-0012.
