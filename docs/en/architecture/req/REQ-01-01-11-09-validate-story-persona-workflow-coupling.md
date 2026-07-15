---
id: REQ-01-01-11-09
title: Validate Story Persona Workflow Coupling
slug: validate-story-persona-workflow-coupling
iri: arqix:requirements/req-01-01-11-09

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-11
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: A story whose persona is missing from its workflow's declared personas makes the check exit non-zero and names the persona and workflow, unless the persona document carries consolidation true.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-15
  updated: 2026-07-15
  lang: en
  translation-of:
  generated: false
---

## Requirement

When `arqix lint requirements` runs, arqix SHALL report every user story that names a persona which is neither declared on the story's workflow nor marked as a consolidation persona.

### Notes

The workflow declares its personas via `has-primary-persona` and `has-relevant-persona`; the membership convention lives on the ontology property document `has-persona`.
The exemption is data, not code: a persona document with `consolidation: true` in its properties (today PER-09 and PER-10) bundles several viewpoints, so its stories attach to the workflow their content belongs to.
The check reads the story's declared workflow, so an unresolvable workflow reference is out of scope here (graph-resolution linting owns that concern).
