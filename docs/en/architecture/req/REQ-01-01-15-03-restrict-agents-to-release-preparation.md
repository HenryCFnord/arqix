---
id: REQ-01-01-15-03
title: Restrict Agents to Release Preparation
slug: restrict-agents-to-release-preparation
iri: arqix:requirements/req-01-01-15-03

rdf:
  type:
    - arqix:classes/constraint

triples:
  - predicate: arqix:properties/derived-from
    object: arqix:user-stories/us-01-01-15
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: No agent-driven workflow performs tagging or publishing; those steps require human approval.

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

Coding agents SHALL NOT tag or publish releases without explicit approval.

### Notes

Derived from the acceptance criteria of US-01-01-15 during the group-01 pilot derivation (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`). Cross-cutting behaviour is linked via the story's `has-requirement`, not restated here.
