---
id: US-08-01-18
title: Standardize Agent Workflow Documents
slug: standardize-agent-workflow-documents
iri: arqix:user-stories/us-08-01-18

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-08
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-01-01-09-01
      - arqix:requirements/req-01-01-09-02
      - arqix:requirements/req-01-01-09-03
      - arqix:requirements/req-01-01-09-04
      - arqix:requirements/req-01-01-09-05
      - arqix:requirements/req-01-01-09-06
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-08-01

properties:
  priority: medium
  edge-case: false

external-references: []

meta:
  lifecycle-status: retired
  owner: hcf
  created: 2026-03-30
  updated: 2026-07-11
  lang: en
  translation-of:
  generated: false
---

## Standardize Agent Workflow Documents

As a coding agent, I want the agent instruction and plan documents to follow a standardized structure and clear editing rules, so that I can execute tasks story-by-story without rewriting planning documents or guessing process constraints.

### Acceptance Criteria

- [ ] The agent instruction document defines scope rules for story-by-story execution, including one story at a time and no opportunistic refactors.
- [ ] The agent instruction document defines editing constraints for the plan document and the required arqix verification loop.
- [ ] The plan document includes story tasks with scope in/out, acceptance criteria, required command checks, and status fields or checkboxes that agents may update.
- [ ] The document structures are explicit enough that an agent can follow them without guessing process constraints.
- [ ] Agent-specific extension points such as skills or prompt libraries are documented per supported agent and carry no normative process rules.

### Notes

In scope are the minimal structures for the agent instruction and plan documents, clear editing constraints for agents, and a workflow contract for story-by-story execution.
Which concrete files serve these roles per coding agent (for example `AGENTS.md` as the canonical instruction document with `CLAUDE.md` as an adapter, and `PLANS.md` as the plan document) is decided in ADR-0001; the story is intentionally agent-agnostic.
Out of scope are automatic enforcement or generation of these documents.
The main value here is a stable execution contract for agent workflows.

Retired in the consolidation sweep of 2026-07-11: this story is a persona clone — its non-cross-cutting requirements are canonically owned by US-01-01-09, and the requirements' derived-from provenance keeps this story's contribution on record.
