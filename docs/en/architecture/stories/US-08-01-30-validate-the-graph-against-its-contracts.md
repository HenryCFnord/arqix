---
id: US-08-01-30
title: Validate the Graph Against Its Contracts
slug: validate-the-graph-against-its-contracts
iri: arqix:user-stories/us-08-01-30

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-08
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-08-01-30-01
      - arqix:requirements/req-08-01-30-02
      - arqix:requirements/req-08-01-30-03
      - arqix:requirements/req-08-01-30-04
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

## Validate the Graph Against Its Contracts

As a knowledge engineer, I want every declared edge and placement checked against the ontology's contracts, so that the graph is validated data instead of decoration.

### Acceptance Criteria

- [ ] A triple whose object is an `arqix:` IRI carried by no corpus document is a `lint frontmatter` finding naming the IRI and the referencing document.
- [ ] Objects outside the `arqix:` namespace are not resolved; external vocabularies stay usable as opaque references.
- [ ] Body reference markers keep their own resolution (LNT-003); this check covers the frontmatter graph.
- [ ] A triple whose predicate declares `rdfs.domain` or `rdfs.range` is an ONT-007 finding when the subject's or the resolvable object's types lie outside the declared classes, subclass closure included.
- [ ] A `sub-class-of` cycle longer than a class's own root self-reference is an ONT-008 finding.
- [ ] With a declared `dir-template`, a document whose parent directory does not equal the template rendered from its own properties is an FM-010 finding.

### Notes

The graph's value is its edges; an edge into nothing is the one error class no gate reports today (ADR-0017, the checker validates the configured ontology).
A second arqix-governed corpus reported the gap by injecting a bogus reference that `verify` accepted (FR-A1 in `docs/en/plans/knowledge-repository-2026-07-15/FEEDBACK-2026-07-17-psi.md`); the resolution rule turned out to exist (ONT-003) but reaches only documents inside a configured `[kinds.<family>].dir`, so the reproduction points at scanning scope, not at resolution.
