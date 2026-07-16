---
id: REQ-04-01-19-02
title: Rewrite CSV Links to the Table Pages
slug: rewrite-csv-links-to-the-table-pages
iri: arqix:requirements/req-04-01-19-02

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
  fit-criterion: A corpus page linking to a relative .csv target links to the corresponding .md table page after staging, while the committed corpus page keeps linking the raw CSV.

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

When `arqix publish site` stages a corpus page, arqix SHALL rewrite the page's relative CSV links to the staged table pages.

### Notes

The repository view keeps the raw-CSV link (GitHub renders CSVs); the site gets the page that exists there.
Derived from US-04-01-19.
