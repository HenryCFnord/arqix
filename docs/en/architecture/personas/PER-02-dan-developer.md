---
id: PER-02
title: Dan Developer
slug: dan-developer
iri: arqix:personas/per-02

rdf:
  type:
    - arqix:classes/persona

triples: []

properties:
  role: developer and documentation contributor
  description: Writes code and documentation in the same flow. Values speed, low-friction tooling, and predictable local checks that match CI.

external-references: []

meta:
  lifecycle-status: retired
  owner: hcf
  created: 2026-03-05
  updated: 2026-07-12
  lang: en
  translation-of:
  generated: false
---

## Dan Developer

Dan is a developer who writes code and documentation in the same flow.
He values speed and low-friction tooling.
If documentation feels like a separate project, it will not happen reliably.

### Goals

- Create documentation quickly using templates.
- Keep code, tests, and docs linked via stable IDs.
- Validate changes locally before opening a PR.
- Avoid time spent on formatting debates.

### Success Looks Like

- New docs are created with one command and correct structure.
- Includes and assembling work without manual stitching.
- Local checks predict CI results.
- PR feedback is about content, not formatting.

### Pain Points

- Copy-pasting old docs as “templates”.
- Broken includes discovered late.
- Missing IDs and inconsistent metadata.
- Unclear rules for what must be documented.

### Typical Workflow with arqix

Dan creates or updates docs while implementing a feature.
He uses assemble to preview pages, runs formatting and linting before committing, and uses IDs to connect requirements, code, and tests.

### Important arqix Capabilities and Commands

- `doc new`
- `assemble build`
- `fmt`
- `lint run`
- `trace scan` and `trace coverage` (when required)
- just recipes for local “CI dry runs” (`just ci`)

### artefacts They Care About

- Feature documentation pages
- Requirements, user stories, and ADR links
- Assemble outputs and logs
- Lint and trace diagnostics

### Boundaries

Dan does not own the global rules and templates.
He follows the conventions defined by maintainers and expects them to be predictable.

### Open Needs

Dan benefits from fast search and read functions, clear diagnostics, and a simple local workflow that matches CI.

Retired in the persona merge of 2026-07-12: this viewpoint is carried by PER-09 (Builder), which consolidates the user-side personas while the maintainer and the coding agent stay dedicated.
