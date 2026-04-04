---


id: US-08-01-04
title: Lint Documents with Actionable Diagnostics
slug: lint-documents-with-actionable-diagnostics
iri: arqix:user-stories/us-08-01-04

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
  edge-case: false

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-03-29
  updated: 2026-04-04
  lang: en
  translation-of:
  generated: false
---


## Lint Documents with Actionable Diagnostics

As a coding agent, I want to lint documents, so that I can detect include, metadata, and ID errors deterministically and stop with actionable diagnostics.

### Acceptance Criteria

- [ ] `arqix lint run` checks include targets for existence.
- [ ] `arqix lint run` reports forbidden frontmatter keys in units according to an allowlist.
- [ ] `arqix lint run` reports duplicate IDs globally across units, requirements, user stories, ADRs, and glossary entries.
- [ ] All configured checks report precise file and line context.
- [ ] Invalid input returns a failing status.
- [ ] Lint output is deterministic across repeated runs on the same input.

### Notes

The lint pass is ready when all configured checks report precise file and line context and return a failing status for invalid input.
Add targeted fixtures for missing includes, forbidden unit metadata keys, and duplicate IDs across document types.
Keep the output deterministic so automation can compare failures reliably and act without guesswork.
The main value for a coding agent is machine-usable diagnostics and clear stop conditions within scope.