---
id: WF-xx-yy
title:
slug:
iri:

rdf:
  type:
    - arqix:classes/workflow

triples:
  - predicate: arqix:properties/has-primary-persona
    object:
  - predicate: arqix:properties/has-relevant-persona
    object:

properties:
  goal:
  entry-state:
  end-state:

external-references: []

meta:
  lifecycle-status: draft
  owner:
  created:
  updated:
  lang: en
  translation-of:
  generated: false
---

## Workflow

<!-- Introduction: Explain the repository problem, context, or user need that makes this workflow necessary. Keep it short and specific. -->

Describe the end-to-end workflow here.

<!-- Goal: State the desired outcome of the workflow in one or two sentences. -->

### Goal

Describe the goal here.

<!-- Steps: List the ordered actions needed to achieve the goal. -->

### Steps

1. Describe step 1 here.
2. Describe step 2 here.
3. Describe step 3 here.

<!-- Outputs: List the concrete artefacts, docs, or system changes produced by the workflow. -->

### Outputs

- Describe output 1 here.
- Describe output 2 here.
- Describe output 3 here.

<!-- Failure Modes: Capture common ways this workflow can fail or produce weak results. -->

### Failure Modes

- Describe failure mode 1 here.
- Describe failure mode 2 here.
- Describe failure mode 3 here.

<!-- Related Commands: Document the commands, tools, or workflows that support this workflow. -->

### Related Commands

- `arqix config validate`
- `arqix doc new <kind>`
- `arqix fmt`
- `arqix lint run`
- `arqix trace scan`

<!-- Automation (optional): Mention just recipes, scripts, or agent workflows that can automate parts of this process. -->

### Automation (optional)

- Describe automation here.
