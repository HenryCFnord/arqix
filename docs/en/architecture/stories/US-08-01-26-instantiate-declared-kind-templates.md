---
id: US-08-01-26
title: Instantiate Declared Kind Templates
slug: instantiate-declared-kind-templates
iri: arqix:user-stories/us-08-01-26

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-08
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-08-01-26-01
      - arqix:requirements/req-08-01-26-02
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
  updated: 2026-07-15
  lang: en
  translation-of:
  generated: false
---

## Instantiate Declared Kind Templates

As a coding agent, I want a kind's contract to name its template file and the placeholder vocabulary to be validated, so that a bounded context's custom templates are first-class configuration instead of magic filenames, and a typo in a placeholder is a finding instead of a silent literal in the created document.

### Acceptance Criteria

- [ ] When `[kinds.<family>]` declares a `template`, `doc new <family>` instantiates that file (repository-relative path); a missing declared template is a configuration error.
- [ ] A declared template's placeholders are validated against the documented vocabulary (`{id}`, `{title}`, `{slug}`, `{iri_slug}`, `{kind}`, `{namespace}`, `{lifecycle}`): an unknown `{placeholder}` is a finding and nothing is written.
- [ ] Without a declared `template`, resolution stays exactly as today (`[templates] dir` or the package-local scaffold, then the embedded default) and no placeholder validation is added to that path.

### Notes

Second slice of the authoring-ergonomics band from the knowledge-repository intake (`docs/en/plans/knowledge-repository-2026-07-15/`, gap G1, plan slice K1).
Today custom templates work only by dropping a `<kind>.tpl.md` into the one template directory, and an unknown placeholder survives substitution as literal text.
