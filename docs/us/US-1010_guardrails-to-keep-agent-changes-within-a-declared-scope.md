---
id: US-1010
kind: user_story
title: Guardrails to keep agent changes within a declared scope
status: draft
workflows:
- WF-0008
- WF-0001
story_type: governance
persona: PER-0001
old_id: US-8005
related:
  requirements: []
  personas:
  - PER-0001
---
# US-1010 — Guardrails to keep agent changes within a declared scope

As a Maintainer, I want enforceable guardrails for automation agents that limit modifications to a declared scope (files, directories, and story), so that agent contributions remain reviewable and do not cause opportunistic repository-wide churn.

## Scope

In scope:
- A policy definition mechanism (file-based) for allowed paths and allowed operations per story/task
- A check command suitable for CI, e.g. `darcy policy check ...`

Out of scope:
- Full sandboxing of execution
- Fine-grained AST-level modification constraints

## Acceptance Criteria

- A policy file format is defined (minimal YAML or TOML).
- `darcy policy check` can evaluate a list of changed files against a policy.
- Violations produce structured diagnostics.
- The mechanism is optional and can be introduced as warn-only first.
