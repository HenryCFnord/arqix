---
id: REQ-08-01-32-01
title: Fill Set Placeholders and Report Unused Keys
slug: fill-set-placeholders-and-report-unused-keys
iri: arqix:requirements/req-08-01-32-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-08-01-32
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: doc new with --set context=tmforum fills {context} in the created document; --set with a key the template does not contain exits non-zero naming the key; a malformed --set without '=' is a usage error.

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

When `arqix doc new` runs with `--set key=value` arguments, arqix SHALL fill each named `{key}` template placeholder with its value and report every key the template does not use.

### Notes

Rule TPL-003 for the unused key; TPL-002 keeps covering placeholders nothing fills.
Derived from US-08-01-32.
