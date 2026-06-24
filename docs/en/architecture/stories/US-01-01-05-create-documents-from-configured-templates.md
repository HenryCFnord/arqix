---
id: US-01-01-05
title: Create Documents from Configured Templates
slug: create-documents-from-configured-templates
iri: arqix:user-stories/us-01-01-05

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-01
  - predicate: arqix:properties/has-requirement
    object:
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-01-01

properties:
  priority: high
  edge-case: false

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-03-30
  updated: 2026-04-05
  lang: en
  translation-of:
  generated: false
---

## Create Documents from Configured Templates

As a maintainer, I want to create documents from templates, so that new artefacts such as requirements, user stories, and ADRs are compliant from the start.

### Acceptance Criteria

- [ ] `arqix doc new <kind>` creates files from templates.
- [ ] Supported `<kind>` values come exclusively from configuration.
- [ ] Aliases such as `req new`, `us new`, and `adr new` are available, or their absence is clearly documented via `doc new`.
- [ ] Templates support the placeholders `{title}`, `{slug}`, and `{id}`.
- [ ] Placeholder substitution for `{slug}` and `{id}` is deterministic for the same title input and configuration.

### Notes

Acceptance should verify that every configured template kind renders the expected file skeleton with placeholders resolved consistently.
Add tests for supported aliases, unknown kinds, and deterministic `{slug}` and `{id}` substitution from the same title.
Template configuration should remain the single source of truth so command behavior and documentation stay aligned.
This is a core standards and repository hygiene capability.
