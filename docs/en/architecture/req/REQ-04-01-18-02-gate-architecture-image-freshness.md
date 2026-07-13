---
id: REQ-04-01-18-02
title: Gate Architecture-Image Freshness
slug: gate-architecture-image-freshness
iri: arqix:requirements/req-04-01-18-02

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
  fit-criterion: Regenerating the architecture images from workspace.dsl and finding any difference against the committed images fails CI; regenerating an unchanged model leaves them byte-identical and passes.

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

When a committed architecture-view image differs from a fresh render of the model, the verification gate SHALL fail.

### Notes

Derived from US-04-01-18.
This is the drift guarantee the withdrawn derivation checker used to provide, moved to the regenerate-and-diff freshness gate already used for the report snapshots (ADR-0016); it runs in CI and locally via `just`, where a container runtime is available.
