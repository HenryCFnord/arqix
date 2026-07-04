---
id: unit-arc42-07
title: Deployment View
slug: deployment-view
iri: arqix:units/unit-arc42-07

rdf:
  type:
    - arqix:classes/unit

triples: []

properties:
  section-kind: arc42-chapter

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-03
  updated: 2026-07-03
  lang: en
  translation-of:
  generated: false
---

## Deployment View

TODO — to be filled when distribution is set up.

Intended shape: a single statically-built binary installed via `cargo install` (later: release artefacts per platform); no runtime dependencies except the optional external render toolchain; CI consumes the same binary as developers (REQ-00-00-00-02 keeps behaviour identical across environments).
