---
id: REQ-08-01-40-03
title: Let Derived Triples Join the Graph Checks
slug: let-derived-triples-join-the-graph-checks
iri: arqix:requirements/req-08-01-40-03

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
  fit-criterion: A derived-triples entry pointing at a missing document is ONT-003, one pointing at a non-source document is ONT-007 against the declared range; the optional key after triples passes the format contract.

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

When `arqix lint frontmatter` checks a document carrying a `derived-triples` section, arqix SHALL validate its entries exactly like declared triples, including target resolution and the declared range of `supported-by`.

### Notes

The optional `derived-triples` key follows `triples` in the canonical key order.
Derived from US-08-01-40.
