---
id: property-has-persona
label: has-persona
iri: arqix:properties/has-persona

rdf:
  type:
    - rdf:Property

rdfs:
  domain:
    - arqix:classes/user-story
  range:
    - arqix:classes/persona

owl:
  inverse-of: arqix:properties/is-persona-for-story

triples: []

properties: {}

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-03-27
  updated: 2026-07-15
  lang: en
  generated: false
---

## Has-persona

Relates a user story to one or more personas.

### Workflow membership convention

A story belongs to the workflow whose declared personas (`has-primary-persona`, `has-relevant-persona`) include the story's persona, and the story ID encodes that workflow (`US-<WW>-<SS>-<NN>` sits in `WF-WW-SS`).
The consolidation personas — declared by `consolidation: true` in the persona document's properties, today PER-09 and PER-10 — are the exception: they bundle several viewpoints, so their stories attach directly to the workflow their content belongs to, and the persona does not govern the workflow membership.
`arqix lint requirements` enforces the convention (US-WF-001, US-PER-001; REQ-01-01-11-08 and REQ-01-01-11-09).
