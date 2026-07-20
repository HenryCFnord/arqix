---
id: unit-icd-04
title: Wire Schemas
slug: wire-schemas
iri: arqix:units/unit-icd-04

rdf:
  type:
    - arqix:classes/unit

triples: []

properties:
  section-kind: icd-wire-schemas

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

## Wire Schemas

<!-- arqix:references-artefact arqix:adrs/adr-0006 -->
<!-- arqix:references-artefact arqix:requirements/req-04-01-10-01 -->
The machine-readable outputs a consumer parses under `--format json`.
Every object key is sorted (the JSON is emitted from a sorted map), so byte output is stable for a given input.

- **`doc list`** — `{"schema_version", "documents": [{"id", "title", "kind", "file", "lang"}]}`.
- **`doc read`** — `{"schema_version", "id", "title", "iri", "kind", "lang", "file", "body"}`.
- **`doc search`** — `{"schema_version", "query", "hits": [{"id", "file", "line"}]}`.
- **`doc query`** — `{"schema_version", "documents": [{"id", "title", "kind", "file", "lang", "lifecycle", "edges": [{"predicate", "object"}]}]}`; conjunctive filters, the MCP `query` tool answers identically (ADR-0023).
- **`trace scan`** — the graph: `{"schema_version", "nodes": [...], "edges": [...]}` (ADR-0006 layer 1).
- **`trace coverage`** — coverage by requirement kind with `TRC-*` diagnostics.
- **`trace matrix`** — CSV, not JSON (ADR-0006 layer 3).
- **`verify`** — `{"schema_version", "steps": [{"step", "exit_code", "ok", "informational", "skipped"}], "ok"}`; `informational` marks a step whose findings do not gate, and `skipped` marks a step the profile declared but the context did not run (report-freshness under a non-gating snapshot strategy).
- **`assemble build`** — the assembly log `pages/assembly.jsonl`, one JSON object per line with `doc`, `chapter_id`, `out`, `include`, `sha256`, `bytes`, `at_line`.

**`schema_version` axis (ADR-0009):** each interface owns its own version rather than one global number; every contract above currently sits at `1`.
The assembly log is the one output that does not yet carry a `schema_version` field — a follow-up adds it.
When the diagnostic and result payloads move to typed serde DTOs, these shapes become a generated JSON-Schema fragment instead of this authored prose.
