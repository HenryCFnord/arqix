---


id: US-08-01-12
title: Use MCP Tools Deterministically in Agent Workflows
slug: use-mcp-tools-deterministically-in-agent-workflows
iri: arqix:user-stories/us-08-01-12

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-08
  - predicate: arqix:properties/has-requirement
    object:
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-08-01

properties:
  priority: medium
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


## Use MCP Tools Deterministically in Agent Workflows

As a Casey Coding Agent, I want arqix to expose MCP tools over stdio, so that I can use standardized search, read, and list operations in automation workflows without custom integration code.

### Acceptance Criteria

- [ ] `arqix mcp serve` supports stdio transport.
- [ ] The MCP server exposes at least the tools `search`, `read`, and `list`.
- [ ] An MCP client can start the server over stdio, discover the declared tools, and execute the supported tools successfully.
- [ ] Transport handling remains separate from tool logic.

### Notes

Acceptance should prove that an MCP client can start the server over stdio, discover the declared tools, and execute `search`, `read`, and `list` successfully. Add an integration test or protocol fixture that validates request and response structure rather than only unit-testing handlers. Keep transport concerns separate from tool logic so the CLI and MCP layers can evolve independently. The main value for Casey is deterministic tool access through a standard protocol.
