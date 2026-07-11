---
id: REQ-01-01-09-06
title: Keep Extension Points Free of Process Rules
slug: keep-extension-points-free-of-process-rules
iri: arqix:requirements/req-01-01-09-06

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
  fit-criterion: No skill or prompt library contains scope rules, editing constraints, or verification requirements; those live only in the canonical agent instruction document.

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

Agent-specific extension points SHALL NOT carry normative process rules.

### Notes

Derived from the acceptance criteria of US-01-01-09 after the agent-agnostic generalisation (see ADR-0001).
Normative process rules live exclusively in the canonical agent instruction document so that every coding agent sees the same contract.
