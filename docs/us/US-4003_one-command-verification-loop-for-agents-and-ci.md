---
id: US-4003
kind: user_story
title: One-command verification loop for agents and CI
status: draft
tags: []
owner: 
created: 
updated: 
priority: 
related:
  requirements: []
  personas:
  - PER-0004
lang: 
translation_of: 
translation_status: 
generated: 
source: 
---

# US-4003 — One-command verification loop for agents and CI

As an Automation Agent (and DevOps Engineer), I want a single arqix command to run the standard verification loop (format, lint, trace, coverage) and return a consolidated result, so that automation and CI can validate changes with one deterministic invocation.

## Acceptance Criteria

- `arqix check` runs the configured sub-steps and fails fast or aggregates results (configurable).
- Exit code reflects overall status using the standard contract.
- A JSON mode emits the per-step results and diagnostics references.

## Notes

In scope:
- A `arqix check` or `arqix verify` command that runs:
  - formatting (optionally check-only)
  - lint
  - trace scan
  - coverage report
- Consolidated summary output
- Structured JSON output option

Out of scope:
- Rendering (PDF/site) in the default check loop
- Auto-fix beyond formatting

