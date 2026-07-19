---
id: REQ-08-01-43-01
title: Ship the Entity and Mapping Vocabulary
slug: ship-the-entity-and-mapping-vocabulary
iri: arqix:requirements/req-08-01-43-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-08-01-43
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: In a corpus with the knowledge-base module effective, triples with the mapping predicates and rdf.type arqix:classes/entity pass the vocabulary checks; with the module deselected the same declarations are ONT-001/ONT-002 findings.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-19
  updated: 2026-07-19
  lang: en
  translation-of:
  generated: false
---

## Requirement

Where the knowledge-base module is effective, arqix SHALL define the entity class, the describes property, and the mapping properties in the effective ontology.

### Notes

The vocabulary ships as module data (ADR-0021, ADR-0022): the entity class with `describes` ranging over it, and `maps-to` with its four refinements targeting external IRIs.
Derived from US-08-01-43.
