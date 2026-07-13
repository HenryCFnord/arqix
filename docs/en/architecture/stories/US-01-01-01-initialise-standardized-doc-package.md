---
id: US-01-01-01
title: Initialise Standardised Doc Package
slug: initialise-standardized-doc-package
iri: arqix:user-stories/us-01-01-01

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-01
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-00-00-00-04
      - arqix:requirements/req-00-00-00-06
      - arqix:requirements/req-00-00-00-08
      - arqix:requirements/req-01-01-01-01
      - arqix:requirements/req-01-01-01-02
      - arqix:requirements/req-01-01-01-03
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
  updated: 2026-07-02
  lang: en
  translation-of:
  generated: false
---

## Initialise Standardised Doc Package

As a maintainer, I want to initialise a new doc package with a standardised directory structure, so that I can build documentation consistently and reproducibly.

### Acceptance Criteria

- [x] `arqix doc init <path>` creates `index.md`, `units/`, `pages/`, `artefacts/`, `logs/`, and `.arqix/`.
- [x] `index.md` contains frontmatter with `id`, `kind=doc_index`, and `title`.
- [x] `id`/`slug` are derived deterministically from `title` based on configurable slug rules.
- [x] Existing files are not overwritten without explicit approval.
- [x] `arqix init` behaves exactly like `arqix doc init` for the same arguments.

### Notes

Acceptance is met when initialisation works in an empty target path and produces the expected scaffold without manual cleanup.
Add tests for deterministic `id` and `slug` generation from the same title input and for the refusal path when files already exist.
Document any prompt or force behaviour explicitly if overwrite protection is configurable.
