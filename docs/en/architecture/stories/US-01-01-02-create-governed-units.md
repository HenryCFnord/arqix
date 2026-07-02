---
id: US-01-01-02
title: Create Governed Units
slug: create-governed-units
iri: arqix:user-stories/us-01-01-02

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-01
  - predicate: arqix:properties/has-requirement
    object: arqix:requirements/req-00-00-00-06
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

## Create Governed Units

As a maintainer, I want to create units with governed metadata and uniqueness rules, so that I can keep modular documentation consistent and lintable across the repository.

### Acceptance Criteria

- [ ] `arqix unit new` creates a unit file in the configured unit location.
- [ ] Unit frontmatter is optional and follows repository configuration.
- [ ] Units can carry a global ID, either in frontmatter or via a supported directive.
- [ ] Global unit IDs are linted for uniqueness across all units.
- [ ] The generated unit shape follows repository rules when optional metadata is disabled.
- [ ] The command help explains where units are created, which metadata is optional, and how IDs are supplied.

### Notes

Acceptance should cover both the default unit creation path and the configured variant without frontmatter.
Add tests for unique ID validation across multiple units and for the generated file shape when optional metadata is disabled.
The primary value for a maintainer is repository-wide consistency, metadata governance, and deterministic lintable structure for units.