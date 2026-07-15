# CLAUDE.md

This file is a thin adapter for Claude Code, as decided in [ADR-0001](docs/en/architecture/adr/ADR-0001-agent-instruction-document-layout.md).

## Process contract

The canonical agent instruction document for this repository is [AGENTS.md](AGENTS.md).
Read it first.
All normative process rules — workflow, task sources, branching, scope control, testing, commits, pull requests, and safety rules — live there and only there.
This file never restates or overrides them.

## Role mapping for Claude Code

AGENTS.md is written agent-neutrally.
Claude Code takes the implementer role; intake may arrive as direct instructions in a Claude Code session and is treated by the task-source rules defined in AGENTS.md.

This is a mapping only; it introduces no new rules.

## Claude-Code-specific notes

- Skills: `.claude/skills/arqix/` packages the arqix tool-usage skill (the command surface and the MCP tools; agent-onboarding strand).
  Skills are agent-specific extension points and carry no normative process rules (REQ-01-01-09-06) — the skill points back at AGENTS.md for process.
- Non-normative convenience pointers:
  - When working on `docs/en/architecture/stories/` or `docs/en/architecture/req/`, run `target/debug/arqix lint requirements`.
  - When touching any document under `docs/en/architecture/` or `docs/ontology/`, run `target/debug/arqix lint frontmatter`.
  - When touching Rust code under `src/` or `tests/`, run `target/debug/arqix trace markers` and `cargo test`.
  - Daily gate for any change: `just verify` runs `cargo test`, the dogfooded `arqix verify` (format, lint including the frontmatter/requirements checkers, trace scan, informational coverage, ratchet, the marker gate, report freshness), and markdownlint in one command.
  - Requirement authoring rules (RFC 2119 subset + EARS patterns) are documented in `docs/en/processes/requirements-style-guide.md`.
  - Markdown authoring rules (markdownlint via `.markdownlint.jsonc`, one sentence per line, markers above their block) are documented in `docs/en/processes/markdown-style-guide.md`; run `npx markdownlint-cli2` on touched Markdown.
  - Refactoring follows the four-phase loop (assess, strengthen tests, refactor, tidy) documented in `docs/en/processes/refactoring-methodology.md`; the normative rules live in AGENTS.md `## Refactoring`.
