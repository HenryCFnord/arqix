---
id: REQ-04-01-18-01
title: Render Views from the Model
slug: render-views-from-the-model
iri: arqix:requirements/req-04-01-18-01

rdf:
  type:
    - arqix:classes/quality-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-04-01-18
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: The arc42 chapters embed images generated from workspace.dsl under model/generated/, not hand-authored C4 Mermaid blocks; a reviewer can confirm each embedded view has a generated source.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-13
  updated: 2026-07-13
  lang: en
  translation-of:
  generated: false
---

## Requirement

The published architecture views SHALL be rendered from the C4 model rather than hand-authored.

### Notes

Derived from US-04-01-18.
Rendering runs through a containerised toolchain (Kroki) via `just` and CI, orchestrated like the other external renderers (Pandoc, the site command); the model stays the single source of truth (ADR-0002, ADR-0016).
