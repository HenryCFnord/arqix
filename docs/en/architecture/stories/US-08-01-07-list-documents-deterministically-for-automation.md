---

id: US-08-01-07
title: List documents deterministically for automation
slug: list-documents-deterministically-for-automation
iri: arqix:user-stories/us-08-01-07

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-08
  - predicate: arqix:properties/has-requirement
    object:
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-08-01

properties:
  priority: medium
  edge-case: false

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-03-30
  updated: 2026-04-04
  lang: en
  translation-of:
  generated: false
---

## User-story

As a Casey Coding Agent, I want to list documents deterministically, so that I can discover repository content without guessing file structure or parsing raw Markdown.

### Acceptance Criteria

- [ ] `arqix doc list` can emit JSON with stable ordering and core metadata for each document.
- [ ] Catalog entries include at minimum `id`, `kind`, `title`, `lang`, and source path.
- [ ] Filtering by kind and language is supported.
- [ ] The catalog output is deterministic across repeated runs on the same input.

### Notes

Acceptance should show that downstream indexing and retrieval systems can consume the catalog without scraping Markdown. Add tests for stable ordering, filtering by kind and language, and deterministic JSON rendering. The main value for Casey is stable discovery and automation-safe input selection.
