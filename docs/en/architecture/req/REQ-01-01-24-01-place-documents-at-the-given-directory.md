---
id: REQ-01-01-24-01
title: Place Documents at the Given Directory
slug: place-documents-at-the-given-directory
iri: arqix:requirements/req-01-01-24-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-24
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: doc new adr --dir contexts/geo/decisions writes the document under contexts/geo/decisions/; an absolute path or one containing ".." exits 2 without writing.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-15
  updated: 2026-07-15
  lang: en
  translation-of:
  generated: false
---

## Requirement

When the caller passes an explicit target directory, arqix SHALL create the document under that repository-contained directory.

### Notes

Derived from US-01-01-24 (authoring-ergonomics band, knowledge-repository intake gap G3).
The explicit directory takes precedence over the declared `[kinds.<family>].dir` (REQ-01-01-22-01) and the `<first-root>/<kind>/` default.
An absolute path or a path containing `..` is rejected as a usage error before anything is written (filesystem containment, REQ-00-00-00-13).
