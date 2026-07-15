---
id: WF-04-01
title: Run CI Gates and Publish artefacts
slug: run-ci-gates-and-publish-artefacts
iri: arqix:workflows/wf-04-01

rdf:
  type:
    - arqix:classes/workflow

triples:
  - predicate: arqix:properties/has-primary-persona
    object: arqix:personas/per-09
  - predicate: arqix:properties/has-relevant-persona
    object:

properties:
  goal: Provide CI gates for quality and publish multi-lingual site outputs.
  entry-state: Local workflows need to match CI behavior, with deterministic checks for PRs and publishable outputs on main.
  end-state: CI status is reproducible and actionable, and multiple language-specific site artefacts are produced and optionally deployed.

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-03-25
  updated: 2026-07-15
  lang: en
  translation-of:
  generated: false
---

## Run CI Gates and Publish artefacts

### Context

Local workflows must match CI behavior.
The pipeline should run deterministic checks for PRs and produce publishable outputs on main.

### Goal

Provide CI gates for quality, and publish bilingual site outputs (Zensical-first).

### Steps

1. PR pipeline: 2) Run formatting in check mode.
   3) Run lint rules and i18n lint.
   4) Run trace scan and coverage.
2. Main pipeline: 6) Run PR gates.
   7) Build sites for EN and DE via Zensical.
   8) Upload artefacts and/or deploy to GitHub Pages.

### Outputs

- CI status (pass/fail) with actionable diagnostics
- Site artefacts for EN and DE
- Optional deployment to GitHub Pages (EN root, DE under `/de`)

### Failure Modes

- External tool not available (Zensical, renderer).
- Non-deterministic outputs causing CI noise.
- Missing translation or drift fails i18n checks (if enforced).

### Related Commands

- `arqix fmt --check`
- `arqix lint run`
- `arqix trace scan`
- `arqix trace coverage`
- `arqix publish site --lang en|de`

### Automation

- CI: `.github/workflows/ci.yml` runs the daily gate (`scripts/arqix verify`) and the Rust lints on every PR and push to main (the oracle conformance cross-check retired with the Python oracles, 2026-07-15)
- just: `just ci` mirrors the pipeline locally
