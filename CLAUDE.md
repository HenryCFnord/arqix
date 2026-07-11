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

- Skills: this repository currently defines no Claude Code skills (there is no `.claude/skills/`).
  If skills are added, they are agent-specific extension points and must not carry normative process rules (see REQ-01-01-09-06).
- Non-normative convenience pointers:
  - When working on `docs/en/architecture/stories/` or `docs/en/architecture/req/`, run `python3 scripts/check_requirements.py`.
  - When touching any document under `docs/en/architecture/` or `docs/ontology/`, run `python3 scripts/check_frontmatter.py`.
  - When touching Rust code under `src/` or `tests/`, run `python3 scripts/check_trace_markers.py` and `cargo test`.
  - Daily gate for any change: `python3 scripts/arqix verify` runs the checkers, the marker gate, `cargo test`, and the dogfooded `arqix verify` (format, lint, trace scan, informational coverage, ratchet) in one command.
  - Requirement authoring rules (RFC 2119 subset + EARS patterns) are documented in `docs/en/processes/requirements-style-guide.md`.
  - Markdown authoring rules (markdownlint via `.markdownlint.jsonc`, one sentence per line, markers above their block) are documented in `docs/en/processes/markdown-style-guide.md`; run `npx markdownlint-cli2` on touched Markdown.
