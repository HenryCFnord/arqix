---
id: US-06-01-09
title: Retrieve Architecture Documentation Quickly
slug: retrieve-architecture-documentation-quickly
iri: arqix:user-stories/us-06-01-09

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-09
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-00-00-00-01
      - arqix:requirements/req-00-00-00-11
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-06-01

properties:
  priority: medium
  edge-case: false

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-03-30
  updated: 2026-07-12
  lang: en
  translation-of:
  generated: false
---

## Retrieve Architecture Documentation Quickly

As an architect, I want to search and read documentation, so that I can quickly retrieve architecture decisions, glossary terms, and handbook content while maintaining the documentation narrative.

### Acceptance Criteria

- [ ] Search is available, with full-text search sufficient for the first version.
- [ ] `doc read` supports reading by document ID and optionally by section or anchor.
- [ ] Search results and read output are deterministic for the same input.
- [ ] Missing documents and missing anchors fail with clear diagnostics.

### Notes

The first version is sufficient if users can reliably find a document by text query and read a full document or anchored section without extra tooling.
Add tests for exact and partial matches, ambiguous search results, and missing anchors.
Keep CLI output concise because the same behavior will likely be reused by MCP later.
The main value for Aria is fast access to modular architecture knowledge.
