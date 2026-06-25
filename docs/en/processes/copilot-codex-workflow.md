---
title: "Copilot PR Review → Codex Update workflow"
date: 2026-03-25
status: draft
---

# Copilot PR Review → Codex Update (for arqix)

This document describes the review/repair workflow used in this repository.

Goal
- Convert automated review comments (e.g. Copilot) into actionable tasks, have Codex attempt safe, reversible fixes, and update the PR with explicit commits addressing each comment.

Actors
- Copilot: automated reviewer that produces review comments on PRs
- Codex: coding agent used to implement fixes and produce commits
- gh (GitHub CLI): used to manage PRs and post comments

High‑level flow
1. Inspect PR and collect review comments (`gh pr view <num> --json reviews` and `gh api repos/<owner>/<repo>/pulls/<num>/comments`).
2. Convert comments into a prioritized TODO list with file/path and suggested change.
3. For each TODO entry:
   - Run Codex (or similar coding agent) in a safe working directory (this repository, on a dedicated branch) to implement the fix.
   - Create a focused commit per TODO and push to the PR branch.
   - Post a PR comment summarising the change and referencing the commit.
4. If Codex cannot be used (missing CLI or auth), the assistant falls back to human-reviewed manual edits.

Guidelines
- Never push directly to `main`. Use a dedicated PR branch (here: `chore/ai-setup`).
- Make one commit per review comment when practical; keep commit messages short and traceable.
- Prefer small, easily reviewed commits over large automated refactors.
- Back up or stash workspace files before large automated edits.
- After automated edits, run linters/tests if available and report results.

Commands / Examples
- Collect comments:
  - `gh pr view 4 --json reviews`
  - `gh api repos/<owner>/<repo>/pulls/4/comments`
- Create PR comment:
  - `gh pr comment 4 --body "Fixed X (commit <sha>)"`
- Create a PR via CLI:
  - `gh pr create --base main --head chore/ai-setup --title "..." --body "..."`

Notes
- This process is intentionally conservative: Codex is allowed to change repository files but only on dedicated branches and with one commit per resolved review item. Human oversight is required for potentially risky changes.

