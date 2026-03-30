---
id: us-05-01-06
title: Export a deterministic document catalog
slug: export-a-deterministic-document-catalog
iri: arqix:user-stories/us-05-01-06

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-05
  - predicate: arqix:properties/has-requirement
    object:
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-05-01

properties:
  priority: high
  edge-case: false

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-03-30
  updated: 2026-03-30
  lang: en
  translation-of:
  generated: false
---

## User-story

As a Alex AIOps, I want a deterministic document catalog export, so that downstream indexing and retrieval systems can consume arqix content without scraping Markdown.

### Acceptance Criteria

- [ ] `arqix doc list` can emit JSON with stable ordering and core metadata for each document.
- [ ] Catalog entries include at minimum `id`, `kind`, `title`, `lang`, and source path.
- [ ] Filtering by kind and language is supported.
- [ ] The catalog output is deterministic across repeated runs on the same input.

### Notes

Acceptance should show that downstream indexing and retrieval systems can consume the catalog without scraping Markdown. Add tests for stable ordering, filtering by kind and language, and deterministic JSON rendering. This is a core machine-readable dataset capability.
