---
id: US-01-01-08
title: Generate Governed Coverage Reports
slug: generate-governed-coverage-reports
iri: arqix:user-stories/us-01-01-08

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
  priority: medium
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

## Generate Governed Coverage Reports

As a maintainer, I want to generate coverage reports, so that missing links are surfaced early and documentation quality stays enforceable over time.

### Acceptance Criteria

- [ ] `arqix trace coverage` identifies requirements without `verifies` tests.
- [ ] `arqix trace coverage` identifies requirements without `implements` code.
- [ ] Output supports at least Markdown and JSON.
- [ ] Output ordering is deterministic for identical inputs.

### Notes

Acceptance should verify that uncovered requirements are easy to spot and that identical inputs produce identical output ordering.
Add tests that exercise uncovered requirements for both code and tests, plus format checks for Markdown and JSON rendering.
A useful next step is to define whether partially covered requirements should be flagged separately from fully uncovered ones.
The main value for a maintainer is deterministic repository hygiene and standards enforcement.
