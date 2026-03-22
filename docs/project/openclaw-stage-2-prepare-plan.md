# OpenClaw Stage 2 Prepare Plan

## Summary

Stage 2 adds a `prepare` capability that starts with one conservative execution step.

It should:

- validate whether the local repository is ready for the next scoped task
- read an approved handoff and derive deterministic preparation outputs
- create or switch to the task branch derived from the handoff
- stop before issue creation, PR creation, Codex execution, or any delivery action

This keeps the OpenClaw integration lightweight and aligned with the repository workflow in `AGENTS.md` and `CONTRIBUTING.md`.

## Recommended Architecture

The OpenClaw integration remains split into three stages:

1. `readonly`
2. `prepare`
3. `execute / delivery`

The role of stage 2 is to bridge repository inspection and later operational execution.

It should consume facts already exposed by the readonly stage, then produce preparation outputs that a human or a later stage-3 workflow can review and execute.

For the first implementation slice, stage 2 stays shell-based, deterministic, and limited to safe local branch setup.

## File Plan

The first stage-2 implementation should add:

- `.agents/skills/arqix-repo-prepare/SKILL.md`
- `tools/openclaw/prepare_validate_env.sh`
- `tools/openclaw/prepare_branch_name.sh`
- `tools/openclaw/prepare_issue_from_handoff.sh`
- `tools/openclaw/create_branch_from_handoff.sh`

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

### `create_branch_from_handoff.sh`

Purpose:

- read a handoff path from the repository
- select the handoff branch if explicitly provided
- otherwise derive a deterministic branch name from handoff metadata
- create the branch if missing
- switch to the selected branch
- print a concise machine-friendly summary

Behavior:

- require execution from the `arqix` repository root
- fail if the worktree is dirty
- fail if the handoff is missing or does not yield a usable branch name
- reject `main`
- reuse `prepare_branch_name.sh` instead of duplicating derivation logic
- print stable `key=value` output for later OpenClaw summarization

## Connection to Readonly

Stage 2 should connect to the existing readonly stage in these ways:

- reuse `tools/openclaw/lib.sh` for repository-root and shell safety helpers
- rely on the same repository assumptions as `repo_status.sh`
- treat `docs/handoffs/approved/` as the canonical handoff directory when available
- preserve skill separation:
  - readonly inspects
- prepare validates, derives, and performs safe local branch setup
- execute performs later actions

## Explicitly Out of Scope

The first stage-2 implementation must not include:

- issue creation
- PR creation
- merge or delivery automation
- Signal integration
- broad OpenClaw host management changes
- stage-3 execution behavior

## Assumptions That Need Validation

- approved handoffs will live under `docs/handoffs/approved/`
- the current handoff template is stable enough for shell-based extraction
- a dirty worktree should block branch creation
- printing preparation outputs to `stdout` is sufficient for the first slice

## Suggested Implementation Order

1. Add this planning document.
2. Add the `arqix-repo-prepare` skill.
3. Extend `tools/openclaw/lib.sh` with shared helpers.
4. Implement `prepare_validate_env.sh`.
5. Implement `prepare_branch_name.sh`.
6. Implement `prepare_issue_from_handoff.sh`.
7. Implement `create_branch_from_handoff.sh`.
8. Update repository docs only where the new prepare capability must be discoverable.

## Risks and Open Questions

- Shell parsing will become fragile if the handoff shape drifts significantly.
- The GitHub issue template may need normalization before future automation uses it directly.
- Stage 3 should consume stage-2 outputs, but the exact execution interface is intentionally deferred.

## Smallest Useful Next Task

The smallest useful follow-up after this scaffold is:

- create one approved handoff under `docs/handoffs/approved/`
- run `create_branch_from_handoff.sh` against that handoff
- confirm the branch output and checkout behavior are stable
- then wire the issue-draft output into later `gh` automation without combining the steps
