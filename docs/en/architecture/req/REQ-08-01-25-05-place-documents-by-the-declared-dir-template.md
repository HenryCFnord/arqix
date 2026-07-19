---
id: REQ-08-01-25-05
title: Place Documents by the Declared Dir Template
slug: place-documents-by-the-declared-dir-template
iri: arqix:requirements/req-08-01-25-05

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-08-01-25
      - arqix:user-stories/us-08-01-33
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: With dir-template "contexts/{context}/terms" and --set context=tmforum the document lands in contexts/tmforum/terms/; an explicit --dir still wins; a rendered path that is absolute or contains '..' is a usage error.

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

When `arqix doc new` runs without an explicit `--dir` and the kind declares a `dir-template`, arqix SHALL place the document in the directory produced by filling the template from the `--set` values and the derived slug, containment-guarded like an explicit `--dir`.

### Notes

Placement precedence stays: explicit `--dir`, then `dir-template`, then the declared `dir`, then the default.
Derived from US-08-01-25.
