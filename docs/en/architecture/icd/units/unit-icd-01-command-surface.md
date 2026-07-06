---
id: unit-icd-01
title: Command Surface
slug: command-surface
iri: arqix:units/unit-icd-01

rdf:
  type:
    - arqix:classes/unit

triples: []

properties:
  section-kind: icd-command-surface

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-06
  updated: 2026-07-06
  lang: en
  translation-of:
  generated: false
---

## Command Surface

The interface a caller drives is the command tree. It follows the noun–verb
scheme decided in ADR-0005 (`config`, `doc`, `unit`, `fmt`, `finalise`,
`lint`, `assemble`, `trace`, `report`, `publish`, `render`, `policy`,
`verify`, `mcp`), with `verify` as the one top-level exception. The
normative command-ownership map is the arc42 chapter-5 table; this section
is the machine-interface view of the same surface, defined by the clap
tree in `src/main.rs`.
<!-- arqix:references-artefact arqix:adrs/adr-0005 -->

Every command accepts the global `--format {text,json}` flag (default
`text`); `--format json` is the contract for machine consumers. Only
commands whose story has shipped return a stable result; the rest are
stubs that exit `70` (see the exit-code contract).

The exhaustive, always-current per-command reference (every subcommand,
flag, and argument) is a generated projection of the clap tree — report
question **Q-11**, produced by the single generator surface decided in
ADR-0009 — and is composed into the ICD here once that generator ships.
Until then, `arqix <command> --help` is the authoritative live surface.
