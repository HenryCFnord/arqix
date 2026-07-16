---
id: US-07-01-09
title: Report the Codebase Size by Component
slug: report-the-codebase-size-by-component
iri: arqix:user-stories/us-07-01-09

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-10
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-07-01-09-01
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-07-01

properties:
  priority: medium
  edge-case: false

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-16
  updated: 2026-07-16
  lang: en
  translation-of:
  generated: false
---

## Report the Codebase Size by Component

As an assessor, I want the codebase size reported by component, so that question Q-09 of the report catalog has a generated, always-current answer.

### Acceptance Criteria

- [ ] `report snapshot` regenerates a lines-of-code unit counting the Rust source files per component (engine, checkers, tests) with total and non-blank lines.
- [ ] The count is internal and deterministic — no external tool — so the unit joins the snapshot freshness gate like every other unit.

### Notes

Q-09 anticipated an external counter (tokei); an internal count keeps the unit deterministic and gate-compatible, and the catalog's data-source column follows.
