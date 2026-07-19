---
id: REQ-08-01-31-03
title: Guard the Reserved Core Against Divergent Redefinition
slug: guard-the-reserved-core-against-divergent-redefinition
iri: arqix:requirements/req-08-01-31-03

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-08-01-31
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: A corpus ontology document that redefines a reserved-core IRI with different type, subclass parents, domain, or range is an ONT-009 finding; a re-declaration with identical semantics stays silent, so the authoring corpus needs no special case.

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

When a corpus ontology document redefines a reserved-core IRI with semantics that differ from the shipped definition, arqix SHALL report the redefinition.

### Notes

Shadowing means changing (ADR-0021): re-stating the shipped definition is authorship and stays silent, a divergent redefinition is ONT-009.
Module IRIs are deliberately not guarded — a corpus definition overrides the embedded one by the precedence rule.
Derived from US-08-01-31.
