---
id: REQ-01-01-02-03
title: Explain Unit Creation in Command Help
slug: explain-unit-creation-in-command-help
iri: arqix:requirements/req-01-01-02-03

rdf:
  type:
    - arqix:classes/quality-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-02
      - arqix:user-stories/us-02-01-02
      - arqix:user-stories/us-05-01-01
      - arqix:user-stories/us-06-01-01
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: The help output answers location, optional metadata, and ID supply without consulting external documentation.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-02
  updated: 2026-07-10
  lang: en
  translation-of:
  generated: false
---

## Requirement

The `unit new` command help SHOULD explain where units are created, which metadata is optional, and how IDs are supplied.

### Notes

Derived from the acceptance criteria of US-01-01-02 during the group-01 pilot derivation (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`).
Cross-cutting behaviour is linked via the story's `has-requirement`, not restated here.
