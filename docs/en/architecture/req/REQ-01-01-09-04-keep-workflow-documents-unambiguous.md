---
id: REQ-01-01-09-04
title: Keep Workflow Documents Unambiguous
slug: keep-workflow-documents-unambiguous
iri: arqix:requirements/req-01-01-09-04

rdf:
  type:
    - arqix:classes/quality-requirement

triples:
  - predicate: arqix:properties/derived-from
    object: arqix:user-stories/us-01-01-09
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: An agent can execute a story from the documents alone, without out-of-band clarification.

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

The agent instruction and plan document structures SHOULD be explicit enough that an agent can follow them without guessing process constraints.

### Notes

Derived from the acceptance criteria of US-01-01-09 during the group-01 pilot derivation (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`). Cross-cutting behaviour is linked via the story's `has-requirement`, not restated here.
