---
id: us-02-01-08
title: Assemble documentation during implementation
slug: assemble-documentation-during-implementation
iri: arqix:user-stories/us-02-01-08

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

As a Dev Dan, I want to assemble a doc package into pages, so that I can preview publishable documentation built from units while implementing changes.

### Acceptance Criteria

- [ ] `arqix assemble build <doc-package>` generates outputs under `pages/`.
- [ ] `strip_frontmatter_on_include` can be enabled via configuration.
- [ ] Include cycles are detected and fail with a clear error message.
- [ ] Output ordering is deterministic across repeated runs.

### Notes

The build flow is complete when a doc package with nested includes produces stable page outputs and cycles fail fast with a readable path trace. Add tests for frontmatter stripping on included content and for deterministic output ordering across repeated runs. The first implementation should optimise for clear diagnostics over aggressive assembly features. The main value for Dan is fast feedback on document structure during normal development.
