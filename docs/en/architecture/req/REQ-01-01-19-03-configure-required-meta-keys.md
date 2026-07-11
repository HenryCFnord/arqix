---
id: REQ-01-01-19-03
title: Configure Required Meta Keys
slug: configure-required-meta-keys
iri: arqix:requirements/req-01-01-19-03

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-19
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: Removing a key from the configured required set stops validation requiring it, without a code change; the default set is today's six keys.

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

When a document's meta block is validated, arqix SHALL require exactly the meta keys declared in the effective configuration.

### Notes

Derived from US-01-01-19.
Audit row C6 — the row whose two hardcoded copies had already diverged before the parity fix; the configured set ends that failure mode, and the document templates draw from the same set.
