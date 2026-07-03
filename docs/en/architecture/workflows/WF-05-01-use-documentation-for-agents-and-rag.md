---
id: WF-05-01
title: Use Documentation for Agents and RAG
slug: use-documentation-for-agents-and-rag
iri: arqix:workflows/wf-05-01

rdf:
  type:
    - arqix:classes/workflow

triples:
  - predicate: arqix:properties/has-primary-persona
    object: arqix:personas/per-05
  - predicate: arqix:properties/has-relevant-persona
    object:

properties:
  goal: Make documentation reliably searchable and citeable by tools, with deterministic outputs and stable metadata contracts.
  entry-state: Agents and RAG pipelines depend on documentation, but the docs may not yet fully enforce stable structure, IDs, metadata standards, and translation checks.
  end-state: Documentation is reliably searchable and citeable by tools, with deterministic outputs, stable IDs, linked translations, and structured access for automation.

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-03-25
  updated: 2026-07-02
  lang: en
  translation-of:
  generated: false
---

## Use Documentation for Agents and RAG

Agents and RAG pipelines depend on stable structure, IDs, and machine-readable access to documents.

### Goal

Make documentation reliably searchable and citeable by tools, with deterministic outputs and stable

metadata contracts.

### Steps

1. Ensure docs follow templates and metadata standards (EN source).
2. Ensure translations are linked and checked (i18n lint where required).
3. Produce trace outputs that can feed downstream analysis.
4. Provide structured access patterns (CLI now; MCP later).

### Outputs

- Deterministic, ID-addressable docs
- Trace graph exports for knowledge graphs
- A predictable interface for automation

### Failure Modes

- Metadata drift and inconsistent IDs.
- Weak or missing diagnostics contract.
- Mixed-language trees without clear source/translation rules.

### Related Commands

- `arqix doc list`
- `arqix doc read`
- `arqix doc search`
- `arqix trace scan`
- `arqix lint run --profile i18n`

### Automation

- `arqix mcp serve`
