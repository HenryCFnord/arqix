---
id: property-describes
label: describes
iri: arqix:properties/describes

rdf:
  type:
    - rdf:Property

rdfs:
  range:
    - arqix:classes/entity

triples: []

properties: {}

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-19
  updated: 2026-07-19
  lang: en
  translation-of:
  generated: false
---

## describes

Connects a describing document to the domain entity it describes (ADR-0022).
Several documents may describe one entity — the per-context descriptions of a shared term are the driving case.
The declared range makes every target an entity document; the domain stays undeclared, so any document kind may describe.
