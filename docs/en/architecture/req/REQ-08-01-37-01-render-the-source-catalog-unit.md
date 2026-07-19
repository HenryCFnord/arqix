---
id: REQ-08-01-37-01
title: Render the Source Catalog Unit
slug: render-the-source-catalog-unit
iri: arqix:requirements/req-08-01-37-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-08-01-37
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: report snapshot writes units/source-catalog.md with one row per source document sorted by id and the provenance columns from the frontmatter; a corpus without sources renders the empty table.

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

When `arqix report snapshot` runs, arqix SHALL render the source-catalog unit with one row per source document, sorted by id, projecting the provenance columns from the frontmatter.

### Notes

Question Q-11; the unit joins the snapshot freshness gate like every other unit.
Derived from US-08-01-37.
