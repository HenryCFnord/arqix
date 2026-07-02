---
id: REQ-01-01-16-03
title: Identify Failing Key in Config Diagnostics
slug: identify-failing-key-in-config-diagnostics
iri: arqix:requirements/req-01-01-16-03

rdf:
  type:
    - arqix:classes/quality-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-16
      - arqix:user-stories/us-04-01-11
      - arqix:user-stories/us-05-01-11
      - arqix:user-stories/us-08-01-20
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: A validation finding names the offending key and the file it came from where the source is known.

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-02
  updated: 2026-07-02
  lang: en
  translation-of:
  generated: false
---

## Requirement

Configuration diagnostics SHOULD identify the failing key and source file.

### Notes

Derived from the acceptance criteria of US-01-01-16 during the group-01 pilot derivation (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`). Cross-cutting behaviour is linked via the story's `has-requirement`, not restated here.
