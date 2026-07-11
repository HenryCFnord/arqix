---
id: US-02-01-04
title: Lint Documents Before Commit
slug: lint-documents-before-commit
iri: arqix:user-stories/us-02-01-04

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-02
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-00-00-00-01
      - arqix:requirements/req-00-00-00-04
      - arqix:requirements/req-00-00-00-06
      - arqix:requirements/req-01-01-04-01
      - arqix:requirements/req-01-01-04-02
      - arqix:requirements/req-01-01-04-03
      - arqix:requirements/req-01-01-04-04
      - arqix:requirements/req-01-01-04-05
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-02-01

properties:
  priority: high
  edge-case: false

external-references: []

meta:
  lifecycle-status: retired
  owner: hcf
  created: 2026-03-29
  updated: 2026-07-11
  lang: en
  translation-of:
  generated: false
---

## Lint Documents Before Commit

As a developer, I want to lint documents, so that I can catch include, metadata, and ID errors before opening a PR.

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
Keep the output deterministic so local failures are easy to compare with CI and fix quickly.
The main value for a developer is fast feedback in the normal implementation workflow.

Retired in the consolidation sweep of 2026-07-11: this story is a persona clone — its non-cross-cutting requirements are canonically owned by US-01-01-04, and the requirements' derived-from provenance keeps this story's contribution on record.
