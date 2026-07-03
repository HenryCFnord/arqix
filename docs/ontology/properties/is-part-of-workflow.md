---
id: property-is-part-of-workflow
label: is-part-of-workflow
iri: arqix:properties/is-part-of-workflow

rdf:
  type:
    - rdf:Property

rdfs:
  domain:
    - arqix:classes/user-story
  range:
    - arqix:classes/workflow

owl:
  inverse-of: arqix:properties/has-story

triples: []

properties: {}

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-02
  updated: 2026-07-02
  lang: en
  generated: false
---

## Is-part-of-workflow

Relates a user story to the workflow it belongs to. Inverse of `has-story`.
