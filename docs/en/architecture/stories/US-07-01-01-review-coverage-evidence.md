---
id: US-07-01-01
title: Review Coverage Evidence
slug: review-coverage-evidence
iri: arqix:user-stories/us-07-01-01

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-07
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-00-00-00-01
      - arqix:requirements/req-00-00-00-03
      - arqix:requirements/req-01-01-08-01
      - arqix:requirements/req-01-01-08-02
      - arqix:requirements/req-01-01-08-03
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-07-01

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

## Review Coverage Evidence

As an auditor, I want to generate coverage reports, so that I can review evidence chains quickly and identify missing implementation or verification links.

### Acceptance Criteria

- [ ] `arqix trace coverage` identifies requirements without `verifies` tests.
- [ ] `arqix trace coverage` identifies requirements without `implements` code.
- [ ] Output supports at least Markdown and JSON.
- [ ] Output ordering is deterministic for identical inputs.

### Notes

Acceptance should verify that uncovered requirements are easy to spot and that identical inputs produce identical output ordering.
Add tests that exercise uncovered requirements for both code and tests, plus format checks for Markdown and JSON rendering.
A useful next step is to define whether partially covered requirements should be flagged separately from fully uncovered ones.
The main value for Avery is reproducible audit evidence that can be consumed without manual data wrangling.
