---
id: US-02-01-03
title: Format Documents During Implementation
slug: format-documents-during-implementation
iri: arqix:user-stories/us-02-01-03

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-02
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-00-00-00-01
      - arqix:requirements/req-00-00-00-06
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-02-01

properties:
  priority: high
  edge-case: false

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-03-29
  updated: 2026-07-02
  lang: en
  translation-of:
  generated: false
---

## Format Documents During Implementation

As a developer, I want to format documents canonically, so that I can keep documentation clean during implementation without wasting time on formatting debates.

### Acceptance Criteria

- [ ] `arqix fmt` sorts frontmatter keys according to configurable `key_order`.
- [ ] `arqix fmt` normalizes directives, including attribute order and whitespace, without semantic changes.
- [ ] Formatting is idempotent across repeated runs on the same input.
- [ ] Formatting keeps document diffs focused on content rather than incidental style changes.

### Notes

Acceptance should prove that formatting is idempotent and does not change document meaning beyond canonical ordering and whitespace normalization.
Add snapshot-style tests for frontmatter key ordering and directive normalization on realistic inputs.
Any fields or directives intentionally left untouched should be called out explicitly to avoid accidental scope growth.
The main value for a developer is fast, predictable cleanup in the normal implementation workflow.
