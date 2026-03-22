# OpenClaw Stage 2 Prepare Plan

## Summary

Stage 2 adds a `prepare` capability that stays non-executing.

It should:

- validate whether the local repository is ready for the next scoped task
- read an approved handoff and derive deterministic preparation outputs
- stop before branch creation, issue creation, or any delivery action

This keeps the OpenClaw integration lightweight and aligned with the repository workflow in `AGENTS.md` and `CONTRIBUTING.md`.

## Recommended Architecture

The OpenClaw integration remains split into three stages:

1. `readonly`
2. `prepare`
3. `execute / delivery`

The role of stage 2 is to bridge repository inspection and later operational execution.

It should consume facts already exposed by the readonly stage, then produce preparation outputs that a human or a later stage-3 workflow can review and execute.

For the first implementation slice, stage 2 stays shell-based and deterministic.

## File Plan

The first stage-2 implementation should add:

- `.agents/skills/arqix-repo-prepare/SKILL.md`
- `tools/openclaw/prepare_validate_env.sh`
- `tools/openclaw/prepare_branch_name.sh`
- `tools/openclaw/prepare_issue_from_handoff.sh`

It may also extend:

- `tools/openclaw/lib.sh`

Documentation should live here:

- `docs/project/openclaw-stage-2-prepare-plan.md`

## Script Breakdown

### `prepare_validate_env.sh`

Purpose:

- confirm the script is running from the `arqix` repository root
- confirm that `git`, `gh`, and `codex` are available
- fail on `main`
- report whether the worktree is clean
- confirm the issue template exists
- report whether the approved handoff directory exists

The script should not create files, create branches, or call external services.

### `prepare_branch_name.sh`

Purpose:

- derive a deterministic branch proposal for a task

Supported modes:

- explicit `--type` plus `--slug`
- `--handoff <path>`

Behavior:

- map handoff types to the repository branch prefixes
- normalize the slug to lowercase kebab-case
- print only the proposed branch name

The script should not create a branch.

### `prepare_issue_from_handoff.sh`

Purpose:

- render a GitHub issue draft from a handoff document

Behavior:

- read a handoff path from the repository
- extract core fields and sections
- print a draft issue body aligned with `.github/ISSUE_TEMPLATE/task-from-handoff.md`
- include a deterministic branch proposal

The script should not call `gh issue create`.

## Connection to Readonly

Stage 2 should connect to the existing readonly stage in these ways:

- reuse `tools/openclaw/lib.sh` for repository-root and shell safety helpers
- rely on the same repository assumptions as `repo_status.sh`
- treat `docs/handoffs/approved/` as the canonical handoff directory when available
- preserve skill separation:
  - readonly inspects
  - prepare validates and drafts
  - execute performs later actions

## Explicitly Out of Scope

The first stage-2 implementation must not include:

- branch creation
- issue creation
- PR creation
- merge or delivery automation
- Signal integration
- broad OpenClaw host management changes
- stage-3 execution behavior

## Assumptions That Need Validation

- approved handoffs will live under `docs/handoffs/approved/`
- the current handoff template is stable enough for shell-based extraction
- a dirty worktree should warn, not block, during prepare
- printing preparation outputs to `stdout` is sufficient for the first slice

## Suggested Implementation Order

1. Add this planning document.
2. Add the `arqix-repo-prepare` skill.
3. Extend `tools/openclaw/lib.sh` with shared helpers.
4. Implement `prepare_validate_env.sh`.
5. Implement `prepare_branch_name.sh`.
6. Implement `prepare_issue_from_handoff.sh`.
7. Update repository docs only where the new prepare capability must be discoverable.

## Risks and Open Questions

- `docs/handoffs/approved/` now exists, but it does not yet contain an approved handoff to exercise the flow end to end.
- Shell parsing will become fragile if the handoff shape drifts significantly.
- The GitHub issue template may need normalization before future automation uses it directly.
- Stage 3 should consume stage-2 outputs, but the exact execution interface is intentionally deferred.

## Smallest Useful Next Task

The smallest useful follow-up after this scaffold is:

- create one approved handoff under `docs/handoffs/approved/`
- run `prepare_validate_env.sh`
- run `prepare_issue_from_handoff.sh` against that handoff
- review the generated issue draft before any `gh` automation is added
