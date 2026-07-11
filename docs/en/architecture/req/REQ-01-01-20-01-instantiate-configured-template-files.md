---
id: REQ-01-01-20-01
title: Instantiate Configured Template Files
slug: instantiate-configured-template-files
iri: arqix:requirements/req-01-01-20-01

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
  fit-criterion: Editing a kind's template file changes the next created document, without rebuilding arqix.

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

When a document is created, arqix SHALL instantiate the template file configured for the document's kind.

### Notes

Derived from US-01-01-20.
Replaces the template string literals in the engine (audit row C5); placeholder substitution for `{id}`, `{title}`, and `{slug}` is unchanged.
