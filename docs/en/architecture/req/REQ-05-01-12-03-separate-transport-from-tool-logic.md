---
id: REQ-05-01-12-03
title: Separate Transport from Tool Logic
slug: separate-transport-from-tool-logic
iri: arqix:requirements/req-05-01-12-03

rdf:
  type:
    - arqix:classes/constraint

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-05-01-12
      - arqix:user-stories/us-08-01-12
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Tool implementations have no dependency on the transport layer.

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

The MCP transport handling SHALL remain separate from the tool logic.

### Notes

Derived from the acceptance criteria of US-05-01-12, US-08-01-12 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10). Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
