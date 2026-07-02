---
id: US-05-01-09
title: Observe Assembled Outputs for Downstream Tooling
slug: observe-assembled-outputs-for-downstream-tooling
iri: arqix:user-stories/us-05-01-09

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-05
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-00-00-00-01
      - arqix:requirements/req-00-00-00-06
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-05-01

properties:
  priority: medium
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


## Observe Assembled Outputs for Downstream Tooling

As an AIOps engineer, I want to assemble a doc package into pages, so that downstream tools can consume stable, deterministically produced outputs from modular source units.

### Acceptance Criteria

- [ ] `arqix assemble build <doc-package>` generates outputs under `pages/`.
- [ ] `strip_frontmatter_on_include` can be enabled via configuration.
- [ ] Include cycles are detected and fail with a clear error message.
- [ ] Output ordering is deterministic across repeated runs.

### Notes

The build flow is complete when a doc package with nested includes produces stable page outputs and cycles fail fast with a readable path trace. Add tests for frontmatter stripping on included content and for deterministic output ordering across repeated runs. The first implementation should optimise for clear diagnostics over aggressive assembly features. The main value for Alex is predictable assembled artefacts for indexing and automation.
