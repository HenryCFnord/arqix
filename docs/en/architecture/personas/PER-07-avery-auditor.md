---
id: PER-07
title: Avery Auditor
slug: avery-auditor
iri: arqix:personas/per-07

rdf:
  type:
    - arqix:classes/persona

triples: []

properties:
  role: Auditor focused on traceability and compliance evidence
  description: Evaluates reproducible evidence chains from requirements to implementation and verification, relying on deterministic reports and stable IDs.

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-03-05
  updated: 2026-03-28
  lang: en
  translation-of:
  generated: false
---

## Avery Auditor

Avery evaluates evidence chains for audits and compliance reviews.
Avery is not interested in tooling details, but in reproducible proof: what was required, what was decided, what was implemented, and what was verified.

### Goals

- Review traceability from requirements to code and tests.
- Consume deterministic reports without manual data wrangling.
- Identify gaps and risks quickly.
- Rely on stable IDs for references.

### Success Looks Like

- Trace matrices and coverage reports are reproducible.
- Findings link to concrete documents and file locations.
- Evidence chains are complete and explainable.
- Documentation is publishable and navigable.

### Pain Points

- “It is documented somewhere” responses.
- Manual spreadsheets and ad-hoc reports.
- Missing links between requirements and tests.
- Non-reproducible documentation outputs.

### Typical Workflow with arqix

Avery starts with trace matrices and coverage reports, then drills down via stable IDs to relevant requirements, ADRs, and implementation evidence.

### Important arqix Capabilities and Commands

- `trace scan`
- `trace matrix`
- `trace coverage`
- `publish artefacts` (later)
- Deterministic exports (CSV and JSON)

### artefacts They Care About

- Trace matrices
- Coverage reports
- ADRs and architecture docs
- Published documentation outputs

### Boundaries

Avery does not create documents or pipelines.
Avery consumes evidence and reports.

### Open Needs

Avery benefits from filtered report views, stable export schemas, and consistent bilingual publishing.
