---
id: REQ-03-01-09-01
title: Check the Done Claim against Active Tests
slug: check-the-done-claim-against-active-tests
iri: arqix:requirements/req-03-01-09-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-03-01-09
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Marking a story done while one of its requirements has no active verifying test produces an error naming that requirement.

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

When a story declares `lifecycle-status: done`, arqix SHALL report an error for each of its requirements that no active test verifies.

### Notes

Derived from US-03-01-09; the vocabularies and the declared-versus-computed rule are ADR-0010.
Active means not `#[ignore]`d: planned coverage does not satisfy a done claim.
