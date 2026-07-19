---
id: REQ-08-01-25-03
title: Reject Unknown Template Placeholders
slug: reject-unknown-template-placeholders
iri: arqix:requirements/req-08-01-25-03

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-08-01-25
      - arqix:user-stories/us-08-01-26
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: A declared template containing {bogus} makes doc new report a TPL-002 finding naming the placeholder and exit 1 without writing a file.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-15
  updated: 2026-07-19
  lang: en
  translation-of:
  generated: false
---

## Requirement

When a declared template contains a placeholder outside the documented vocabulary, arqix SHALL report the unknown placeholder as a finding and create no document.

### Notes

Derived from US-08-01-25 (authoring-ergonomics band, knowledge-repository intake gap G1).
The vocabulary is `{id}`, `{title}`, `{slug}`, `{iri_slug}`, `{kind}`, `{namespace}`, and `{lifecycle}`; a placeholder is a braced lowercase identifier, so YAML literals such as `{}` never match.
Validation applies to declared templates only: the directory-resolved and embedded paths keep their present behaviour.
