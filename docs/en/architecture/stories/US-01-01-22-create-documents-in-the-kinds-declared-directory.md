---
id: US-01-01-22
title: Create Documents in the Kinds Declared Directory
slug: create-documents-in-the-kinds-declared-directory
iri: arqix:user-stories/us-01-01-22

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-08
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-01-01-22-01
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-01-01

properties:
  priority: medium
  edge-case: false

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-15
  updated: 2026-07-15
  lang: en
  translation-of:
  generated: false
---

## Create Documents in the Kinds Declared Directory

As a coding agent, I want `doc new <kind>` to create the document in the directory the kind's contract declares, so that a bounded context's documents land where its validation expects them instead of in a generic `<root>/<kind>/` folder.

### Acceptance Criteria

- [ ] When `[kinds.<family>]` declares a `dir`, `arqix doc new <family>` creates (and `--dry-run` plans) the document under that directory.
- [ ] Without a declared contract, placement stays `<first-root>/<kind>/` — an unconfigured repository behaves exactly as before.
- [ ] Creation and validation read the same declared `dir` (one source, ADR-0011): a document created for a configured family passes that family's directory-based checks in place.

### Notes

First slice of the authoring-ergonomics band from the knowledge-repository intake (`docs/en/plans/knowledge-repository-2026-07-15/`, gap G2).
Today the declared `dir` is used for validation only (US-01-01-19); creation ignores it, so `doc new` output for configured families must be moved by hand before the checkers accept it.
