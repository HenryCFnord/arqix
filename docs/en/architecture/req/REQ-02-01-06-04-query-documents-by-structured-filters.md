---
id: REQ-02-01-06-04
title: Query Documents by Structured Filters
slug: query-documents-by-structured-filters
iri: arqix:requirements/req-02-01-06-04

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-02-01-06
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: doc query with any combination of --kind, --lifecycle, and repeatable --edge predicate=target filters returns exactly the documents matching every filter, each with its matching declared edges; a target with a trailing * matches as a prefix, edge matching includes external targets, output is deterministic and versioned; a malformed --edge is a usage error.

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

When `arqix doc query` runs with kind, lifecycle, or edge filters, arqix SHALL return exactly the documents matching every given filter, each with its matching declared edges.

### Notes

The filter set is conjunctive and structured — no query language (ADR-0023); the edge target matches exactly or as a prefix with a trailing `*`, and edge matching reads the declared triples from the raw frontmatter so external targets are queryable.
Derived from US-02-01-06.
