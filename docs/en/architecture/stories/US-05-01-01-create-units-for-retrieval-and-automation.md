---
id: US-05-01-01
title: Create Units for Retrieval and Automation
slug: create-units-for-retrieval-and-automation
iri: arqix:user-stories/us-05-01-01

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-05
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-00-00-00-06
      - arqix:requirements/req-01-01-02-01
      - arqix:requirements/req-01-01-02-02
      - arqix:requirements/req-01-01-02-03
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-05-01

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

## Create Units for Retrieval and Automation

As an AIOps engineer, I want to create units quickly, so that I can maintain documentation as modular, metadata-aware building blocks for retrieval and downstream automation.

### Acceptance Criteria

- [ ] `arqix unit new` creates a unit file in the configured unit location.
- [ ] Frontmatter for units is optional and follows repository configuration.
- [ ] Units can carry a global ID, either in frontmatter or via a supported directive.
- [ ] Global unit IDs are linted for uniqueness across all units.
- [ ] The command help explains where units are created and how IDs are supplied.

### Notes

Acceptance should cover both the default unit creation path and the configured variant without frontmatter.
Add tests for unique ID validation across multiple units and for the generated file shape when optional metadata is disabled.
Units are first-class retrieval chunks and may carry metadata needed for search, read, and downstream automation.

Retired in the consolidation sweep of 2026-07-11: this story is a persona clone — its non-cross-cutting requirements are canonically owned by US-01-01-02, and the requirements' derived-from provenance keeps this story's contribution on record.
