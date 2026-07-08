---
id: REQ-04-01-10-01
title: Accept a JSON Format Option per Command
slug: accept-a-json-format-option-per-command
iri: arqix:requirements/req-04-01-10-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-04-01-10
      - arqix:user-stories/us-05-01-14
      - arqix:user-stories/us-08-01-21
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Every supported command can be switched to JSON diagnostics via a documented option.

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

Each supported command SHALL accept `--format json` or an equivalent option to emit JSON diagnostics.

### Notes

Derived from the acceptance criteria of US-04-01-10, US-05-01-14, US-08-01-21 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10).
Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
