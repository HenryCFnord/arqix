---
id: US-03-01-02
title: Export Trace Matrices
slug: export-trace-matrices
iri: arqix:user-stories/us-03-01-02

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-03
  - predicate: arqix:properties/has-requirement
    object:
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-03-01

properties:
  priority: high
  edge-case: false

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-03-30
  updated: 2026-04-06
  lang: en
  translation-of:
  generated: false
---

## Export Trace Matrices

As a QA engineer, I want to export a trace matrix, so that I can analyze relationships such as REQ×Test and US×REQ in tabular form.

### Acceptance Criteria

- [ ] `arqix trace matrix` can export CSV.
- [ ] At least `REQ×Test` and `US×REQ` matrices are supported.
- [ ] Exported CSV uses stable headers and a deterministic row model for each supported matrix type.
- [ ] Empty-link cases still appear in a reviewer-friendly form.

### Notes

Acceptance should confirm that the exported CSV has stable headers and one row model per supported matrix type.
Add tests for both `REQ×Test` and `US×REQ`, including empty-link cases that should still appear in a reviewer-friendly form.
Keep the command explicit about which matrix is being generated so downstream analysis stays predictable.
This is the canonical traceability analysis view.
