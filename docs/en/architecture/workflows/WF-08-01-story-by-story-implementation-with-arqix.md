---
id: WF-08-01
title: "Automation Agent: Story-by-story Implementation with arqix"
slug: story-by-story-implementation-with-arqix
iri: arqix:workflows/wf-08-01

rdf:
  type:
    - arqix:classes/workflow

triples:
  - predicate: arqix:properties/has-primary-persona
    object: arqix:personas/per-08
  - predicate: arqix:properties/has-relevant-persona
    object:

properties:
  goal: Implement one user story at a time with consistent documentation, trace markers, and verification loops.
  entry-state: A coding agent or automation workflow has a single story in scope, with acceptance criteria and required docs identified.
  end-state: The story is implemented with traceable links between requirements, code, tests, and docs, and deterministic verification outputs are produced.

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-03-25
  updated: 2026-07-02
  lang: en
  translation-of:
  generated: false
---

## Automation Agent: Story-by-story Implementation with arqix

A coding agent (or automation workflow) must execute tasks deterministically, within scope, and with machine-readable diagnostics.

### Goal

Implement one user story at a time with consistent documentation, trace markers, and verification

loops.

### Steps

1. Read the plan for a single story (scope in/out, acceptance criteria).
2. Create required docs via templates (`doc new`) instead of manual file creation.
3. Implement code changes and tests.
4. Add trace markers:
   - `arqix:implements REQ-xxxx`
   - `arqix:verifies REQ-xxxx`
1. Run the blessed verification loop (local equivalent of CI):
   - `fmt`
   - `lint`
   - `trace scan`
   - `trace coverage`
6. Fix only within scope. If ambiguous, stop and report blockers.

### Outputs

- Story-complete PR/commit: code + tests + docs
- Traceable linkage between REQs, code, and tests
- Deterministic verification outputs

### Failure Modes

- Non-deterministic reports and noisy diffs.
- Missing defaults (IDs, routing) causing guesswork.
- Diagnostics without source locations.

### Related Commands

- `arqix doc new <kind>`
- `arqix fmt`
- `arqix lint run`
- `arqix trace scan`
- `arqix trace coverage`
- `arqix trace check --req REQ-xxxx` (assist)

### Automation

- Taskfile: `task verify` / `task ci:pr`
- Taskfile module: `taskfiles/wf-0008.yml`
