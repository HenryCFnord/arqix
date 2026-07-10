---
id: REQ-04-01-15-02
title: Pass the Ratchet without Regression
slug: pass-the-ratchet-without-regression
iri: arqix:requirements/req-04-01-15-02

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
  fit-criterion: A change that only adds specification documents leaves the ratchet at exit 0.

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-10
  updated: 2026-07-10
  lang: en
  translation-of:
  generated: false
---

## Requirement

When a change does not reduce the set of verified requirements, the coverage ratchet SHALL pass.

### Notes

Derived from US-04-01-15.
Specification growth must stay free: new uncovered requirements lower the percentage but never trip the ratchet.
