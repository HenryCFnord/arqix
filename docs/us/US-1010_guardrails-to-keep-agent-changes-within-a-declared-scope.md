---
id: US-1010
kind: user_story
title: "Guardrails to keep agent changes within a declared scope"
status: draft
tags:
  - user-story
owner: hcf
created: 2026-03-26
updated: 2026-03-26
priority: low
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

In scope, the work should define a file-based policy mechanism that specifies allowed paths and allowed operations per story or task, and it should also provide a check command suitable for CI, for example `arqix policy check ...`.

Out of scope are full sandboxing of execution and fine-grained AST-level modification constraints.
