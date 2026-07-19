---
id: US-08-01-38
title: Hold Documents to Their Kinds Placement Contract
slug: hold-documents-to-their-kinds-placement-contract
iri: arqix:user-stories/us-08-01-38

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-08
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-08-01-38-01
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

## Hold Documents to Their Kinds Placement Contract

As a repository owner, I want every document checked against its kind's declared placement, so that a document's path and its own frontmatter can never silently disagree.

### Acceptance Criteria

- [ ] When a kind declares a `dir-template`, `lint frontmatter` reports every document of the family whose parent directory does not equal the template rendered from the document's own `properties`, slug, and kind (FM-010).
- [ ] A document missing a property the template names is the same finding — the binding is broken either way.
- [ ] Kinds without a `dir-template` stay unchecked; the kind's `dir` remains the family's walk root.

### Notes

The inverse of the creation contract: `doc new` renders the `dir-template` to place a document, and the checker renders it from the document to verify the place — one declared source, checked in both directions.
A second arqix-governed corpus names the drift this closes: a `context` field and a `contexts/<ctx>/` path that can disagree with nothing to catch it (FR-C3 in the second intake of `docs/en/plans/knowledge-repository-2026-07-15/`).
