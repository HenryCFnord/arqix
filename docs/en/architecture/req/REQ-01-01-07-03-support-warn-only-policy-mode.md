---
id: REQ-01-01-07-03
title: Support Warn-Only Policy Mode
slug: support-warn-only-policy-mode
iri: arqix:requirements/req-01-01-07-03

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object: arqix:user-stories/us-01-01-07
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: In warn-only mode, violations are reported as warnings and the exit status remains successful.

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

Where warn-only mode is configured, `arqix policy check` SHALL report violations without failing.

### Notes

Derived from the acceptance criteria of US-01-01-07 during the group-01 pilot derivation (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`). Cross-cutting behaviour is linked via the story's `has-requirement`, not restated here.
