---
id: WF-03-01
title: Validate Traceability and Coverage
slug: validate-traceability-and-coverage
iri: arqix:workflows/wf-03-01

rdf:
  type:
    - arqix:classes/workflow

triples:
  - predicate: arqix:properties/has-primary-persona
    object: arqix:personas/per-03
  - predicate: arqix:properties/has-relevant-persona
    object:

properties:
  goal: Generate traceability data and coverage reports that identify gaps.
  entry-state: The repository contains requirements, code, tests, and documentation, but traceability evidence and coverage gaps still need objective validation.
  end-state: Trace graph, coverage reports, and trace matrices are generated so missing implementations, verifications, and unresolved IDs can be reviewed and fixed.

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2025-03-25
  updated: 2025-03-28
  lang: en
  translation-of:
  generated: false
---

## Validate Traceability and Coverage

The repository contains requirements, code, tests, and documentation.
QA needs objective evidence and

reproducible metrics instead of manual spreadsheets.

### Goal

Generate traceability data and coverage reports that identify gaps:

missing implementations, missing verifications, unresolved IDs.

### Steps

1. Run `trace scan` to build the trace graph from docs, code, and tests.
2. Generate coverage reports to identify missing links.
3. Generate matrices for review (e.g.
   US↔REQ, REQ↔Tests).
4. Review diagnostics and assign fixes (docs, code markers, tests).
5. Export reports for audit or quality dashboards if needed.

### Outputs

- Trace graph (machine-readable)
- Coverage report (missing implements/verifies)
- Trace matrices (MD/CSV/JSON)

### Failure Modes

- Markers missing in tests or code.
- IDs are inconsistent or unresolved.
- Reports are non-deterministic (noisy diffs).

### Related Commands

- `arqix trace scan`
- `arqix trace coverage`
- `arqix trace matrix`
- `arqix lint run`
