---
id: US-04-01-12
title: Publish Stable Report Exports for Automation
slug: publish-stable-report-exports-for-automation
iri: arqix:user-stories/us-04-01-12

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-09
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-00-00-00-03
      - arqix:requirements/req-04-01-12-01
      - arqix:requirements/req-04-01-12-02
      - arqix:requirements/req-04-01-12-03
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-04-01

properties:
  priority: medium
  edge-case: false

external-references: []

meta:
  lifecycle-status: done
  owner: hcf
  created: 2026-03-30
  updated: 2026-07-12
  lang: en
  translation-of:
  generated: false
---

## Publish Stable Report Exports for Automation

As a DevOps engineer, I want stable compliance-ready report exports, so that CI and release automation can attach reviewable report packages without manual post-processing.

### Acceptance Criteria

- [x] Audit-oriented exports support at least Markdown, CSV, and JSON where applicable.
- [x] Export schemas and column ordering are stable across runs.
- [x] Report metadata records generation time, scope, and source inputs.

### Notes

The main value for Daria is deterministic export contracts that can be attached or published directly from automation.
