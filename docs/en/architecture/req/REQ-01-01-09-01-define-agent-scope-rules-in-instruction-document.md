---
id: REQ-01-01-09-01
title: Define Agent Scope Rules in Instruction Document
slug: define-agent-scope-rules-in-instruction-document
iri: arqix:requirements/req-01-01-09-01

rdf:
  type:
    - arqix:classes/constraint

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-09
      - arqix:user-stories/us-08-01-18
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: The agent instruction document contains explicit story-by-story scope rules that reviews can point to.

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

The agent instruction document SHALL define scope rules for story-by-story execution, including one story at a time and no opportunistic refactors.

### Notes

Derived from the acceptance criteria of US-01-01-09 during the group-01 pilot derivation (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`).
Cross-cutting behaviour is linked via the story's `has-requirement`, not restated here.
