---
id: US-05-01-10
title: Read Structured Document Sections with Stable Selectors
slug: read-structured-document-sections-with-stable-selectors
iri: arqix:user-stories/us-05-01-10

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-10
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-00-00-00-01
      - arqix:requirements/req-00-00-00-04
      - arqix:requirements/req-05-01-10-01
      - arqix:requirements/req-05-01-10-02
      - arqix:requirements/req-05-01-10-03
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
  updated: 2026-07-12
  lang: en
  translation-of:
  generated: false
---

## Read Structured Document Sections with Stable Selectors

As an AIOps engineer, I want to read specific sections of a document using stable selectors, so that agents can cite the right context without reparsing whole files.

### Acceptance Criteria

- [ ] `arqix doc read` supports section-level retrieval by heading slug or explicit anchor.
- [ ] Structured output includes resolved document metadata and selector details.
- [ ] Failures identify whether the document or selector was not found.
- [ ] Selector-based reads are deterministic across repeated runs on the same input.

### Notes

Acceptance should show that agents can cite the right context without reparsing whole files.
Add tests for heading-slug selectors, explicit anchors, and failure cases for missing documents and selectors.
This is a core precise-retrieval capability.
