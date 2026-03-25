---
id: US-8002
kind: user_story
title: Document creation without ambiguity via templates
status: draft
workflows:
- WF-0008
story_type: capability
persona: PER-0008
old_id: US-8003
related:
  requirements: []
  personas:
  - PER-0008
---
# US-8002 — Document creation without ambiguity via templates

As an Automation Agent, I want to create documents via `doc new <kind>` using deterministic defaults for ID generation and target paths, so that I can generate conforming documents without guessing where they belong or how they should be structured.

## Scope

In scope:
- `doc new <kind>` supports:
  - template selection by `kind`
  - deterministic ID generation policy (or explicit `--id`)
  - deterministic target path policy (doc-package routing)
  - placeholder substitution `{id}`, `{title}`, `{slug}`

Out of scope:
- Interactive prompts
- Arbitrary free-form template engines

## Acceptance Criteria

- `doc new <kind> --title "<t>"` creates a new document in the configured location for that kind.
- If no `--id` is provided, arqix generates an ID using configured policy and verifies uniqueness.
- The created document uses the configured template and has required frontmatter fields present.
- `--dry-run` reports planned ID and target path without writing files.
