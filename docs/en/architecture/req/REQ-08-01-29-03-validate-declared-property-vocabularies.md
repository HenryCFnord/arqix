---
id: REQ-08-01-29-03
title: Validate Declared Property Vocabularies
slug: validate-declared-property-vocabularies
iri: arqix:requirements/req-08-01-29-03

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-08-01-29
      - arqix:user-stories/us-08-01-35
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: With [kinds.term.vocab] extraction-status = ["extracted", "proposed", "decided"] a term carrying extraction-status proposed passes and one carrying bogus is an FM-009 finding naming field, value, and vocabulary; without the vocab table nothing changes.

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

When `arqix lint frontmatter` checks a document whose kind declares a vocabulary for a `properties` field, arqix SHALL report every value of that field outside the declared vocabulary.

### Notes

Rule FM-009; the domain-state axis next to the guarded lifecycle (FM-008), per the two-axes rule.
Derived from US-08-01-29.
