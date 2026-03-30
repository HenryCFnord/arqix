---
id: US-01-01-01
title: Initialise Standardised Doc Package
slug: initialise-standardized-doc-package
iri: arqix:stories/us-01-01-01

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-01
  - predicate: arqix:properties/has-requirement
    object:
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-01-01

properties:
  priority: high
  edge-case: false

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-02-22
  updated: 2026-03-27
  lang: en
  translation-of:
  generated: false
---

## Initialise Standardised Doc Package

As a maintainer, I want to initialise a new doc package with a standardised directory structure, so that I can build documentation consistently and reproducibly.

### Acceptance Criteria

- [ ] `arqix doc init <path>` creates `index.md`, `units/`, `pages/`, `artefacts/`, `logs/`, and `.arqix/`.
- [ ] `index.md` contains frontmatter with `id`, `kind=doc_index`, and `title`.
- [ ] `id`/`slug` are derived deterministically from `title` based on configurable slug rules.
- [ ] Existing files are not overwritten without explicit approval.

### Notes

Acceptance is met when initialisation works in an empty target path and produces the expected scaffold without manual cleanup. Add tests for deterministic `id` and `slug` generation from the same title input and for the refusal path when files already exist. Document any prompt or force behaviour explicitly if overwrite protection is configurable.
