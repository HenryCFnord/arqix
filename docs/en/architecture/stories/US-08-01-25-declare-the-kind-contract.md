---
id: US-08-01-25
title: Declare the Kind Contract
slug: declare-the-kind-contract
iri: arqix:user-stories/us-08-01-25

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-08
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-08-01-25-01
      - arqix:requirements/req-08-01-25-02
      - arqix:requirements/req-08-01-25-03
      - arqix:requirements/req-08-01-25-04
      - arqix:requirements/req-08-01-25-05
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-08-01

properties:
  priority: medium
  edge-case: false

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-15
  updated: 2026-07-19
  lang: en
  translation-of:
  generated: false
---

## Declare the Kind Contract

As a repository owner, I want a document kind to declare where its documents live, how they are templated, and how their ids and paths derive, so that correct creation is configuration instead of per-call discipline.

### Acceptance Criteria

- [ ] When `[kinds.<family>]` declares a `dir`, `arqix doc new <family>` creates (and `--dry-run` plans) the document under that directory.
- [ ] Without a declared contract, placement stays `<first-root>/<kind>/` — an unconfigured repository behaves exactly as before.
- [ ] Creation and validation read the same declared `dir` (one source, ADR-0011): a document created for a configured family passes that family's directory-based checks in place.
- [ ] `[kinds.<family>].template` names the template file, and an unknown placeholder is a TPL-002 finding, never a silent literal.
- [ ] `[kinds.<family>].id-template` and `dir-template` derive the id and the target directory from `--set` values and the slug; `--id` and `--dir` stay manual overrides, and kinds without the keys behave exactly as before.

### Notes

First slice of the authoring-ergonomics band from the knowledge-repository intake (`docs/en/plans/knowledge-repository-2026-07-15/`, gap G2).
Today the declared `dir` is used for validation only (US-01-01-19); creation ignores it, so `doc new` output for configured families must be moved by hand before the checkers accept it.
