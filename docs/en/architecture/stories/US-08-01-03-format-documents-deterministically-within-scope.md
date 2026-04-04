---


id: US-08-01-03
title: Format Documents Deterministically within Scope
slug: format-documents-deterministically-within-scope
iri: arqix:user-stories/us-08-01-03

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-08
  - predicate: arqix:properties/has-requirement
    object:
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-08-01

properties:
  priority: high
  edge-case: false

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-03-29
  updated: 2026-04-04
  lang: en
  translation-of:
  generated: false
---


## Format Documents Deterministically within Scope

As a coding agent, I want to format documents canonically, so that I can apply repository conventions automatically without causing unnecessary churn outside my scope.

### Acceptance Criteria

- [ ] `arqix fmt` sorts frontmatter keys according to configurable `key_order`.
- [ ] `arqix fmt` normalizes directives, including attribute order and whitespace, without semantic changes.
- [ ] Formatting is idempotent across repeated runs on the same input.
- [ ] Formatting produces deterministic results that do not require human interpretation.
- [ ] Formatting does not introduce unrelated semantic or structural changes beyond canonical ordering and whitespace normalization.

### Notes

Acceptance should prove that formatting is idempotent and does not change document meaning beyond canonical ordering and whitespace normalization.
Add snapshot-style tests for frontmatter key ordering and directive normalization on realistic inputs.
Any fields or directives intentionally left untouched should be called out explicitly to avoid accidental scope growth.
The main value for a coding agent is deterministic convention enforcement with minimal churn and clear scope boundaries.