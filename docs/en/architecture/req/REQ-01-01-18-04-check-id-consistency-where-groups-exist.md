---
id: REQ-01-01-18-04
title: Check ID Consistency Where Groups Exist
slug: check-id-consistency-where-groups-exist
iri: arqix:requirements/req-01-01-18-04

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
  priority: medium
  fit-criterion: Under the default policy, an ID whose story slice contradicts the declared owner triple is reported; under a group-free pattern, no such check runs.

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

Where the configured ID pattern declares semantic groups, arqix SHALL report inconsistencies between the values encoded in an ID and the document's declared triples.

### Notes

Derived from US-01-01-18.
Named groups activate checks, not derivation (ADR-0012): a `story` group is checked against the first `derived-from` triple, a `seq` group drives the per-story sequencing validation.
This is how arqix's own corpus keeps its ID discipline while group-free policies stay first-class.
