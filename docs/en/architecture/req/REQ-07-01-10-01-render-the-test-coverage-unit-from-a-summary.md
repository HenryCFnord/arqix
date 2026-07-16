---
id: REQ-07-01-10-01
title: Render the Test-Coverage Unit from a Summary
slug: render-the-test-coverage-unit-from-a-summary
iri: arqix:requirements/req-07-01-10-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-07-01-10
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: Given a cargo-llvm-cov JSON export, arqix report coverage --input <file> --stamp <text> writes units/test-coverage.md with the total percentages and a per-file table, byte-identical for identical input; a missing or malformed input is a usage error.

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

When `arqix report coverage` runs with a coverage summary file, arqix SHALL render the test-coverage unit from the summary's totals and per-file rows.

### Notes

Answers Q-10 of the report catalog; the unit carries the generated-by-CI provenance header and deliberately stays outside the snapshot freshness gate.
Derived from US-07-01-10.
