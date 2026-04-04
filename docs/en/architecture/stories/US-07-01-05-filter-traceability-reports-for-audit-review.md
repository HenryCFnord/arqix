---


id: US-07-01-05
title: Filter Traceability Reports for Audit Review
slug: filter-traceability-reports-for-audit-review
iri: arqix:user-stories/us-07-01-05

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
  priority: medium
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


## Filter Traceability Reports for Audit Review

As a Avery Auditor, I want filtered traceability report views, so that I can focus on high-risk gaps without manually reshaping raw exports.

### Acceptance Criteria

- [ ] Trace and coverage reports support filtering by document kind, status, and missing-link category.
- [ ] Filtered outputs remain reproducible and exportable.
- [ ] Each finding links back to the originating document or file location.

### Notes

This story improves risk-oriented audit review by letting auditors narrow reports without breaking reproducibility or losing drill-down links.
