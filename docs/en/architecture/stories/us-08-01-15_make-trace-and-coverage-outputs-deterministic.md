---
id: us-08-01-15
title: Make trace and coverage outputs deterministic
slug: make-trace-and-coverage-outputs-deterministic
iri: arqix:user-stories/us-08-01-15

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-08
  - predicate: arqix:properties/has-requirement
    object:
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-08-01

properties:
  priority: high
  edge-case: false

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-03-30
  updated: 2026-03-30
  lang: en
  translation-of:
  generated: false
---

## User-story

As a Casey Coding Agent, I want trace graphs, matrices, and coverage reports to be deterministic, so that I can produce clean diffs and reliably detect meaningful changes.

### Acceptance Criteria

- [ ] `trace scan` JSON output orders nodes and edges deterministically.
- [ ] `trace matrix` outputs deterministic row and column ordering, with configurable defaults allowed.
- [ ] `report coverage` output is deterministic.
- [ ] CSV and JSON outputs use stable field ordering where applicable.

### Notes

This story treats output ordering and formatting as part of the contract, not as incidental behavior. The scope includes deterministic ordering for trace graphs, matrices, and coverage reports, along with stable JSON and CSV formatting. Out of scope are semantic diffing, change explanation, and performance tuning.
