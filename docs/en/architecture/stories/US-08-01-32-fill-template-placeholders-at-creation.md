---
id: US-08-01-32
title: Fill Template Placeholders at Creation
slug: fill-template-placeholders-at-creation
iri: arqix:user-stories/us-08-01-32

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-08
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-08-01-27-02
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-08-01

properties:
  priority: medium
  edge-case: false

external-references: []

meta:
  lifecycle-status: retired
  owner: hcf
  created: 2026-07-19
  updated: 2026-07-19
  lang: en
  translation-of:
  generated: false
---

## Fill Template Placeholders at Creation

As a coding agent, I want `doc new` to fill template placeholders from explicit arguments, so that a template-driven document needs no hand editing after creation.

### Acceptance Criteria

- [ ] `doc new` and the creation aliases accept repeatable `--set key=value` arguments; each fills the `{key}` placeholder in the template.
- [ ] A `--set` key the template does not use is an error naming the key (TPL-003) — a typo never vanishes silently.
- [ ] A template placeholder that no built-in value and no `--set` covers stays a TPL-002 finding, unchanged.

### Notes

Extends the declared-template contract (US-08-01-26): the built-in placeholder vocabulary stays fixed, and `--set` supplies the template's own placeholders per call.
A second arqix-governed corpus hand-edits every created term for exactly these fields (FR-B1 in the second intake of `docs/en/plans/knowledge-repository-2026-07-15/`).
