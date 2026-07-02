---
id: wf-01-01
title: Establish Standards and Repository Hygiene
slug: establish-standards-and-repository-hygiene
iri: arqix/workflows/wf-01-01

rdf:
  type:
    - arqix:classes/workflow

triples:
  - predicate: arqix:properties/has-primary-persona
    object: arqix:persona/per-01
  - predicate: arqix:properties/has-relevant-persona
    object:

properties:
  goal: Establish a predictable documentation system with templates, routing, markup rules, and quality gates that scale with the team.
  entry-state: A repository may be setup from scratch or may contain Markdown documentation written by multiple contributors, but standards, metadata contracts, and deterministic tooling are not yet clearly established.
  end-state: The documentation system has consistent templates and routing rules, deterministic formatting and linting, documented conventions, and a stable foundation for CI and agents.

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-03-25
  updated: 2026-03-27
  lang: en
  translation-of:
  generated: false
---

## Establish Standards and Repository Hygiene

A repository contains Markdown documentation written by multiple contributors. Without clear standards, metadata contracts, and deterministic tooling, documentation drifts and becomes unreliable.

### Goal

Establish a predictable documentation system: templates, routing, markup rules, and quality gates that scale with the team.

### Steps

1. Initialize or validate `arqix.toml` and the documentation structure.
2. Define the set of supported document kinds (ADR, US, REQ, workflow, persona, unit, glossary).
3. Create and store templates for each kind (EN source).
4. Define markup rules for includes and trace markers.
5. Run `fmt` and `lint` over the documentation tree.
6. Fix violations or adjust rules where they block legitimate use.
7. Document standards and processes in the handbook (including agent rules).

### Outputs

- Consistent templates and routing rules
- Deterministic formatting and linting baseline
- Documented conventions in the handbook
- A stable foundation for CI and agents

### Failure Modes

- Rules are too strict and block real writing patterns.
- Templates are missing and contributors start copy-pasting.
- IDs drift or duplicate due to lack of validation.

### Related Commands

- `arqix config validate`
- `arqix doc new <kind>`
- `arqix fmt`
- `arqix lint run`
- `arqix trace scan` (baseline quality check)

### Automation (optional)

- Taskfile: `taskfiles/wf-0001.yml` (hygiene pass)
