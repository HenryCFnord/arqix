---
id: REQ-08-01-33-01
title: Mint Ids From the Declared Id Template
slug: mint-ids-from-the-declared-id-template
iri: arqix:requirements/req-08-01-33-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-08-01-33
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: With id-template "{context}-{slug}" and --set context=tmforum --title Intent the created id is tmforum-intent; without the covering --set the command exits non-zero naming the placeholder; an explicit --id still wins.

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

When `arqix doc new` runs without an explicit `--id` and the kind declares an `id-template`, arqix SHALL mint the id by filling the template from the `--set` values and the derived slug and report every placeholder no value covers.

### Notes

The minted id passes the same validity and uniqueness checks as an explicit `--id`.
Derived from US-08-01-33.
