---
id: WF-06-01
title: Maintain architecture, ADRs, and glossary
slug: maintain-architecture-adrs-and-glossary
iri: arqix:workflows/wf-06-01

rdf:
  type:
    - arqix:classes/workflow

triples:
  - predicate: arqix:properties/has-primary-persona
    object: arqix:personas/per-06
  - predicate: arqix:properties/has-relevant-persona
    object:

properties:
  goal: Create and maintain architecture docs, ADRs, and glossary terms using templates and stable IDs.
  entry-state: Architecture knowledge, decisions, and terminology need to be documented and kept traceable as the system evolves.
  end-state: Architecture docs, ADRs, and glossary terms are maintained with stable IDs, clear links, and publishable structure.

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

## Maintain Architecture, ADRs, and Glossary

Architecture knowledge and decisions must remain accessible and traceable as the system evolves.
ADRs capture decisions; glossary terms reduce terminology drift.

### Goal

Create and maintain architecture docs, ADRs, and glossary terms using templates and stable IDs.

### Steps

1. Identify a decision, concept, or term that must be documented.
2. Create an ADR or glossary term using `doc new`.
3. Link the document to relevant stories/requirements via IDs.
4. Validate structure and metadata using formatting and linting.
5. Publish the site to validate navigation and readability.

### Outputs

- ADRs with clear consequences and validation
- Atomic glossary terms
- Consistent, publishable architecture docs

### Failure Modes

- Decisions are made but not recorded.
- Glossary is missing and terminology drifts.
- Links are free-text and not traceable.

### Related Commands

- `arqix doc new adr`
- `arqix doc new glossary`
- `arqix fmt`
- `arqix lint run`
- `arqix publish site`
