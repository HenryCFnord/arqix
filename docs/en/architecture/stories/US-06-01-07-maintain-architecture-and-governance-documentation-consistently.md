---

id: US-06-01-07
title: Maintain architecture and governance documentation consistently
slug: maintain-architecture-and-governance-documentation-consistently
iri: arqix:user-stories/us-06-01-07

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
  created: 2026-03-30
  updated: 2026-04-04
  lang: en
  translation-of:
  generated: false
---

## User-story

As a Aria Architect, I want to maintain architecture and governance documentation consistently, so that arqix applies its own Documentation-as-Code principles to arc42, ADRs, and the handbook.

### Acceptance Criteria

- [ ] The arc42 architecture document is structured into units per chapter and can be assembled.
- [ ] ADRs are maintained using the path model with a canonical governance language.
- [ ] A multi-layer documentation strategy is used across handbook, CLI help, man page, and rustdoc.
- [ ] Mermaid diagrams are used in a C4-oriented way for views.
- [ ] A future documentation consistency check is documented as an extension path.

### Notes

This story is complete when the governance documents follow the same assembly and maintenance rules expected from product documentation. Add checks or examples that show how arc42 units, ADRs, and handbook layers relate and where each concern belongs. Treat the future consistency check as a documented extension path, not as an implicit requirement for the first delivery. This is the canonical architecture and governance authoring view.
