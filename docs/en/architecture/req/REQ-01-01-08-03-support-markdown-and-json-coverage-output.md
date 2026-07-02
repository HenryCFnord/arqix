---
id: REQ-01-01-08-03
title: Support Markdown and JSON Coverage Output
slug: support-markdown-and-json-coverage-output
iri: arqix:requirements/req-01-01-08-03

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object: arqix:user-stories/us-01-01-08
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Both output formats are selectable and contain the same findings.

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

The coverage report SHALL support at least Markdown and JSON output formats.

### Notes

Derived from the acceptance criteria of US-01-01-08 during the group-01 pilot derivation (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`). Cross-cutting behaviour is linked via the story's `has-requirement`, not restated here.
