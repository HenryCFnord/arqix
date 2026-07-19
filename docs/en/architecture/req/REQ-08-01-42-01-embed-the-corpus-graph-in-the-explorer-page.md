---
id: REQ-08-01-42-01
title: Embed the Corpus Graph in the Explorer Page
slug: embed-the-corpus-graph-in-the-explorer-page
iri: arqix:requirements/req-08-01-42-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-08-01-42
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: The generated page carries every trace-graph node and edge as embedded data; document nodes carry type and declared lifecycle status, artefact nodes stay distinguishable so the code layer can be toggled.

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

When `arqix report graph` runs, arqix SHALL embed every trace-graph node and edge in the generated explorer page, each document node carrying its type and declared lifecycle status.

### Notes

The embedded model is the trace core graph (ADR-0006) enriched with title and lifecycle status; enrichment decorates existing nodes and never invents new ones (ADR-0007, ADR-0020).
Derived from US-08-01-42.
