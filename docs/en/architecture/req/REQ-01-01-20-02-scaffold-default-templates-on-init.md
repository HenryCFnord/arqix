---
id: REQ-01-01-20-02
title: Scaffold Default Templates on Init
slug: scaffold-default-templates-on-init
iri: arqix:requirements/req-01-01-20-02

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-20
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: A fresh package contains editable template files that reproduce the built-in defaults byte-identically.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-11
  updated: 2026-07-11
  lang: en
  translation-of:
  generated: false
---

## Requirement

When `arqix doc init` scaffolds a package, arqix SHALL write the default template files into the configured template directory.

### Notes

Derived from US-01-01-20.
Init never overwrites (REQ-00-00-00-08), so re-running it leaves customised templates untouched.
