---
id: US-03-01-03
title: Generate Coverage Reports
slug: generate-coverage-reports
iri: arqix:user-stories/us-03-01-03

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-03
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-00-00-00-01
      - arqix:requirements/req-00-00-00-03
      - arqix:requirements/req-01-01-08-01
      - arqix:requirements/req-01-01-08-02
      - arqix:requirements/req-01-01-08-03
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-03-01

properties:
  priority: high
  edge-case: false

external-references: []

meta:
  lifecycle-status: retired
  owner: hcf
  created: 2026-03-30
  updated: 2026-07-11
  lang: en
  translation-of:
  generated: false
---

## Generate Coverage Reports

As a QA engineer, I want to generate coverage reports, so that I can detect gaps between requirements, code, and tests objectively and reproducibly.

### Acceptance Criteria

- [ ] `arqix trace coverage` identifies requirements without `verifies` tests.
- [ ] `arqix trace coverage` identifies requirements without `implements` code.
- [ ] Output supports at least Markdown and JSON.
- [ ] Output ordering is deterministic for identical inputs.

### Notes

Acceptance should verify that uncovered requirements are easy to spot and that identical inputs produce identical output ordering.
Add tests that exercise uncovered requirements for both code and tests, plus format checks for Markdown and JSON rendering.
A useful next step is to define whether partially covered requirements should be flagged separately from fully uncovered ones.
The main value for QA is measurable quality evidence and actionable review findings.

Retired in the consolidation sweep of 2026-07-11: this story is a persona clone — its non-cross-cutting requirements are canonically owned by US-01-01-08, and the requirements' derived-from provenance keeps this story's contribution on record.
