---


id: US-06-01-01
title: Create Modular Document Units
slug: create-modular-document-units
iri: arqix:user-stories/us-06-01-01

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-06
  - predicate: arqix:properties/has-requirement
    object:
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-06-01

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


## Create Modular Document Units

As an architect, I want to create units quickly, so that I can maintain documentation modularly and consistently in small building blocks.

### Acceptance Criteria

- [ ] `arqix unit new` creates a unit file in the configured unit location.
- [ ] Frontmatter for units is optional and follows repository configuration.
- [ ] Units can carry a global ID, either in frontmatter or via a supported directive.
- [ ] Global unit IDs are linted for uniqueness across all units.
- [ ] The command help explains where units are created and how IDs are supplied.

### Notes

Acceptance should cover both the default unit creation path and the configured variant without frontmatter.
Add tests for unique ID validation across multiple units and for the generated file shape when optional metadata is disabled.
Units are used as modular building blocks for chapters, sections, and reusable document composition.