---
id: US-8004
kind: user_story
title: Standardize agent workflow documents (AGENTS.md and PLANS.md)
status: draft
workflows:
- WF-0008
- WF-0001
story_type: operational
persona: PER-0008
old_id: US-8006
related:
  requirements: []
  personas:
  - PER-0008
---
# US-8004 — Standardize agent workflow documents (AGENTS.md and PLANS.md)

As an Automation Agent, I want AGENTS.md and PLANS.md to follow a standardized structure and clear editing rules, so that I can reliably execute tasks story-by-story without rewriting planning documents or guessing process constraints.

## Scope

In scope:
- Define the minimal structure for AGENTS.md and PLANS.md
- Define editing constraints for agents (what may be changed)
- Provide a workflow contract for story-by-story execution

Out of scope:
- Tooling that automatically enforces or generates these documents (may come later)

## Acceptance Criteria

- AGENTS.md defines:
  - scope rules (one story at a time)
  - no opportunistic refactors
  - editing constraints for PLANS.md
  - required arqix command loop (fmt/lint/trace/coverage)
- PLANS.md includes story tasks with:
  - scope in/out
  - acceptance criteria
  - required command checks
  - status fields/checkboxes that agents may update
