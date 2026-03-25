---
id: US-3004
kind: user_story
title: Deterministic trace and coverage outputs
status: draft
tags: []
owner: 
created: 
updated: 
priority: 
related:
  requirements: []
  personas:
  - PER-0003
lang: 
translation_of: 
translation_status: 
generated: 
source: 
---

# US-3004 — Deterministic trace and coverage outputs

As an Automation Agent, I want trace graphs, matrices, and coverage reports to be deterministic (stable ordering and stable formatting), so that I can produce clean diffs and reliably detect meaningful changes.

## Acceptance Criteria

- `trace scan` JSON output orders nodes and edges deterministically (e.g., by kind then id, then path).
- `trace matrix` outputs deterministic row/column ordering (configurable defaults allowed).
- `coverage report` output is deterministic.
- CSV and JSON outputs have stable field ordering where applicable.

## Notes

In scope:
- Deterministic ordering for nodes/edges, matrix rows/columns, and report sections
- Stable formatting for JSON/CSV outputs

Out of scope:
- Semantic diffing or change explanations
- Performance optimizations

