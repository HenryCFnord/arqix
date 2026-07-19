---
id: REQ-08-01-43-02
title: Hold External Predicates to the Declared List
slug: hold-external-predicates-to-the-declared-list
iri: arqix:requirements/req-08-01-43-02

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-08-01-43
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: A triple predicate like skos:exactMatch is ONT-010 unless [frontmatter].allowed-external-properties lists it; the shipped default list is empty, so a corpus using only arqix vocabulary is unaffected.

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

When a declared triple names a predicate outside the arqix namespace that the effective external-property list does not carry, arqix SHALL report the predicate.

### Notes

The same declared-list discipline `allowed-external-types` applies to `rdf.type` (ONT-002), extended to predicates as ONT-010 (ADR-0022) — external-vocabulary typos surface at lint time.
Derived from US-08-01-43.
