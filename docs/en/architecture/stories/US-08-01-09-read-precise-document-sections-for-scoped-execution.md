---

id: US-08-01-09
title: Read precise document sections for scoped execution
slug: read-precise-document-sections-for-scoped-execution
iri: arqix:user-stories/us-08-01-09

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

As a Casey Coding Agent, I want to read specific document sections using stable selectors, so that I can retrieve only the context needed for the current story without reparsing whole files.

### Acceptance Criteria

- [ ] `arqix doc read` supports section-level retrieval by heading slug or explicit anchor.
- [ ] Structured output includes resolved document metadata and selector details.
- [ ] Failures identify whether the document or selector was not found.
- [ ] Selector-based reads are deterministic across repeated runs on the same input.

### Notes

Acceptance should show that agents can cite the right context without reparsing whole files. Add tests for heading-slug selectors, explicit anchors, and failure cases for missing documents and selectors. The main value for Casey is precise, bounded context retrieval.
