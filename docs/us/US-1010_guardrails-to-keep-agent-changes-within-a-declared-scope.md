---
id: US-1010
kind: user_story
title: "Guardrails to keep agent changes within a declared scope"
status: draft
tags:
  - user-story
owner: hcf
created: 
updated: 
priority: 
related:
  requirements: []
  docs: [ADR-0012]
  adrs: []
lang: en
translation_of:
translation_status: draft
generated: false
source:
---

## Guardrails to Keep Agent Changes within a Declared Scope

### Story

As a maintainer, I want enforceable guardrails for automation agents that limit modifications to a declared scope (files, directories, and story), so that agent contributions remain reviewable and do not cause opportunistic repository-wide churn.

### Acceptance Criteria

- A policy file format is defined (minimal YAML or TOML).
- `arqix policy check` can evaluate a list of changed files against a policy.
- Violations produce structured diagnostics.
- The mechanism is optional and can be introduced as warn-only first.

### Notes

Scope:

In scope:

- A policy definition mechanism (file-based) for allowed paths and allowed operations per story/task
- A check command suitable for CI, e.g. `arqix policy check ...`

Out of scope:

- Full sandboxing of execution
- Fine-grained AST-level modification constraints

Meta:

- Workflows: WF-0008; WF-0001
- Story type: governance
- Old ID: US-8005
