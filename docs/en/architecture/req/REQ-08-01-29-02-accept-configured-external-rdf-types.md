---
id: REQ-08-01-29-02
title: Accept Configured External Rdf Types
slug: accept-configured-external-rdf-types
iri: arqix:requirements/req-08-01-29-02

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-08-01-29
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: A corpus configuring [frontmatter].allowed-external-types accepts a listed non-arqix rdf.type and reports ONT-002 for an unlisted one; without configuration the built-in list gates unchanged.

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

When `arqix lint frontmatter` runs, arqix SHALL accept a non-arqix `rdf.type` only when it lies in the effective external-type vocabulary — the configured `[frontmatter].allowed-external-types` when present, the built-in list otherwise.

### Notes

Rule ONT-002 keeps its substance; only its vocabulary binding moves to configuration (ADR-0011 one-source rule, ADR-0017).
Derived from US-08-01-29.
