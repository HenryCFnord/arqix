---
id: REQ-04-01-18-02
title: Justify Diagram Relationships Against the Model
slug: justify-diagram-relationships-against-the-model
iri: arqix:requirements/req-04-01-18-02

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-04-01-18
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: A diagram relationship whose endpoints have no direct or implied model edge yields a finding; the current views, including the implied agent-to-cli container edge, yield none.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-13
  updated: 2026-07-13
  lang: en
  translation-of:
  generated: false
---

## Requirement

When `arqix lint run` checks a derived Mermaid view, arqix SHALL report a finding for each diagram relationship whose endpoints the model does not connect by a direct or implied edge.

### Notes

Derived from US-04-01-18.
A container view inherits Structurizr's implied relationships: a system-level edge is justified for a container of that system (`agent -> arqix` justifies `agent -> cli`), so the check accepts an edge to or from a container's enclosing system (ADR-0016).
