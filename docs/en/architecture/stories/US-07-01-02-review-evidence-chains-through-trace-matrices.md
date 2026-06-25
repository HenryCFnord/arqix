---
id: US-07-01-02
title: Review Evidence Chains through Trace Matrices
slug: review-evidence-chains-through-trace-matrices
iri: arqix:user-stories/us-07-01-02

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-07
  - predicate: arqix:properties/has-requirement
    object:
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
  updated: 2026-04-04
  lang: en
  translation-of:
  generated: false
---


## Review Evidence Chains through Trace Matrices

As an auditor, I want to export a trace matrix, so that I can review evidence chains such as requirements to tests and user stories to requirements in a reproducible tabular form.

### Acceptance Criteria

- [ ] `arqix trace matrix` can export CSV.
- [ ] At least `REQ×Test` and `US×REQ` matrices are supported.
- [ ] Exported CSV uses stable headers and a deterministic row model for each supported matrix type.
- [ ] Empty-link cases still appear in a reviewer-friendly form.

### Notes

Acceptance should confirm that the exported CSV has stable headers and one row model per supported matrix type. Add tests for both `REQ×Test` and `US×REQ`, including empty-link cases that should still appear in a reviewer-friendly form. Keep the command explicit about which matrix is being generated so downstream analysis stays predictable. The main value for Avery is reviewer-friendly and reproducible audit evidence.
