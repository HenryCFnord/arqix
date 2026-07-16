---
id: US-08-01-24
title: Scaffold Agent Instructions on Init
slug: scaffold-agent-instructions-on-init
iri: arqix:user-stories/us-08-01-24

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-08
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-08-01-24-01
      - arqix:requirements/req-08-01-24-02
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-08-01

properties:
  priority: medium
  edge-case: false

external-references: []

meta:
  lifecycle-status: done
  owner: hcf
  created: 2026-07-12
  updated: 2026-07-15
  lang: en
  translation-of:
  generated: false
---

## Scaffold Agent Instructions on Init

As a coding agent, I want `doc init` to scaffold an agent instruction document, so that any repository I enter through arqix tells me its gate commands and corpus entry points from the first commit.

### Acceptance Criteria

- [x] `arqix doc init` scaffolds an `AGENTS.md` agent instruction document at the repository root.
- [x] The scaffold names the verification loop (`arqix verify`) and the corpus entry points (`doc list/read/search`, `doc new`, `fmt`, `mcp serve`).
- [x] An existing `AGENTS.md` is left byte-identical — init never overwrites authored instructions.

### Notes

This is the `doc init` slice of the agent-onboarding strand (refinement 2026-07-09, strand 3; decision D3 sequenced it after `mcp serve`).
The scaffold is a starting point for the target repository's own process rules, in the spirit of REQ-01-01-09-01..06: the instruction document is where normative rules live, and agent-specific extension points stay thin.
The handbook chapter and the packaged skill from the same strand are corpus content, not CLI behaviour, and carry no requirements of their own.
