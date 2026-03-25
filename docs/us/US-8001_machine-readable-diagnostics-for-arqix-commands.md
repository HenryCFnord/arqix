---
id: US-8001
kind: user_story
title: Machine-readable diagnostics for arqix commands
status: draft
tags: []
owner: 
created: 
updated: 
priority: 
related:
  requirements: []
  personas:
  - PER-0008
lang: 
translation_of: 
translation_status: 
generated: 
source: 
---

# US-8001 — Machine-readable diagnostics for arqix commands

As an Automation Agent, I want arqix commands to emit machine-readable diagnostics (JSON) with stable fields and source locations, so that I can automatically interpret failures, localize issues, and either fix them deterministically or stop with actionable output.

## Acceptance Criteria

- Each supported command accepts `--format json` (or equivalent) to output JSON diagnostics.
- JSON diagnostics objects include at minimum:
  - `severity` (info/warning/error)
  - `code` (stable rule/error identifier)
  - `message`
  - `source.path`
  - `source.line` when available
- Commands produce exit codes consistent with failures (0/1/2 contract as defined elsewhere).
- Diagnostics output is deterministic for identical inputs.

## Notes

In scope:
- JSON diagnostics output for `fmt`, `lint`, `trace scan`, `trace matrix`, and `coverage report`
- Diagnostics include severity, code, message, and source location (path, line when available)
- Deterministic ordering of diagnostics

Out of scope:
- Full structured logging/tracing
- Rich UI formatting
- Auto-fix behavior beyond existing `fmt`


This story is a prerequisite for reliable agent workflows and CI gates.