# CLAUDE.md

This file is a thin adapter for Claude Code, as decided in
[ADR-0001](docs/en/architecture/adr/ADR-0001-agent-instruction-document-layout.md).

## Process contract

The canonical agent instruction document for this repository is
[AGENTS.md](AGENTS.md). Read it first. All normative process rules —
workflow, task sources, branching, scope control, testing, commits,
pull requests, and safety rules — live there and only there. This file
never restates or overrides them.

## Role mapping for Claude Code

Where AGENTS.md names specific tools, the roles map as follows:

- "Codex implements from the reviewed planning artefacts" — the
  implementer role applies equally to Claude Code.
- "OpenClaw orchestrates intake and workflow transitions" — intake may
  also arrive as direct instructions in a Claude Code session; treat
  such instructions by the same task-source rules defined in AGENTS.md.

This is a mapping only; it introduces no new rules.

## Claude-Code-specific notes

- Skills: this repository currently defines no Claude Code skills
  (there is no `.claude/skills/`). If skills are added, they are
  agent-specific extension points and must not carry normative process
  rules (see REQ-01-01-09-06).
- Non-normative convenience pointers:
  - When working on `docs/en/architecture/stories/` or
    `docs/en/architecture/req/`, run
    `python3 scripts/check_requirements.py --allow-unlinked-stories`.
  - Requirement authoring rules (RFC 2119 subset + EARS patterns) are
    documented in `docs/en/processes/requirements-style-guide.md`.
