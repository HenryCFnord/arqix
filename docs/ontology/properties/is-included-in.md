---
id: property-is-included-in
label: is-included-in
iri: arqix:properties/is-included-in

rdf:
  type:
    - rdf:Property

rdfs:
  domain:
    - arqix:classes/unit
  range:
    - arqix:classes/document-page

owl:
  inverse-of: arqix:properties/includes-unit

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

## Is-included-in

Relates a unit to a document page that includes it.
Inverse of `includes-unit`.
