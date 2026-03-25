---
id: US-6001
kind: user_story
title: Architektur/Governance/Doku-Prozess
status: draft
tags:
- user-story
owner: hendrik
created: 2026-02-22
updated: 2026-02-22
priority: medium
related:
  requirements:
  - REQ-US-6001-01
  - REQ-US-6001-02
  - REQ-US-6001-03
  - REQ-US-6001-04
  - REQ-US-6001-05
  docs: []
  adrs: []
  personas:
  - PER-0006
lang: en
translation_of: US-6001
translation_status: draft
generated: false
source:
persona: PER-0006
old_id: US-0022
---
# Architektur/Governance/Doku-Prozess

## Story
As a maintainer, I want to maintain architecture and governance documentation (arc42, ADRs, handbook) consistently, so that darcy applies its own Documentation-as-Code principles.

## Acceptance Criteria
- The arc42 architecture document is structured into units per chapter and can be assembled.
- ADRs are maintained using the path model with a canonical governance language.
- A multi-layer documentation strategy is used (handbook, CLI help, man page, rustdoc).
- Mermaid diagrams are used in a C4-oriented way for views.
- A future documentation consistency check is planned as an extension path.

## Notes
TODO
