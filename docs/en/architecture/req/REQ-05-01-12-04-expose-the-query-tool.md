---
id: REQ-05-01-12-04
title: Expose the Query Tool
slug: expose-the-query-tool
iri: arqix:requirements/req-05-01-12-04

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-05-01-12
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: tools/list carries a query tool; a tools/call with kind, lifecycle, and edge arguments answers with the same payload doc query returns for the same filters.

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

When an MCP client calls the query tool, arqix SHALL answer with the same payload `arqix doc query` returns for the same filters.

### Notes

The tool joins search, read, and list over the shared store function — the transport-separation contract (REQ-05-01-12-03) extends unchanged (ADR-0023).
Derived from US-05-01-12.
