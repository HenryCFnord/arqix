# OpenClaw Plan Intake Wrapper Contract

## Purpose

`tools/openclaw/plan_from_idea.sh` is the thin repository-side wrapper that turns free-text intake into deterministic planning inputs for `plan_intake.sh`.

It exists so the OpenClaw host can stay small and pass a stable interface into the repository.

## Inputs

Required:

- `--idea <text>`

Optional:

- `--category <category>`
- `--title <title>`
- `--slug <slug>`
- `--source <source>`
- `--base <branch>`
- `--commit`

## Behavior

When category is missing:

- derive a short title from the first non-empty idea line
- derive a kebab-case slug
- print `needs_category=true`
- print a short follow-up prompt
- do not create files or branches

When category is provided:

- derive missing title or slug deterministically
- print the chosen branch name
- delegate to `tools/openclaw/plan_intake.sh`
- preserve the same clean-worktree and no-overwrite rules

## Output shape

The wrapper prints stable `key=value` lines.

Typical keys:

- `needs_category`
- `suggested_title`
- `suggested_slug`
- `allowed_categories`
- `follow_up_prompt`
- `branch`
- all `plan_intake.sh` output keys when intake proceeds

## Out of scope

This wrapper does not:

- guess ambiguous category values silently
- run Codex
- create PRs
- send notifications
