---
id: US-08-01-36
title: Validate the Ontology Against Itself
slug: validate-the-ontology-against-itself
iri: arqix:user-stories/us-08-01-36

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-08
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-08-01-36-01
      - arqix:requirements/req-08-01-36-02
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-08-01

properties:
  priority: high
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

## Validate the Ontology Against Itself

As a knowledge engineer, I want every declared edge checked against the properties' declared domains and ranges, so that a configured ontology is a validated contract instead of decoration.

### Acceptance Criteria

- [ ] A triple whose predicate declares `rdfs.domain` or `rdfs.range` is a finding (ONT-007) when the subject's or the resolvable object's types lie outside the declared classes, subclass closure included.
- [ ] Predicates without declarations, external objects, and untyped documents stay unchecked — declaration opts a property into the contract.
- [ ] A `sub-class-of` cycle longer than a class's own root self-reference is a finding (ONT-008).

### Notes

The declared domains and ranges existed as documentation; this makes them the machine-checked contract every layer of the ontology must satisfy (ADR-0017, the checker validates the configured ontology).
A class naming itself as `sub-class-of` marks a hierarchy root and stays legal.
