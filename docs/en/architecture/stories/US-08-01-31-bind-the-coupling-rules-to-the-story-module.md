---
id: US-08-01-31
title: Bind the Coupling Rules to the Story Module
slug: bind-the-coupling-rules-to-the-story-module
iri: arqix:user-stories/us-08-01-31

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-08
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-08-01-31-01
      - arqix:requirements/req-08-01-31-02
      - arqix:requirements/req-08-01-31-03
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

## Bind the Coupling Rules to the Story Module

As a repository owner, I want the story-workflow coupling rules bound to the story-driven process module, so that a corpus without stories and workflows can select its process instead of inheriting every rule.

### Acceptance Criteria

- [ ] `[process].modules` in `arqix.toml` selects the effective process modules.
- [ ] Without a `[process]` section every module is effective — an unconfigured corpus gates exactly as before.
- [ ] When `[process].modules` is configured without `story-driven`, the coupling rules (US-WF-001, US-PER-001) do not run; with `story-driven` listed they run unchanged.
- [ ] The effective modules bring their shipped vocabulary: a corpus without `docs/ontology` validates `arqix:classes/user-story` exactly when `story-driven` is effective, and a corpus definition of a module IRI overrides the embedded one.
- [ ] A corpus redefinition of a reserved-core IRI with different semantics is ONT-009; an identical re-declaration stays silent.

### Notes

The rules stay code; the module selection switches them (ADR-0017 process profiles).
The persona/workflow vocabulary this lint binds to belongs to the story-driven module, so its rules follow the module.
