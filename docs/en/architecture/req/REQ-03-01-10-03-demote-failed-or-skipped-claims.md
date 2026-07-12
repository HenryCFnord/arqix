---
id: REQ-03-01-10-03
title: Demote Failed or Skipped Claims
slug: demote-failed-or-skipped-claims
iri: arqix:requirements/req-03-01-10-03

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-03-01-10
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: A requirement whose only verifying test failed in the joined report is no longer counted as verified; with a second passing claim it stays verified.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-12
  updated: 2026-07-12
  lang: en
  translation-of:
  generated: false
---

## Requirement

Where a joined test outcome is failed or skipped, the corresponding verifying claim SHALL NOT count as verified.

### Notes

Derived from US-03-01-10.
Demotion is per claim, not per requirement: a requirement stays verified while at least one active claim passes or remains unjoined.
A skipped outcome is the runner-level cousin of the planned state, so the demoted claim counts as planned, not lost.
