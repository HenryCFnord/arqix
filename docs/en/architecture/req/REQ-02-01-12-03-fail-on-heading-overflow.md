---
id: REQ-02-01-12-03
title: Fail on Heading Overflow
slug: fail-on-heading-overflow
iri: arqix:requirements/req-02-01-12-03

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-02-01-12
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: A shift that would push any heading beyond level six yields exit 1 with a finding naming the fragment and the heading; no partial page is written.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-11
  updated: 2026-07-11
  lang: en
  translation-of:
  generated: false
---

## Requirement

If a heading shift would exceed level six, then arqix SHALL fail the assembly with a diagnostic naming the fragment and the heading.

### Notes

Derived from US-02-01-12.
Silent clamping would corrupt the outline the site split and the PDF table of contents both consume (ADR-0013: ASM-005).
