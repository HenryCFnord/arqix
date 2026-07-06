---
id: unit-icd-06
title: Forward Contracts
slug: forward-contracts
iri: arqix:units/unit-icd-06

rdf:
  type:
    - arqix:classes/unit

triples:
  - predicate: arqix:properties/references-artefact
    object:
      - arqix:adrs/adr-0006

properties:
  section-kind: icd-forward-contracts

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

## Forward Contracts

Two interfaces are named in the command surface but not yet implemented —
their commands are stubs that exit `70`. Their wire contract is authored
here **ahead of the code**, so the shape is fixed before it ships and
consumers can build against it. This is deliberate: an interface designed
after its callers appear drifts.

### `report bundle`

Produces an evidence bundle for audit and compliance. The intended
contract: a directory (or archive) of the generated question units and
trace matrices plus a top-level `manifest.json` listing each artefact with
its path, `sha256`, and the snapshot commit — a deterministic,
self-describing evidence set. Fixed fields and layout are settled with the
report family's story before the command leaves stub state.

### `mcp serve`

Exposes search/read/list over the Model Context Protocol on stdio, so an
agent host can consume arqix as an MCP server. The intended contract: the
tool set (at least `search`, `read`, `list`) with their parameter and
result schemas mapping onto the same `doc search`/`read`/`list` wire shapes
(see Wire Schemas), carrying the same `schema_version` discipline. The
method set is fixed with the MCP story before the command leaves stub
state.

Until then, both commands honour the exit-code contract by returning `70`,
never a `0/1/2` result that a caller could mistake for real output.
