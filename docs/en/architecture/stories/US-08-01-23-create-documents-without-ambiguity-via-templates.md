---

id: US-08-01-23
title: Create documents without ambiguity via templates
slug: create-documents-without-ambiguity-via-templates
iri: arqix:user-stories/us-08-01-23

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
  created: 2026-03-30
  updated: 2026-04-04
  lang: en
  translation-of:
  generated: false
---

## User-story

As a Casey Coding Agent, I want to create documents via `doc new <kind>` using deterministic defaults for ID generation and target paths, so that I can generate conforming documents without guessing where they belong or how they should be structured.

### Acceptance Criteria

- [ ] `arqix doc new <kind> --title "<t>"` creates a new document in the configured location for that kind.
- [ ] If no `--id` is provided, arqix generates an ID using configured policy and verifies uniqueness.
- [ ] The created document uses the configured template and includes required frontmatter fields.
- [ ] `--dry-run` reports the planned ID and target path without writing files.

### Notes

In scope are template selection by kind, deterministic ID generation or explicit `--id`, deterministic target path policy, and placeholder substitution for `{id}`, `{title}`, and `{slug}`. Out of scope are interactive prompts and arbitrary free-form template engines. The main value is ambiguity-free document creation for automation.
