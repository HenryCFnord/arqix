---
id: REQ-01-01-01-03
title: Provide a Top-Level Init Alias
slug: provide-a-top-level-init-alias
iri: arqix:requirements/req-01-01-01-03

rdf:
  type:
    - arqix:classes/quality-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-01
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: The alias `arqix init` exists and produces the same scaffold as `arqix doc init` for the same arguments.

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

The arqix CLI SHOULD provide a top-level `init` alias that behaves exactly like `doc init`.

### Notes

Repository initialisation writes root-level artefacts (`arqix.toml`, `AGENTS.md`) as well as the doc package, so `arqix init` is the discoverable entry point a newcomer reaches for; it mirrors `doc init` through the same code path, exactly as `req new`/`us new`/`adr new` mirror `doc new <kind>` (REQ-01-01-05-02).
Cross-cutting behaviour is linked via the story's `has-requirement`, not restated here.
