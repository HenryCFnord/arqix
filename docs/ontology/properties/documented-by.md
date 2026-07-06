---
id: property-documented-by
label: documented-by
iri: arqix:properties/documented-by

rdf:
  type:
    - rdf:Property

rdfs:
  domain:
    - arqix:classes/artefact
  range:
    - arqix:classes/unit

owl:
  inverse-of: arqix:properties/documents-artefact

triples: []

properties: {}

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-06
  updated: 2026-07-06
  lang: en
  generated: false
---

## Documented-by

Relates an artefact — typically a `code-artefact` — to a unit that documents
it. Inverse of `documents-artefact`. Materialised from a code-side
`// arqix:documented-by <unit-iri>` marker (ADR-0009), so the code→documentation
link is traversable from the code side (report question Q-08).
