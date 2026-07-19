---
id: class-entity
label: entity
iri: arqix:classes/entity

rdf:
  type:
    - rdfs:Class

rdfs:
  sub-class-of:
    - arqix:classes/knowledge-artefact

triples: []

properties: {}

external-references: []

owl: {}

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-19
  updated: 2026-07-19
  lang: en
  translation-of:
  generated: false
---

## Entity

A domain entity as a first-class corpus document (ADR-0022): the thing several descriptions may share — a term, a concept, a standard's subject — separated from any single document that describes it.
Because the entity is itself a document, the one node-identity rule (ADR-0007) is untouched: the entity node is a document node.
Descriptions attach through `arqix:properties/describes`; mappings onto external standards live canonically on the entity through the mapping properties.
Entities are opt-in — a corpus without entity documents loses nothing.
