---
id: WF-07-01
title: Review Evidence Chains and Exports
slug: review-evidence-chains-and-exports
iri: arqix:workflows/wf-07-01

rdf:
  type:
    - arqix:classes/workflow

triples:
  - predicate: arqix:properties/has-primary-persona
    object: arqix:personas/per-07
  - predicate: arqix:properties/has-relevant-persona
    object:

properties:
  goal: Provide deterministic reports and exports that allow drilling down via stable IDs.
  entry-state: Audits and compliance reviews require reproducible evidence across requirements, decisions, implementation, tests, and outputs.
  end-state: Trace matrices, coverage reports, and optional export bundles are available for handoff and review.

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-03-25
  updated: 2026-03-28
  lang: en
  translation-of:
  generated: false
---

## Review Evidence Chains and Exports

Audits and compliance reviews require reproducible evidence: requirements, decisions, implementation, tests, and outputs.

### Goal

Provide deterministic reports and exports that allow drilling down via stable IDs.

### Steps

1. Review trace matrices and coverage reports.
2. Drill down into specific requirements and linked evidence (docs, code markers, tests).
3. Confirm that reports are reproducible and consistent across runs.
4. Produce an export bundle if required for handoff.

### Outputs

- Trace matrices (CSV/MD/JSON)
- Coverage report (missing implements/verifies)
- Optional export bundle of key artefacts

### Failure Modes

- Reports are incomplete or non-deterministic.
- IDs are inconsistent across docs.
- Missing links between requirements and tests.

### Related Commands

- `arqix trace matrix`
- `arqix trace coverage`
- `arqix publish artefacts` (later)
