---
id: REQ-08-01-30-04
title: Check Paths Against the Rendered Dir Template
slug: check-paths-against-the-rendered-dir-template
iri: arqix:requirements/req-08-01-30-04

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-08-01-30
      - arqix:user-stories/us-08-01-38
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: A term with context tmforum under contexts/itu/terms/ is an FM-010 finding naming both paths, as is a term missing the context property; the same term under contexts/tmforum/terms/ passes, and kinds without dir-template are unchecked.

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

When `arqix lint frontmatter` checks a document whose kind declares a `dir-template`, arqix SHALL report the document unless its parent directory equals the template rendered from the document's own properties, slug, and kind.

### Notes

Rule FM-010; the checker-side direction of the creation contract (REQ-08-01-25-05).
Derived from US-08-01-30.
