---



id: US-01-01-07
title: Enforce Scope Guardrails for Automation Agents
slug: enforce-scope-guardrails-for-automation-agents
iri: arqix:user-stories/us-01-01-07

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-01
  - predicate: arqix:properties/has-requirement
    object:
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-01-01

properties:
  priority: medium
  edge-case: false

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-03-30
  updated: 2026-04-04
  lang: en
  translation-of:
  generated: false
---


## Enforce Scope Guardrails for Automation Agents

As a maintainer, I want enforceable guardrails for automation agents that limit modifications to a declared scope such as files, directories, and story boundaries, so that agent contributions remain reviewable and do not cause opportunistic repository-wide churn.

### Acceptance Criteria

- [ ] A policy file format is defined in minimal YAML or TOML.
- [ ] `arqix policy check` can evaluate a list of changed files against a policy.
- [ ] Violations produce structured diagnostics.
- [ ] The mechanism is optional and can be introduced as warn-only first.

### Notes

In scope, the work should define a file-based policy mechanism that specifies allowed paths and allowed operations per story or task, and it should also provide a check command suitable for CI, for example `arqix policy check ...`. Out of scope are full sandboxing of execution and fine-grained AST-level modification constraints. The main value for Mara is repository governance and reviewable automation behaviour.
