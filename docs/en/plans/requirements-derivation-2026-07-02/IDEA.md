---
title: "Requirements derivation from user stories"
date: 2026-07-02
status: draft-created
category: docs
branch: docs/add-requirements
source: claude-code
---

# Requirements derivation from user stories - IDEA

This planning package captures the intake for deriving requirement documents from the user stories in `docs/en/architecture/stories` and wiring the `has-requirement` traceability links, on branch `docs/add-requirements`.

## Raw intake

The consistency review of the user stories on branch `docs/add-personas-user-stories` (PR #5) confirmed that all 103 stories are structurally consistent, but every `has-requirement` triple is empty.
The US → REQ traceability chain therefore does not exist yet.
Derive requirements from the user stories, create requirement documents under the new architecture documentation layout, and populate the `has-requirement` triples in the stories.
The historic mapping report `docs/en/plans/persona-remapping-2026-03-25/persona_us_req_mapping_report.md` documents an earlier US/REQ mapping, but uses the superseded ID schemes (`US-1001`, `REQ-US-1001-01`) and superseded paths (`docs/us/`, `docs/req/`), so its content must be remapped to the current `US-XX-YY-ZZ` scheme rather than reused verbatim.

## Intake metadata

- Intake source: `claude-code`
- Created: `2026-07-02`
- Chosen branch: `docs/add-requirements`
- Category: `docs`
- Scope: `docs/en/architecture/stories`, new requirements directory, `docs/en/templates/requirement.tpl.md`
- Related topic: `requirements derivation and US → REQ traceability`

## Package notes

- `IDEA.md` remains the intake artefact for this package.
- Execution detail lives in [PLANS.md](PLANS.md).
- Current operational state lives in [STATUS.md](STATUS.md).
