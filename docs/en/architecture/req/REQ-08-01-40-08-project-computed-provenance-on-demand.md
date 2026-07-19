---
id: REQ-08-01-40-08
title: Project Computed Provenance on Demand
slug: project-computed-provenance-on-demand
iri: arqix:requirements/req-08-01-40-08

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
  priority: medium
  fit-criterion: report claims --provenance in a repository with history appends author, date, commit, and agent-involvement per marker; without history the plain projection prints unchanged; the gated claims export never carries the computed columns.

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

When `arqix report claims` runs with `--provenance` in a repository with history, arqix SHALL append the computed provenance columns — author, date, commit, and agent involvement — to every claim row.

### Notes

The computed floor (ADR-0019 carrier one): history-derived values stay an on-demand projection and never enter a gated snapshot.
Derived from US-08-01-40.
