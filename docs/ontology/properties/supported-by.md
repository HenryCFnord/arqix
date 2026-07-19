---
id: property-supported-by
label: supported-by
iri: arqix:properties/supported-by

rdf:
  type:
    - rdf:Property

rdfs:
  range:
    - arqix:classes/source

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

## supported-by

Connects a document to a source record whose content supports a statement in the document's body.
The edge is derived: `fmt` lifts every `arqix:claim` body marker into the document's `derived-triples` section, so the anchor in the text and the edge in the graph cannot disagree while the gate is green (ADR-0018).
The declared range makes every target a source record; position-bound attributes — confidence, the locus inside the source — live on the marker, not on the edge.
