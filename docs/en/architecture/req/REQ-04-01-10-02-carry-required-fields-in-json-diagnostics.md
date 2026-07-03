---
id: REQ-04-01-10-02
title: Carry Required Fields in JSON Diagnostics
slug: carry-required-fields-in-json-diagnostics
iri: arqix:requirements/req-04-01-10-02

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
  fit-criterion: Each diagnostic carries all five fields; source fields are explicit when unavailable.

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

JSON diagnostics SHALL include at least `severity`, `code`, `message`, `source.path`, and `source.line` where available.

### Notes

Derived from the acceptance criteria of US-04-01-10, US-05-01-14, US-08-01-21 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10). Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
