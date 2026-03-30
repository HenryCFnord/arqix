---
id: us-02-01-05
title: Find and read documentation during implementation
slug: find-and-read-documentation-during-implementation
iri: arqix:user-stories/us-02-01-05

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-02
  - predicate: arqix:properties/has-requirement
    object:
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-02-01

properties:
  priority: medium
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

As a Dev Dan, I want to search and read documentation, so that I can quickly find relevant requirements, ADRs, and handbook content while implementing changes.

### Acceptance Criteria

- [ ] Search is available, with full-text search sufficient for the first version.
- [ ] `doc read` supports reading by document ID and optionally by section or anchor.
- [ ] Search results and read output are deterministic for the same input.
- [ ] Missing documents and missing anchors fail with clear diagnostics.

### Notes

The first version is sufficient if users can reliably find a document by text query and read a full document or anchored section without extra tooling. Add tests for exact and partial matches, ambiguous search results, and missing anchors. Keep CLI output concise because the same behavior will likely be reused by MCP later. The main value for Dan is low-friction lookup in the implementation flow.
