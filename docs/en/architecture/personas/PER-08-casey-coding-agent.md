---
id: PER-08
title: Casey Coding Agent
slug: casey-coding-agent
iri: arqix:personas/per-08

rdf:
  type:
    - arqix:classes/persona

triples: []

properties:
  role: Automation-focused coding agent
  description: Executes story-by-story implementation and verification within deterministic, machine-readable contracts, using templates, trace markers, and structured diagnostics.

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-03-05
  updated: 2026-03-28
  lang: en
  translation-of:
  generated: false
---

# Casey Coding Agent

Casey is an automation-focused coding agent that executes tasks story by story. Casey is effective when inputs, rules, and outputs are deterministic and machine-readable. Casey follows contracts, not vibes.

## Goals

- Create documents via templates without ambiguity.
- Annotate code and tests with trace markers consistently.
- Run verification loops locally and in CI without special cases.
- Stay within a declared scope and avoid repository-wide churn.

## Success Looks Like

- arqix commands are used instead of manual file creation.
- Markers (`implements`, `verifies`) link requirements to code and tests.
- `fmt`, `lint`, `trace scan`, and coverage checks are reproducible.
- Failures produce structured diagnostics and clear stop conditions.

## Pain Points

- Unclear defaults (where to write, how to choose IDs).
- Non-deterministic outputs that create noisy diffs.
- Missing source locations in diagnostics.
- Implicit processes that require human interpretation.

## Typical Workflow with arqix

Casey reads the plan for a single story, creates or updates docs via templates, implements code and tests, adds trace markers, and runs the blessed verification loop (Taskfile or `agent verify`). If checks fail, Casey fixes minimally within scope or stops with actionable diagnostics.

## Important arqix Capabilities and Commands

- `doc new` (including translation scaffolding)
- `fmt`, `lint run`
- `trace scan`, `trace coverage`
- Taskfile workflows (`task verify`, `task ci:pr`)
- Structured diagnostics (`--format json`) and deterministic outputs

## artefacts They Care About

- User stories and requirements (as contracts)
- Trace and coverage outputs
- Diagnostics in JSON
- Published site outputs (to validate navigation)

## Boundaries

Casey must not perform opportunistic refactors or large-scale reorganizations unless explicitly requested. Casey operates within scope.

## Open Needs

Casey benefits from consistent CLI contracts, stable export schemas, and optional policy checks that validate scope boundaries.
