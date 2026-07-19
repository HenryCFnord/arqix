---
id: US-08-01-37
title: Project the Source Catalog
slug: project-the-source-catalog
iri: arqix:user-stories/us-08-01-37

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-08
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-08-01-28-05
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-08-01

properties:
  priority: medium
  edge-case: false

external-references: []

meta:
  lifecycle-status: retired
  owner: hcf
  created: 2026-07-19
  updated: 2026-07-19
  lang: en
  translation-of:
  generated: false
---

## Project the Source Catalog

As a knowledge engineer, I want the source records projected into one deterministic catalog table, so that the corpus's provenance inventory is a generated artifact under the drift gate instead of a hand-maintained file.

### Acceptance Criteria

- [ ] `arqix report snapshot` renders `source-catalog.md`: one row per document of `arqix:classes/source`, sorted by id, with id, title, uri, accessed, licence, and the local-copy state projected from the frontmatter.
- [ ] The unit sits under the existing snapshot freshness gate, so a record change without regeneration is drift.
- [ ] A corpus without source records renders the empty table, not an error.

### Notes

The tenth question unit (ADR-0008): one unit answers one named question, and the catalog is a projection of frontmatter, never hand-edited.
A second arqix-governed corpus maintains exactly this table by hand-rolled generation plus its own drift check (FR-A3 in the second intake of `docs/en/plans/knowledge-repository-2026-07-15/`).
