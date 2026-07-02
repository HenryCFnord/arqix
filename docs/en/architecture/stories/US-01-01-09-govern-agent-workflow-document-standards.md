---
id: US-01-01-09
title: Govern Agent Workflow Document Standards
slug: govern-agent-workflow-document-standards
iri: arqix:user-stories/us-01-01-09

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-01
  - predicate: arqix:properties/has-requirement
    object:
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-01-01

properties:
  priority: medium
  edge-case: false

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-03-30
  updated: 2026-04-05
  lang: en
  translation-of:
  generated: false
---

## Govern Agent Workflow Document Standards

As a maintainer, I want AGENTS.md and PLANS.md to follow a standardized structure and editing policy, so that automation workflows remain predictable, reviewable, and aligned with repository standards.

### Acceptance Criteria

- [ ] AGENTS.md defines scope rules for story-by-story execution, including one story at a time and no opportunistic refactors.
- [ ] AGENTS.md defines editing constraints for `PLANS.md` and the required arqix verification loop.
- [ ] PLANS.md includes story tasks with scope in/out, acceptance criteria, required command checks, and status fields or checkboxes that agents may update.
- [ ] The document structures are explicit enough that an agent can follow them without guessing process constraints.

### Notes

In scope are the minimal structures for `AGENTS.md` and `PLANS.md`, clear editing constraints for agents, and a workflow contract for story-by-story execution.
Out of scope are automatic enforcement or generation of these documents. The main value for a maintainer is governed process documentation and predictable automation conventions.
