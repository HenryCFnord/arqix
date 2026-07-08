---
id: US-08-01-08
title: Stay within Declared Change Scope
slug: stay-within-declared-change-scope
iri: arqix:user-stories/us-08-01-08

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-08
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-00-00-00-03
      - arqix:requirements/req-00-00-00-07
      - arqix:requirements/req-01-01-07-01
      - arqix:requirements/req-01-01-07-02
      - arqix:requirements/req-01-01-07-03
      - arqix:requirements/req-00-00-00-13
      - arqix:requirements/req-00-00-00-14
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-08-01

properties:
  priority: high
  edge-case: false

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-03-30
  updated: 2026-07-02
  lang: en
  translation-of:
  generated: false
---

## Stay within Declared Change Scope

As a coding agent, I want enforceable guardrails for automation agents that limit modifications to a declared scope such as files, directories, and story boundaries, so that I can execute work predictably without causing out-of-scope repository churn.

### Acceptance Criteria

- [ ] A policy file format is defined in minimal YAML or TOML.
- [ ] `arqix policy check` can evaluate a list of changed files against a policy.
- [ ] Violations produce structured diagnostics.
- [ ] The mechanism is optional and can be introduced as warn-only first.

### Notes

In scope, the work should define a file-based policy mechanism that specifies allowed paths and allowed operations per story or task, and it should also provide a check command suitable for CI, for example `arqix policy check ...`.
Out of scope are full sandboxing of execution and fine-grained AST-level modification constraints.
The main value for Casey is deterministic scope enforcement and clear stop conditions.
