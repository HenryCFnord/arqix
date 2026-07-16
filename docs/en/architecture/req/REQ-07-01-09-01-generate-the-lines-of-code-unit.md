---
id: REQ-07-01-09-01
title: Generate the Lines-of-Code Unit
slug: generate-the-lines-of-code-unit
iri: arqix:requirements/req-07-01-09-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-07-01-09
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: After arqix report snapshot, units/lines-of-code.md exists with one table row per component (files, lines, non-blank lines) plus a total row, and report snapshot --check gates it for freshness.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-16
  updated: 2026-07-16
  lang: en
  translation-of:
  generated: false
---

## Requirement

When `arqix report snapshot` runs, arqix SHALL regenerate a lines-of-code unit counting the repository's Rust source files per component.

### Notes

Answers Q-09 of the report catalog; the count is internal (files, total lines, non-blank lines per component), so the unit is deterministic and freshness-gated like the other units.
Derived from US-07-01-09.
