---
id: US-04-01-06
title: Build Deterministic Page Artefacts in CI
slug: build-deterministic-page-artefacts-in-ci
iri: arqix:user-stories/us-04-01-06

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-04
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-00-00-00-01
      - arqix:requirements/req-00-00-00-06
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-04-01

properties:
  priority: high
  edge-case: false

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-03-30
  updated: 2026-07-02
  lang: en
  translation-of:
  generated: false
---


## Build Deterministic Page Artefacts in CI

As a DevOps engineer, I want to assemble a doc package into pages, so that stable publishable artefacts can be produced automatically in CI.

### Acceptance Criteria

- [ ] `arqix assemble build <doc-package>` generates outputs under `pages/`.
- [ ] `strip_frontmatter_on_include` can be enabled via configuration.
- [ ] Include cycles are detected and fail with a clear error message.
- [ ] Output ordering is deterministic across repeated runs.

### Notes

The build flow is complete when a doc package with nested includes produces stable page outputs and cycles fail fast with a readable path trace. Add tests for frontmatter stripping on included content and for deterministic output ordering across repeated runs. The first implementation should optimise for clear diagnostics over aggressive assembly features. The main value for Daria is reproducible build outputs and clear failure signals.
