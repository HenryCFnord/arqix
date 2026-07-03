---
id: US-02-01-01
title: Initialize a Doc Package with One Command
slug: initialize-doc-package-with-one-command
iri: arqix:user-stories/us-02-01-01

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-02
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-00-00-00-04
      - arqix:requirements/req-00-00-00-08
      - arqix:requirements/req-01-01-01-01
      - arqix:requirements/req-01-01-01-02
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-02-01

properties:
  priority: medium
  edge-case: true

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-03-28
  updated: 2026-07-02
  lang: en
  translation-of:
  generated: false
---

## Initialize a Doc Package with One Command

As a developer, I want to initialize a new doc package with one command, so that I can start documenting a feature without manual setup.

### Acceptance Criteria

- [ ] `arqix doc init <path>` creates a ready-to-use documentation scaffold in the target path.
- [ ] The scaffold includes `index.md`, `units/`, `pages/`, `artefacts/`, `logs/`, and `.arqix/`.
- [ ] The generated `index.md` already contains valid frontmatter with deterministic `id` and `slug` values derived from the title.
- [ ] The command refuses to overwrite existing files unless explicit approval or force behavior is provided.
- [ ] The resulting structure can be used immediately in the local documentation workflow without manual cleanup.

### Notes

This is a workflow-adjacent enabling story for writing docs alongside implementation.
The primary value for a developer is speed and low friction, not repository governance.
Local usability matters: the initialized package should be immediately compatible with `assemble build`, `fmt`, and `lint run`.
