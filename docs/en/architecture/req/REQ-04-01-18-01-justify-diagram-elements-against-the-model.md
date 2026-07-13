---
id: REQ-04-01-18-01
title: Justify Diagram Elements Against the Model
slug: justify-diagram-elements-against-the-model
iri: arqix:requirements/req-04-01-18-01

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
  fit-criterion: A derived Mermaid view that names an element absent from the referenced model view yields one finding per unjustified element; the two current views yield none.

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

When `arqix lint run` checks a Mermaid view marked as derived from the C4 model, arqix SHALL report a finding for each diagram element the referenced model view does not define.

### Notes

Derived from US-04-01-18.
Elements are matched to the model by display name and kind, tolerating the diagrams' shortened ids (ADR-0016); the `External` tag maps to `System_Ext` and container boundaries to the enclosing system.
