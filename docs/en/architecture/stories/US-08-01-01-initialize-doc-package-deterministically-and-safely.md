---
id: US-08-01-01
title: Initialize a Doc Package Deterministically and Safely
slug: initialize-doc-package-deterministically-and-safely
iri: arqix:user-stories/us-08-01-01

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-08
  - predicate: arqix:properties/has-requirement
    object:
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-08-01

properties:
  priority: high
  edge-case: true

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-03-28
  updated: 2026-04-04
  lang: en
  translation-of:
  generated: false
---


## Initialize a Doc Package Deterministically and Safely

As a coding agent, I want to initialise a new doc package deterministically and without unsafe overwrites, so that I can create repository artefacts within scope and without ambiguity.

### Acceptance Criteria

- [ ] `arqix doc init <path>` creates the same directory and file scaffold for the same input conditions.
- [ ] `id` and `slug` generation are deterministic for the same title input and slug-rule configuration.
- [ ] The command does not overwrite existing files unless an explicit override mechanism is provided.
- [ ] Failure cases return actionable diagnostics that make the stop condition clear.
- [ ] The generated package can be used directly in the verification loop without requiring manual interpretation or repair.

### Notes

This is an enabling story for story-by-story implementation with arqix.
The key concerns are determinism, safe defaults, and machine-readable failure behavior.
If force or approval modes exist, their contracts should be explicit and testable.
Structured diagnostics for refusal paths would be especially valuable for automation.