---


id: US-02-01-07
title: Create Conforming Documents Quickly via Templates
slug: create-conforming-documents-quickly-via-templates
iri: arqix:user-stories/us-02-01-07

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-02
  - predicate: arqix:properties/has-requirement
    object:
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-02-01

properties:
  priority: high
  edge-case: false

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-03-30
  updated: 2026-04-04
  lang: en
  translation-of:
  generated: false
---


## Create Conforming Documents Quickly via Templates

As a Dev Dan, I want to create documents via `doc new <kind>` using deterministic defaults, so that I can add conforming artefacts quickly without manual setup or routing guesswork.

### Acceptance Criteria

- [ ] `arqix doc new <kind> --title "<t>"` creates a new document in the configured location for that kind.
- [ ] If no `--id` is provided, arqix generates an ID using configured policy and verifies uniqueness.
- [ ] The created document uses the configured template and includes required frontmatter fields.
- [ ] `--dry-run` reports the planned ID and target path without writing files.

### Notes

In scope are template selection by kind, deterministic ID generation or explicit `--id`, deterministic target path policy, and placeholder substitution for `{id}`, `{title}`, and `{slug}`. Out of scope are interactive prompts and arbitrary free-form template engines. The main value for Dan is low-friction creation of compliant artefacts during implementation.
