---
id: REQ-03-01-10-01
title: Claim Planned Tests with a Language-Neutral Marker
slug: claim-planned-tests-with-a-language-neutral-marker
iri: arqix:requirements/req-03-01-10-01

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
  fit-criterion: A scanned file carrying an arqix:plans marker yields a planned claim in trace coverage, without any framework skip syntax.

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

When `arqix trace scan` detects an `arqix:plans` marker, arqix SHALL record the claimed requirement as planned.

### Notes

Derived from US-03-01-10.
The marker rides in comments like its siblings (`verifies`, `implements`), so it is language-neutral by construction; Rust's `#[ignore]` detection stays as a convenience on top.
