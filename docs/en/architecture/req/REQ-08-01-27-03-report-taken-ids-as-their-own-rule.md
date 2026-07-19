---
id: REQ-08-01-27-03
title: Report Taken Ids as Their Own Rule
slug: report-taken-ids-as-their-own-rule
iri: arqix:requirements/req-08-01-27-03

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-08-01-27
      - arqix:user-stories/us-08-01-39
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: low
  fit-criterion: doc new with a taken id (explicit or template-minted) exits non-zero with a TPL-004 finding naming id and holder; an unknown template placeholder stays TPL-002.

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

When `arqix doc new` is asked for an id another document already carries, arqix SHALL report it as TPL-004 naming the id and its holder.

### Notes

TPL-002 keeps covering unknown template placeholders only.
Derived from US-08-01-27.
