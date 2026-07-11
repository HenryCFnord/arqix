---
id: REQ-01-01-20-03
title: Fail Clearly on Missing Template Files
slug: fail-clearly-on-missing-template-files
iri: arqix:requirements/req-01-01-20-03

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-20
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: A missing template file yields exit 1 and a finding that carries the expected path; no partial document is written.

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

If a configured template file is missing, then arqix SHALL fail with a diagnostic naming the expected path.

### Notes

Derived from US-01-01-20.
The error case that string literals could not have: once templates are files, their absence is a user-fixable condition and must be diagnosed as one.
