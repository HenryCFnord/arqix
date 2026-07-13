---
id: US-04-01-18
title: Gate Architecture-View Derivation
slug: gate-architecture-view-derivation
iri: arqix:user-stories/us-04-01-18

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-09
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-04-01-18-01
      - arqix:requirements/req-04-01-18-02
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-04-01

properties:
  priority: medium
  edge-case: false

external-references: []

meta:
  lifecycle-status: done
  owner: hcf
  created: 2026-07-13
  updated: 2026-07-13
  lang: en
  translation-of:
  generated: false
---

## Gate Architecture-View Derivation

As a builder, I want the committed C4 Mermaid views checked against the C4 model, so that a diagram cannot silently drift from `workspace.dsl` the way the C4 audit found drift by hand.

### Acceptance Criteria

- [x] `arqix lint run` reports a finding when a Mermaid view marked as derived from the C4 model contains an element the referenced model view does not define.
- [x] `arqix lint run` reports a finding when a derived view contains a relationship the model does not justify, accounting for Structurizr's implied container-view edges (a system-level edge pushed down to a container).
- [x] The check compares topology — element identity by display name and kind, and relationship endpoints — not the hand-abbreviated ids, labels, or descriptions.
- [x] The two current derived views (SystemContext, Containers) pass unchanged.

### Notes

The check is in-process and structural (ADR-0016): it matches diagram elements to model elements by their display name, tolerating the diagrams' shortened ids (`render` for `renderToolchain`), and validates relationship endpoints against the model rather than the reworded labels.
It deliberately does not regenerate the diagrams or invoke `structurizr-cli`: it keeps the hand-authored views and adds the missing gate (ADR-0002 named the single source and the derived-from marker; ADR-0016 records why the first slice checks rather than generates).
The full DSL-to-Mermaid generator (auto-producing the views and gating on an exact match) is a later slice, not this one.
