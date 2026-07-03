---
id: US-08-01-05
title: Create Documents Deterministically from Templates
slug: create-documents-deterministically-from-templates
iri: arqix:user-stories/us-08-01-05

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-08
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-00-00-00-04
      - arqix:requirements/req-00-00-00-05
      - arqix:requirements/req-00-00-00-06
      - arqix:requirements/req-01-01-05-01
      - arqix:requirements/req-01-01-05-02
      - arqix:requirements/req-01-01-05-03
      - arqix:requirements/req-08-01-05-01
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-08-01

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


## Create Documents Deterministically from Templates

As a coding agent, I want to create documents from templates, so that I can generate compliant artefacts deterministically without guessing kinds, placeholders, or file structure.

### Acceptance Criteria

- [ ] `arqix doc new <kind>` creates files from templates.
- [ ] Supported `<kind>` values come exclusively from configuration.
- [ ] Aliases such as `req new`, `us new`, and `adr new` are available, or their absence is clearly documented via `doc new`.
- [ ] Templates support the placeholders `{title}`, `{slug}`, and `{id}`.
- [ ] Placeholder substitution for `{slug}` and `{id}` is deterministic for the same title input and configuration.
- [ ] Unknown kinds fail with a clear, actionable error.

### Notes

Acceptance should verify that every configured template kind renders the expected file skeleton with placeholders resolved consistently.
Add tests for supported aliases, unknown kinds, and deterministic `{slug}` and `{id}` substitution from the same title.
The main value for a coding agent is deterministic, configuration-driven document creation with clear failure behaviour and no hidden defaults.
