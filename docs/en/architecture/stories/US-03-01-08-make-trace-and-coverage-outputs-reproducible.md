---
id: US-03-01-08
title: Make Trace and Coverage Outputs Reproducible
slug: make-trace-and-coverage-outputs-reproducible
iri: arqix:user-stories/us-03-01-08

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
    object: arqix:workflows/wf-03-01

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

## Make Trace and Coverage Outputs Reproducible

As a QA engineer, I want trace graphs, matrices, and coverage reports to be deterministic, so that repeated runs produce reproducible quality evidence and reviewer-friendly comparisons.

### Acceptance Criteria

- [ ] `trace scan` JSON output orders nodes and edges deterministically.
- [ ] `trace matrix` outputs deterministic row and column ordering, with configurable defaults allowed.
- [ ] `trace coverage` output is deterministic.
- [ ] CSV and JSON outputs use stable field ordering where applicable.

### Notes

This story treats output ordering and formatting as part of the contract, not as incidental behavior.
The scope includes deterministic ordering for trace graphs, matrices, and coverage reports, along with stable JSON and CSV formatting.
Out of scope are semantic diffing, change explanation, and performance tuning.
The main value for Quinn is reproducible evidence for quality review.
