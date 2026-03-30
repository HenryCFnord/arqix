---
id: us-04-01-07
title: Publish stable report exports for automation
slug: publish-stable-report-exports-for-automation
iri: arqix:user-stories/us-04-01-07

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-04
  - predicate: arqix:properties/has-requirement
    object:
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-04-01

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

As a DevOps Daria, I want stable compliance-ready report exports, so that CI and release automation can attach reviewable report packages without manual post-processing.

### Acceptance Criteria

- [ ] Audit-oriented exports support at least Markdown, CSV, and JSON where applicable.
- [ ] Export schemas and column ordering are stable across runs.
- [ ] Report metadata records generation time, scope, and source inputs.

### Notes

The main value for Daria is deterministic export contracts that can be attached or published directly from automation.
