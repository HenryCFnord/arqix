---
id: US-04-01-18
title: Render Architecture Views from the Model
slug: render-architecture-views
iri: arqix:user-stories/us-04-01-18

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-09
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-04-01-18-01
      - arqix:requirements/req-04-01-18-02
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-04-01

properties:
  priority: medium
  edge-case: false

external-references: []

meta:
  lifecycle-status: specified
  owner: hcf
  created: 2026-07-13
  updated: 2026-07-13
  lang: en
  translation-of:
  generated: false
---

## Render Architecture Views from the Model

As a builder, I want the C4 views generated from `workspace.dsl` as committed images and embedded in the docs, so that a view cannot drift from the model — there is no hand-authored diagram left to drift.

### Acceptance Criteria

- [x] The architecture views are rendered from `workspace.dsl` to SVG by a containerised renderer (Kroki), run identically through `just` and in CI.
- [x] The generated SVGs are committed under `docs/en/architecture/model/generated/` and embedded in the arc42 chapters, replacing the hand-authored C4 Mermaid.
- [ ] CI fails when a committed image is stale against a fresh render of the model — the same regenerate-and-diff freshness gate used for the report snapshots.
- [x] Mermaid remains available for non-C4 diagrams; only the C4 views become generated images.

### Notes

This supersedes the in-process derivation checker (the earlier take on item 8): once the views are generated from the single source, there is no hand-authored diagram to validate, so the checker is withdrawn and the drift guarantee moves to the render-freshness gate (ADR-0016).
structurizr-cli and Kroki emit a flowchart Mermaid dialect rather than the `C4Context` dialect the diagrams used, so the views become rendered images, not regenerated Mermaid.
Rendering needs a container runtime, so the generation and the freshness gate run in CI and locally via `just` with Docker; they are not part of the offline `arqix verify` binary.
