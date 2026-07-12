---
id: US-07-01-06
title: Export Deterministic Trace and Coverage Evidence
slug: export-deterministic-trace-and-coverage-evidence
iri: arqix:user-stories/us-07-01-06

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-10
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-00-00-00-01
      - arqix:requirements/req-00-00-00-03
      - arqix:requirements/req-00-00-00-06
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
  updated: 2026-07-12
  lang: en
  translation-of:
  generated: false
---

## Export Deterministic Trace and Coverage Evidence

As an auditor, I want trace graphs, matrices, and coverage reports to be deterministic, so that audit evidence remains stable across repeated runs and review packages stay comparable.

### Acceptance Criteria

- [ ] `trace scan` JSON output orders nodes and edges deterministically.
- [ ] `trace matrix` outputs deterministic row and column ordering, with configurable defaults allowed.
- [ ] `trace coverage` output is deterministic.
- [ ] CSV and JSON outputs use stable field ordering where applicable.

### Notes

This story treats output ordering and formatting as part of the contract, not as incidental behavior.
The scope includes deterministic ordering for trace graphs, matrices, and coverage reports, along with stable JSON and CSV formatting.
Out of scope are semantic diffing, change explanation, and performance tuning.
The main value for Avery is stable evidence chains for audit review.
