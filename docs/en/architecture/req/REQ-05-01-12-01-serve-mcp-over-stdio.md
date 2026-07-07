---
id: REQ-05-01-12-01
title: Serve MCP over Stdio
slug: serve-mcp-over-stdio
iri: arqix:requirements/req-05-01-12-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-05-01-12
      - arqix:user-stories/us-08-01-12
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: An MCP client can start the server over stdio, discover the declared tools, and execute them successfully.

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-02
  updated: 2026-07-02
  lang: en
  translation-of:
  generated: false
---

## Requirement

When `arqix mcp serve` runs, arqix SHALL serve MCP over stdio transport.

### Notes

Derived from the acceptance criteria of US-05-01-12, US-08-01-12 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10).
Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
