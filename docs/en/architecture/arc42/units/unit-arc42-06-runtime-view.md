---
id: unit-arc42-06
title: Runtime View
slug: runtime-view
iri: arqix:units/unit-arc42-06

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

## Runtime View

TODO — to be filled during implementation, one scenario per story batch.

Candidate scenarios, in implementation order:

- `arqix verify` — the one-command loop: config resolution → fmt check → lint → trace scan → coverage, fail-fast vs aggregate (REQ-04-01-05-*)
- `arqix assemble build` — directive parsing, include resolution, cycle detection, JSONL logging (REQ-02-01-11-*, REQ-04-01-01-*)
- `arqix doc new` — template instantiation with deterministic ID/routing and dry-run (REQ-01-01-13-*)
- `arqix publish site --lang` — per-language root resolution and toolchain orchestration with error forwarding (REQ-04-01-07-*)
