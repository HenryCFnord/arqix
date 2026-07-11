---
id: REQ-01-01-09-05
title: Document Agent-Specific Extension Points
slug: document-agent-specific-extension-points
iri: arqix:requirements/req-01-01-09-05

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-09
      - arqix:user-stories/us-08-01-18
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Each supported coding agent has a documented list of its extension points (for example skills or prompt libraries) and the files they live in.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-02
  updated: 2026-07-10
  lang: en
  translation-of:
  generated: false
---

## Requirement

When a coding agent is supported, its agent-specific extension points SHALL be documented.

### Notes

Derived from the acceptance criteria of US-01-01-09 after the agent-agnostic generalisation (see ADR-0001).
The concrete file mapping per agent lives in the ADR, not in this requirement.
