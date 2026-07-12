---
id: US-07-01-07
title: Publish Stable Compliance-Ready Report Exports
slug: publish-stable-compliance-ready-report-exports
iri: arqix:user-stories/us-07-01-07

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-07
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-00-00-00-03
      - arqix:requirements/req-04-01-12-01
      - arqix:requirements/req-04-01-12-02
      - arqix:requirements/req-04-01-12-03
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-07-01

properties:
  priority: medium
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

## Publish Stable Compliance-Ready Report Exports

As an auditor, I want stable compliance-ready report exports, so that review packages can be attached to audits without manual cleanup or reformatting.

### Acceptance Criteria

- [ ] Audit-oriented exports support at least Markdown, CSV, and JSON where applicable.
- [ ] Export schemas and column ordering are stable across runs.
- [ ] Report metadata records generation time, scope, and source inputs.

### Notes

This story focuses on reproducible external review packages with stable schemas and explicit generation metadata.

Retired in the consolidation sweep of 2026-07-11: this story is a persona clone — its non-cross-cutting requirements are canonically owned by US-04-01-12, and the requirements' derived-from provenance keeps this story's contribution on record.
