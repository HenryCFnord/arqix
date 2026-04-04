---



id: US-01-01-13
title: Govern Deterministic Document Creation via Templates
slug: govern-deterministic-document-creation-via-templates
iri: arqix:user-stories/us-01-01-13

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-01
  - predicate: arqix:properties/has-requirement
    object:
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-01-01

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


## Govern Deterministic Document Creation via Templates

As a maintainer, I want document creation to use deterministic template, ID, and routing rules, so that new artefacts remain compliant from the moment they are created.

### Acceptance Criteria

- [ ] `arqix doc new <kind> --title "<t>"` creates a new document in the configured location for that kind.
- [ ] If no `--id` is provided, arqix generates an ID using configured policy and verifies uniqueness.
- [ ] The created document uses the configured template and includes required frontmatter fields.
- [ ] `--dry-run` reports the planned ID and target path without writing files.

### Notes

In scope are template selection by kind, deterministic ID generation or explicit `--id`, deterministic target path policy, and placeholder substitution for `{id}`, `{title}`, and `{slug}`. Out of scope are interactive prompts and arbitrary free-form template engines. The main value for Mara is configuration-driven compliance from the start.
