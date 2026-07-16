---
id: REQ-04-01-19-01
title: Stage CSV Artefacts as Table Pages
slug: stage-csv-artefacts-as-table-pages
iri: arqix:requirements/req-04-01-19-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-04-01-19
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: After publish site, every corpus CSV has a staged Markdown page at its corpus location whose table carries the CSV's header and rows with pipe characters escaped, and no CSV file reaches the staging tree unconverted.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-16
  updated: 2026-07-16
  lang: en
  translation-of:
  generated: false
---

## Requirement

When `arqix publish site` stages the corpus, arqix SHALL stage every corpus CSV artefact as a Markdown page rendering the CSV rows as a table.

### Notes

Generated at staging time (never committed), titled by the file name, with a provenance line naming the source CSV.
Derived from US-04-01-19.
