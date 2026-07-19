---
id: US-08-01-33
title: Derive Placement and Id From Kind Templates
slug: derive-placement-and-id-from-kind-templates
iri: arqix:user-stories/us-08-01-33

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-08
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-08-01-33-01
      - arqix:requirements/req-08-01-33-02
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-08-01

properties:
  priority: medium
  edge-case: false

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-19
  updated: 2026-07-19
  lang: en
  translation-of:
  generated: false
---

## Derive Placement and Id From Kind Templates

As a repository owner, I want a kind to declare how its ids and target paths derive from creation arguments, so that the correct name and place are the default instead of a per-call chore.

### Acceptance Criteria

- [ ] `[kinds.<family>].id-template` mints the id from `--set` values and the derived slug (for example `{context}-{slug}`); an uncovered placeholder is an error naming it, and the minted id passes the same validity and uniqueness checks as an explicit `--id`.
- [ ] `[kinds.<family>].dir-template` derives the target directory the same way (for example `contexts/{context}/terms`), containment-guarded like an explicit `--dir`.
- [ ] `--id` and `--dir` stay manual overrides, and a kind without the template keys behaves exactly as before.

### Notes

Extends the declared-kind contract (ADR-0012 id policy, ADR-0017 process profiles): the derivation is data on the kind, the mechanics stay code.
A second arqix-governed corpus names this the per-term chore — every creation call carries `--dir` and `--id` by hand (FR-B2 in the second intake of `docs/en/plans/knowledge-repository-2026-07-15/`).
