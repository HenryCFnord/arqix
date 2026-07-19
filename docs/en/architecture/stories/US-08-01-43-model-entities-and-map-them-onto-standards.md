---
id: US-08-01-43
title: Model Entities and Map Them Onto Standards
slug: model-entities-and-map-them-onto-standards
iri: arqix:user-stories/us-08-01-43

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-08
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-08-01-43-01
      - arqix:requirements/req-08-01-43-02
      - arqix:requirements/req-08-01-43-03
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-08-01

properties:
  priority: medium
  edge-case: false

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

## Model Entities and Map Them Onto Standards

As a knowledge engineer, I want domain entities as first-class documents and their mappings onto external standards as validated edges, so that several descriptions share one identity and the corpus's crosswalk is governed, queryable, and reportable.

### Acceptance Criteria

- [ ] With the knowledge-base module effective, `arqix:classes/entity`, `arqix:properties/describes` (range: entity), and the mapping properties (`maps-to`, `exact-match`, `close-match`, `broader-match`, `narrower-match`) are defined vocabulary; without the module they are not.
- [ ] A declared triple whose predicate lies outside the arqix namespace and outside `[frontmatter].allowed-external-properties` is ONT-010; the shipped default list is empty.
- [ ] The crosswalk unit renders one row per mapping edge — mapping document, mapping property, external target — grouped by the target's namespace, under the snapshot drift gate.
- [ ] An entity kind contract (directory, template, id pattern) is ordinary project configuration, exactly like source and claim records.

### Notes

The entity is a document, so the node-identity rule (ADR-0007) is untouched; the design is fixed in ADR-0022.
