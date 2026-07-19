---
id: REQ-08-01-25-02
title: Instantiate the Declared Template
slug: instantiate-the-declared-template
iri: arqix:requirements/req-08-01-25-02

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
  fit-criterion: With [kinds.adr] template = "tpl/adr.tpl.md" configured, doc new adr instantiates that file; if the file is missing the command exits 2 without writing.

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

When a `[kinds.<family>]` contract declares a `template`, arqix SHALL instantiate documents of that family from the declared template file.

### Notes

Derived from US-08-01-25 (authoring-ergonomics band, knowledge-repository intake gap G1).
The path is repository-relative; a declared-but-missing template is a configuration error (exit 2), mirroring the configured `[templates] dir` contract (US-01-01-20).
Without a declared `template`, resolution stays exactly as today, so present behaviour is preserved.
