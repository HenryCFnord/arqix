---
id: REQ-02-01-12-04
title: Apply the Configured Heading Ownership Default
slug: apply-the-configured-heading-ownership-default
iri: arqix:requirements/req-02-01-12-04

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
  fit-criterion: Switching `heading-ownership` between `child` and `parent` changes the behaviour of bare includes corpus-wide without editing a single directive.

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

Where no heading level is declared on an include directive, arqix SHALL apply the configured heading-ownership default.

### Notes

Derived from US-02-01-12.
Under `child` (the default) a bare include behaves as `level=+1`; under `parent` fragments are authored headingless and inline verbatim (ADR-0013).
