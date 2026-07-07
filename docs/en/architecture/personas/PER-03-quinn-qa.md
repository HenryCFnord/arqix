---
id: PER-03
title: Quinn QA
slug: quinn-qa
iri: arqix:personas/per-03

rdf:
  type:
    - arqix:classes/persona

triples: []

properties:
  role: Quality assurance and traceability analyst
  description: Ensures deterministic quality signals by tracing requirements, implementation, and tests, then surfacing gaps with machine-readable reports.

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-03-05
  updated: 2026-07-02
  lang: en
  translation-of:
  generated: false
---

## Quinn QA

Quinn ensures traceability and measurable quality.
Quinn wants evidence, not opinions.
For Quinn, arqix is valuable when it produces deterministic reports that reveal gaps between requirements, implementation, and tests.

### Goals

- Generate traceability graphs and matrices.
- Detect missing implementations and missing verifications.
- Export reports for reviews and audits.
- Provide actionable feedback to developers.

### Success Looks Like

- Coverage gaps are visible and reproducible.
- Reports are machine-readable and stable across runs.
- Findings point to exact file locations.
- Quality gates are objective and enforceable.

### Pain Points

- Requirements with no test coverage.
- Tests with no requirement linkage.
- Free-text references instead of IDs.
- Manual spreadsheets that drift immediately.

### Typical Workflow with arqix

Quinn runs trace scanning, creates coverage reports, and checks matrices for missing or broken links.
Results are shared as review notes, CI gates, or structured exports.

### Important arqix Capabilities and Commands

- `trace scan`
- `trace matrix`
- `trace coverage`
- `lint run` (including i18n profile when needed)
- JSON diagnostics for CI consumption

### artefacts They Care About

- Trace graph JSON
- Coverage reports
- Trace matrices (CSV and Markdown)
- CI outputs and quality dashboards

### Boundaries

Quinn does not define templates or CI pipelines, but relies on those systems to produce reliable quality signals.

### Open Needs

Quinn benefits from stable schemas for exports, filtering capabilities, and consistent diagnostics contracts.
