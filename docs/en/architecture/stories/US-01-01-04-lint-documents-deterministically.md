---
id: US-01-01-04
title: Lint Documents Deterministically
slug: lint-documents-deterministically
iri: arqix:user-stories/us-01-01-04

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-01
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
    object: arqix:workflows/wf-01-01

properties:
  priority: high
  edge-case: false

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-03-29
  updated: 2026-07-02
  lang: en
  translation-of:
  generated: false
---

## Lint Documents Deterministically

As a maintainer, I want to lint documents, so that errors in includes, metadata, and IDs are found early and deterministically.

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
Keep the output deterministic so CI failures are easy to compare and review.
This is a core repository hygiene capability.