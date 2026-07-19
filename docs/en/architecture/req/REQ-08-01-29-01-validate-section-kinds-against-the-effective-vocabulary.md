---
id: REQ-08-01-29-01
title: Validate Section Kinds Against the Effective Vocabulary
slug: validate-section-kinds-against-the-effective-vocabulary
iri: arqix:requirements/req-08-01-29-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-08-01-29
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: A corpus configuring [frontmatter].section-kinds accepts a listed kind and reports FM-007 for an unlisted kind; without configuration the built-in vocabulary gates unchanged.

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

When `arqix lint frontmatter` runs, arqix SHALL validate `properties.section-kind` against the effective vocabulary — the configured `[frontmatter].section-kinds` when present, the built-in vocabulary otherwise.

### Notes

Rule FM-007 keeps its substance; only its vocabulary binding moves to configuration (ADR-0011 one-source rule, ADR-0017).
Derived from US-08-01-29.
