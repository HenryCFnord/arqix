---
id: PER-06
title: Aria Architect
slug: aria-architect
iri: arqix:personas/per-06

rdf:
  type:
    - arqix:classes/persona

triples: []

properties:
  role: Architect documenting system decisions and terminology
  description: Documents architecture, ADRs, glossary terms, and architectural narratives with a focus on clarity, structure, and traceable reasoning.

external-references: []

meta:
  lifecycle-status: retired
  owner: hcf
  created: 2026-03-05
  updated: 2026-07-12
  lang: en
  translation-of:
  generated: false
---

## Aria Architect

Aria documents architecture, decisions, and vocabulary.
She cares about clarity, structure, and traceable reasoning.
Aria wants documentation to remain useful as the system evolves.

### Goals

- Capture decisions in ADRs with clear consequences.
- Keep architectural documentation modular and navigable.
- Maintain consistent terminology via glossary terms.
- Link architecture to requirements and stories via IDs.

### Success Looks Like

- ADRs are easy to create and reference.
- Handbooks and architecture docs assemble cleanly.
- Glossary prevents terminology drift.
- Key documents are publishable and reviewable.

### Pain Points

- Architecture knowledge scattered across chats and tickets.
- Decisions made but not recorded.
- Inconsistent terminology across documents.
- Tooling friction that interrupts writing flow.

### Typical Workflow with arqix

Aria creates ADRs and glossary terms from templates, links them to relevant requirements and stories, and uses assembling and publishing to validate the full documentation narrative.

### Important arqix Capabilities and Commands

- `doc new adr`
- `doc new glossary`
- `assemble build`
- `fmt`
- `lint run`
- `publish site` (to verify docs are navigable)

### artefacts They Care About

- ADRs and architectural documents
- Glossary term files
- Handbook pages
- Published site outputs

### Boundaries

Aria does not own CI pipelines or code-level trace markers, but depends on them for evidence and consistency.

### Open Needs

Aria benefits from stable templates, clear markup rules, and consistent cross-language handling.

Retired in the persona merge of 2026-07-12: this viewpoint is carried by PER-09 (Builder), which consolidates the user-side personas while the maintainer and the coding agent stay dedicated.
