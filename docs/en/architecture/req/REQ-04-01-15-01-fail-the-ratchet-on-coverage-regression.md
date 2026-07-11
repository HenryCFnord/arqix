---
id: REQ-04-01-15-01
title: Fail the Ratchet on Coverage Regression
slug: fail-the-ratchet-on-coverage-regression
iri: arqix:requirements/req-04-01-15-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-04-01-15
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Removing the last active verifying test of a requirement makes the ratchet exit 1 with a finding naming that requirement.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-10
  updated: 2026-07-10
  lang: en
  translation-of:
  generated: false
---

## Requirement

When verified coverage decreases against the committed baseline, arqix SHALL fail the coverage ratchet with findings that name each regressed requirement.

### Notes

Derived from US-04-01-15.
The baseline is the committed report snapshot state, kept fresh by the report-freshness gate.
