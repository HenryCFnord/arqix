---
id: REQ-08-01-40-02
title: Lift Claim Markers Into Derived Triples
slug: lift-claim-markers-into-derived-triples
iri: arqix:requirements/req-08-01-40-02

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-08-01-40
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: fmt writes the derived-triples section with the deduplicated sorted supported-by targets of all claim markers and removes it when no markers remain; fmt --check exits non-zero on a document whose section disagrees; a document without markers stays byte-identical.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-19
  updated: 2026-07-19
  lang: en
  translation-of:
  generated: false
---

## Requirement

When `arqix fmt` rewrites a document, arqix SHALL derive the `derived-triples` section from the document's claim markers — targets deduplicated and sorted, the section absent without markers — and report the disagreement in check mode.

### Notes

The section is formatter-owned; hand edits do not survive the next run (ADR-0004, ADR-0018).
Derived from US-08-01-40.
