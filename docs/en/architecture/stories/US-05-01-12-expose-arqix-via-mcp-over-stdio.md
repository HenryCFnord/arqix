---
id: US-05-01-12
title: Expose Arqix via MCP over STDIO
slug: expose-arqix-via-mcp-over-stdio
iri: arqix:user-stories/us-05-01-12

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-05
  - predicate: arqix:properties/has-requirement
    object:
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-05-01

properties:
  priority: high
  edge-case: false

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-03-30
  updated: 2026-04-04
  lang: en
  translation-of:
  generated: false
---


## Expose Arqix via MCP over STDIO

As an AIOps engineer, I want to expose arqix via MCP over stdio, so that agents can use standardized tools to access documentation.

### Acceptance Criteria

- [ ] `arqix mcp serve` supports stdio transport.
- [ ] The MCP server exposes at least the tools `search`, `read`, and `list`.
- [ ] An MCP client can start the server over stdio, discover the declared tools, and execute the supported tools successfully.
- [ ] Transport handling remains separate from tool logic.

### Notes

Acceptance should prove that an MCP client can start the server over stdio, discover the declared tools, and execute `search`, `read`, and `list` successfully. Add an integration test or protocol fixture that validates request and response structure rather than only unit-testing handlers. Keep transport concerns separate from tool logic so the CLI and MCP layers can evolve independently. This is a core integration capability for agent tooling.
