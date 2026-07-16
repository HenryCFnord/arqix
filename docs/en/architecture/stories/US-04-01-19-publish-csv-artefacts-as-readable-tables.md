---
id: US-04-01-19
title: Publish CSV Artefacts as Readable Tables
slug: publish-csv-artefacts-as-readable-tables
iri: arqix:user-stories/us-04-01-19

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-09
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-04-01-19-01
      - arqix:requirements/req-04-01-19-02
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-04-01

properties:
  priority: medium
  edge-case: false

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-16
  updated: 2026-07-16
  lang: en
  translation-of:
  generated: false
---

## Publish CSV Artefacts as Readable Tables

As a builder, I want the corpus CSV artefacts published as readable table pages, so that the trace matrices and data exports are browsable on the site instead of missing from it.

### Acceptance Criteria

- [ ] `publish site` stages every corpus CSV as a Markdown page rendering its rows as a table, next to the CSV's corpus location.
- [ ] Corpus links to a CSV are rewritten in the staged pages to the table page, so they resolve on the site while the repository view keeps linking the raw CSV.
- [ ] The pipe character and other cell content survive the conversion escaped; identical CSV input produces an identical page.

### Notes

The site toolchain renders Markdown only, so CSVs (trace matrices, the normative-statement export) never reached the published site.
The conversion happens at staging time and is never committed — the CSV stays the single committed artefact, exactly like the generated specification catalogue (US-04-01-17).
