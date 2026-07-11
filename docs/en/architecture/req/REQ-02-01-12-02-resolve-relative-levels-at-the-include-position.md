---
id: REQ-02-01-12-02
title: Resolve Relative Levels at the Include Position
slug: resolve-relative-levels-at-the-include-position
iri: arqix:requirements/req-02-01-12-02

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-02-01-12
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: Moving a `level=+1` include from an h2 section into an h3 section changes the fragment's first heading from h3 to h4 without editing the fragment.

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

When an include directive declares a relative heading level, arqix SHALL resolve it against the heading level in effect at the include position.

### Notes

Derived from US-02-01-12.
The level in effect is the last heading the assembler has seen in the parent walk; before any heading, the base is zero.
