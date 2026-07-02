---
id: PER-04
title: Daria DevOps
slug: daria-devops
iri: arqix:personas/per-04

rdf:
  type:
    - arqix:classes/persona

triples: []

properties:
  role: CI/CD and publishing owner
  description: Owns reproducible builds, CI gates, and publishing workflows; values deterministic automation, clear exit codes, and identical behavior locally and in CI.

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2025-03-05
  updated: 2025-03-28
  lang: en
  translation-of:
  generated: false
---

## Daria DevOps

Daria owns CI and publishing. She values reproducibility, clear exit codes, and workflows that behave the same locally and in CI. If a tool cannot be automated, it will not scale.

### Goals

- Run arqix checks as CI gates for PRs.
- Build and publish documentation deterministically.
- Keep pipelines simple by reusing Taskfile workflows.
- Provide clear failure signals and diagnostics.

### Success Looks Like

- PR checks fail fast with actionable diagnostics.
- Main branch builds produce predictable artefacts.
- Publishing works for EN and DE sites (Zensical-first).
- Local workflows mirror CI workflows.

### Pain Points

- Non-deterministic outputs causing noisy diffs.
- External tool version drift (site builders, renderers).
- Unclear failure classification and exit codes.
- Pipeline logic that diverges from developer workflows.

### Typical Workflow with arqix

Daria integrates arqix into CI using Taskfile tasks. PR pipelines run formatting checks, lint, trace scan, and coverage. Main pipelines run the same gates and then build and publish artefacts.

### Important arqix Capabilities and Commands

- `fmt --check`
- `lint run`
- `trace scan`
- `trace coverage`
- `publish site --lang en|de` (Zensical-first)
- Taskfile workflows (`task ci:pr`, `task ci:main`)

#### artefacts They Care About

- Site build outputs in `doc/artefacts/site`
- Lint and trace reports
- Build logs and uploaded CI artefacts

#### Boundaries

Daria does not own content quality. She owns automation and reproducibility.

#### Open Needs

Daria benefits from stable CLI contracts, predictable artefact locations, and reliable third-party tool integration.
