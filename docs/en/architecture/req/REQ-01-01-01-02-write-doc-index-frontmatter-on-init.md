---
id: REQ-01-01-01-02
title: Write Doc-Index Frontmatter on Init
slug: write-doc-index-frontmatter-on-init
iri: arqix:requirements/req-01-01-01-02

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-01
      - arqix:user-stories/us-02-01-01
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: The generated `index.md` contains valid frontmatter with `id`, `kind=doc_index`, and `title`.

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-02
  updated: 2026-07-02
  lang: en
  translation-of:
  generated: false
---

## Requirement

When a doc package is initialised, arqix SHALL write `index.md` frontmatter containing `id`, `kind=doc_index`, and `title`.

### Notes

Derived from the acceptance criteria of US-01-01-01 during the group-01 pilot derivation (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`). Cross-cutting behaviour is linked via the story's `has-requirement`, not restated here.
