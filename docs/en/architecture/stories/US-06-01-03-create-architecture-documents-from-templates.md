---
id: US-06-01-03
title: Create Architecture Documents from Templates
slug: create-architecture-documents-from-templates
iri: arqix:user-stories/us-06-01-03

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-06
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-00-00-00-04
      - arqix:requirements/req-00-00-00-05
      - arqix:requirements/req-00-00-00-06
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-06-01

properties:
  priority: high
  edge-case: false

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-03-30
  updated: 2026-07-02
  lang: en
  translation-of:
  generated: false
---


## Create Architecture Documents from Templates

As an architect, I want to create documents from templates, so that ADRs and related documentation artefacts are structured correctly from the start.

### Acceptance Criteria

- [ ] `arqix doc new <kind>` creates files from templates.
- [ ] Supported `<kind>` values come exclusively from configuration.
- [ ] Aliases such as `req new`, `us new`, and `adr new` are available, or their absence is clearly documented via `doc new`.
- [ ] Templates support the placeholders `{title}`, `{slug}`, and `{id}`.
- [ ] Placeholder substitution for `{slug}` and `{id}` is deterministic for the same title input and configuration.

### Notes

Acceptance should verify that every configured template kind renders the expected file skeleton with placeholders resolved consistently.
Add tests for supported aliases, unknown kinds, and deterministic `{slug}` and `{id}` substitution from the same title.
The main value for an architect is consistent structure for ADRs and other architecture documentation artefacts.