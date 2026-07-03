---
id: property-is-requirement-for-story
label: is-requirement-for-story
iri: arqix:properties/is-requirement-for-story

rdf:
  type:
    - rdf:Property

rdfs:
  domain:
    - arqix:classes/requirement
  range:
    - arqix:classes/user-story

owl:
  inverse-of: arqix:properties/has-requirement

triples: []

properties: {}

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-03
  updated: 2026-07-03
  lang: en
  generated: false
---

## Is-requirement-for-story

Relates a requirement to a user story that demands it. Inverse of `has-requirement`.
