---
id: REQ-00-00-00-05
title: Template-Governed Document Creation
slug: template-governed-document-creation
iri: arqix:requirements/req-00-00-00-05

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-05
      - arqix:user-stories/us-01-01-10
      - arqix:user-stories/us-01-01-13
      - arqix:user-stories/us-01-01-20
      - arqix:user-stories/us-02-01-05
      - arqix:user-stories/us-02-01-07
      - arqix:user-stories/us-05-01-03
      - arqix:user-stories/us-06-01-03
      - arqix:user-stories/us-08-01-05
      - arqix:user-stories/us-08-01-10
      - arqix:user-stories/us-08-01-23
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Every document created via `doc new` matches the configured template for its kind, including required frontmatter fields and placeholder substitution.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-02
  updated: 2026-07-11
  lang: en
  translation-of:
  generated: false
---

## Requirement

When a document is created, arqix SHALL instantiate the configured template for the requested kind.

### Notes

Curated from acceptance criteria on template-based `doc new` creation and template/validation contract alignment.
Pandoc render templates and CI workflow templates were excluded.

Contributing stories: 11 (see `derived-from`).
Approved via `docs/en/plans/requirements-derivation-2026-07-02/CROSS-CONCERNS.md`.
