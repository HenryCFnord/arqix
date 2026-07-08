---
id: ADR-0001
title: Agent Instruction Document Layout
slug: agent-instruction-document-layout
iri: arqix:adrs/adr-0001

rdf:
  type:
    - arqix:classes/adr

triples:
  - predicate: arqix:properties/guides-design-of
    object:
  - predicate: arqix:properties/guides-implementation-of
    object:
      - arqix:requirements/req-01-01-09-01
      - arqix:requirements/req-01-01-09-02
      - arqix:requirements/req-01-01-09-03
      - arqix:requirements/req-01-01-09-04
      - arqix:requirements/req-01-01-09-05
      - arqix:requirements/req-01-01-09-06
  - predicate: arqix:properties/guides-verification-of
    object:

properties:
  decision-status: accepted

external-references:
  - type: specification
    label: "AGENTS.md: an open standard for agent instructions"
    uri: https://agents.md/

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-02
  updated: 2026-07-02
  lang: en
  translation-of:
  generated: false
---

## Agent Instruction Document Layout

### Context

The user stories and requirements about agent workflows originally hardcoded `AGENTS.md` and `PLANS.md` and assumed the Codex workflow.
In practice, coding agents read different instruction files: Codex, Cursor, Zed, and Gemini CLI read `AGENTS.md` (an open cross-vendor standard), while Claude Code primarily reads `CLAUDE.md` and can reference or import other files.
Extension mechanisms also differ: Claude Code manages skills under `.claude/skills`, Codex uses prompt files, and other agents bring their own conventions.
Stories US-01-01-09 and US-08-01-18 and requirements REQ-01-01-09-01 through -06 are therefore phrased agent-agnostically in terms of two roles: the *agent instruction document* and the *plan document*.
This ADR fixes the concrete file mapping for those roles in this repository.

### Decision

- `AGENTS.md` is the canonical agent instruction document.
  All normative process rules — scope rules for story-by-story execution, editing constraints for the plan document, and the required arqix verification loop — live there and only there.
- `CLAUDE.md` exists as a thin adapter for Claude Code: it refers to `AGENTS.md` for the process contract and carries only Claude-Code-specific notes (for example skill locations).
  It never restates or overrides normative rules.
- Agents that read `AGENTS.md` natively (Codex, Cursor, Zed, Gemini CLI, and others following the standard) need no adapter.
- `PLANS.md` remains the name of the plan document in this repository.
- Agent-specific extension points (Claude Code skills under `.claude/skills`, Codex prompt files, and equivalents of other agents) are documented per supported agent and carry no normative process rules (REQ-01-01-09-06).
  They may automate or assist the process, but the contract they follow is defined in `AGENTS.md`.

### Alternatives Considered

- One full instruction file per agent (`AGENTS.md`, `CLAUDE.md`, … each complete): rejected because duplicated normative rules drift apart and reviews cannot identify the authoritative source.
- `CLAUDE.md` as the canonical document: rejected because it is vendor-specific and would exclude every agent that follows the `AGENTS.md` standard.
- Symlinking `CLAUDE.md` to `AGENTS.md`: viable, but a thin adapter is preferred because Claude-Code-specific notes (skills) need a home that is not part of the canonical contract.

### Consequences

- Supporting a new coding agent means documenting its instruction-file expectation and extension points, and adding a thin adapter only if it does not read `AGENTS.md`.
- Stories, requirements, and arqix tooling stay file-name-neutral; renaming or adding vendor files never touches the requirements layer.
- Reviews can reject any normative process rule found outside `AGENTS.md` by pointing at REQ-01-01-09-06.
