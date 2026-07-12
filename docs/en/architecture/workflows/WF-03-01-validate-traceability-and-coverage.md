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
    object: arqix:personas/per-10
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
  updated: 2026-07-12
  lang: en
  translation-of:
  generated: false
---

## Validate Traceability and Coverage

The repository contains requirements, code, tests, and documentation.
The Assessor needs objective evidence and reproducible metrics instead of manual spreadsheets.

### Goal

Generate traceability data and coverage reports that identify gaps: missing implementations, missing verifications, unresolved IDs — and keep proven ground from eroding.

### Steps

1. Run `trace scan` to build the trace graph from docs, code, and tests.
2. Generate coverage reports to identify missing links; coverage is informational while the corpus grows.
3. Run `trace ratchet` against the committed matrix snapshot: a requirement that was verified must stay verified unless it is retired — growth stays free, regressions gate.
4. Generate matrices for review (US↔REQ, REQ↔Tests) and refresh the committed snapshots.
5. Review diagnostics and assign fixes (docs, code markers, tests).
6. Let the done claim close the loop: a story may declare `done` only when every requirement it carries is verified by an active test, and the linter checks that claim mechanically (LNT-005).

### Outputs

- Trace graph (machine-readable)
- Coverage report (missing implements/verifies)
- Ratchet verdict against the committed baseline
- Trace matrices (MD/CSV/JSON)
- Machine-checked story done claims

### Failure Modes

- Markers missing in tests or code.
- IDs are inconsistent or unresolved.
- A previously verified requirement loses its active test (ratchet regression).
- A story claims done while a carried requirement is unverified (LNT-005).
- Reports are non-deterministic (noisy diffs).

### Related Commands

- `arqix trace scan`
- `arqix trace coverage`
- `arqix trace ratchet`
- `arqix trace matrix`
- `arqix lint run`
