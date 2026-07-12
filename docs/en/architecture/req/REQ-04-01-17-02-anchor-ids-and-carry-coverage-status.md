---
id: REQ-04-01-17-02
title: Anchor IDs and Carry Coverage Status
slug: anchor-ids-and-carry-coverage-status
iri: arqix:requirements/req-04-01-17-02

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-04-01-17
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: A staged catalogue page carries an HTML anchor per story and requirement ID, and every requirement row shows verified, planned, or uncovered per the trace graph.

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

Each catalogue entry SHALL carry an anchor for its ID and, for requirements, the coverage status from the trace graph.

### Notes

Derived from US-04-01-17.
The status vocabulary is the trace engine's: verified (an active test claims the requirement), planned (only ignored tests claim it), uncovered.
