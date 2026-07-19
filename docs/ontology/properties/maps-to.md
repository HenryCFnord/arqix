---
id: property-maps-to
label: maps-to
iri: arqix:properties/maps-to

rdf:
  type:
    - rdf:Property

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

## maps-to

The general mapping edge from a corpus document onto an external standard's concept (ADR-0022).
Use a refinement — exact-match, close-match, broader-match, narrower-match — when the correspondence is known; use maps-to when only the association is.
The target is the external concept's IRI; domain and range stay undeclared, so any document may map and the external target stays outside the corpus graph.
