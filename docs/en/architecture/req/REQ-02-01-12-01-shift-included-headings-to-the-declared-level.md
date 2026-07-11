---
id: REQ-02-01-12-01
title: Shift Included Headings to the Declared Level
slug: shift-included-headings-to-the-declared-level
iri: arqix:requirements/req-02-01-12-01

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
  fit-criterion: A fragment starting at h2 included with `level=3` renders h3 with its internal sub-headings shifted by the same delta.

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

When an include directive declares a heading level, arqix SHALL shift every heading of the included fragment by the delta between the declared level and the fragment's first heading.

### Notes

Derived from US-02-01-12.
The whole fragment shifts as one unit (ADR-0013): internal structure is preserved, only re-anchored.
