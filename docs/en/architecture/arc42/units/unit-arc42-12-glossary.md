---
id: unit-arc42-12
title: Glossary
slug: glossary
iri: arqix:units/unit-arc42-12

rdf:
  type:
    - arqix:classes/unit

triples: []

properties:
  section-kind: arc42-chapter

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-03
  updated: 2026-07-04
  lang: en
  translation-of:
  generated: false
---

## Glossary

First project-specific terms; the full glossary will be scaffolded via `arqix doc new glossary` with stable IDs (REQ-01-01-12-*) once the tool exists.
For domain vocabulary, the ontology labels under `docs/ontology/` remain the controlled vocabulary.

| Term | Definition |
| --- | --- |
| red skeleton | The complete set of `#[ignore]`d command-contract tests under `tests/cli_*.rs`, one group per row of the command-ownership table (chapter 5), created before any implementation. It is simultaneously the implementation backlog (what is still ignored), the progress gauge (requirements referenced by `verifies` markers), and the done criterion (everything un-ignored and green). The name blends Kent Beck's red–green–refactor cycle with Alistair Cockburn's walking skeleton; the near-collision with the comedian Red Skelton is accidental but mnemonic. |
| red phase | The first step of implementing a story test-first (AGENTS.md, "Test-driven implementation"): un-ignore the story's skeleton tests, refine their bodies and fixtures, run them, and prove they fail. The red output goes into the pull request as evidence that the tests preceded the code and can actually fail. Only then is the behaviour implemented until green. |
