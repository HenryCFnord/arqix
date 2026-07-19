---
id: REQ-08-01-36-02
title: Report Subclass Cycles
slug: report-subclass-cycles
iri: arqix:requirements/req-08-01-36-02

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-08-01-36
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: Two classes naming each other as sub-class-of are an ONT-008 finding; a class naming itself stays legal as the hierarchy-root convention.

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

When `arqix lint frontmatter` runs, arqix SHALL report every `sub-class-of` cycle that is longer than a class's own root self-reference.

### Notes

Rule ONT-008; the self-reference stays the hierarchy-root convention.
Derived from US-08-01-36.
