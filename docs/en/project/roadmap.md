---
title: "Roadmap"
description: "Planned work for arqix, in rough priority order"
date: 2026-07-06
status: active
---

# Roadmap

This document describes planned work for arqix. It is intentionally rough and will evolve as the project matures. Items are in approximate priority order, not a fixed schedule.

Progress through the implementation phases is measured by one number: the share of functional requirements referenced by `arqix:verifies` markers in the test suite (reported by `scripts/check_trace_markers.py`; currently 43/103). "Done" for a story means its skeleton tests are un-ignored and green.

## Phase 0 — Foundation (done)

- [x] Rust CLI skeleton that compiles and runs
- [x] Repository layout with docs/ tree
- [x] README, roadmap, AI transparency document
- [x] Initial blog posts
- [x] Basic `.gitignore` and project metadata complete

## Phase 1 — Specification (done)

What the old phases 1–4 sketched as features is now fully specified and traceable:

- [x] 8 personas, 103 user stories (`docs/en/architecture/stories/`)
- [x] 142 requirements — 103 functional, 17 quality, 22 constraints — under RFC 2119 subset + EARS patterns (`docs/en/architecture/req/`)
- [x] Ontology with requirement kinds and inverse properties (`docs/ontology/`)
- [x] Reference checkers: `check_requirements.py`, `check_frontmatter.py` (stdlib-only, strict, selftested)
- [x] arc42 documentation with Structurizr C4 model (`docs/en/architecture/`)
- [x] ADR-0001..0005 (agent instructions, C4 source, verification orchestrator, mechanical rewriter, command taxonomy)

## Phase 2 — Red skeleton and TDD harness (in review, PR #11)

- [x] clap command tree for the full surface per ADR-0005; stubs exit 70
- [x] 45 ignored command-contract tests mirroring the arc42 chapter 5 command-ownership table
- [x] `check_trace_markers.py` gate (TRC-001..005) and the normative TDD workflow in AGENTS.md
- [x] Glossary seeded: red skeleton, red phase

## Phase 3 — Trace family in Python, harness sharpening (done)

Goal: run arqix semantics on this repository daily, months before the Rust port — and turn the red skeleton into the conformance suite by construction.

- [x] Coverage report by requirement kind in the marker gate (the honest progress number is functional coverage, not total)
- [x] `ARQIX_BIN` override in the test helpers so the skeleton tests can run against any implementation
- [x] Python trace oracle `scripts/arqix_trace.py` behind the `scripts/arqix` dispatcher: `trace scan/check/coverage/matrix`, graph as JSON, coverage by kind, matrix as CSV — per the oracle policy in arc42 chapter 8; the `cli_trace` skeleton suite passes 7/7 against it
- [x] Thin Python `verify` sequencer (`scripts/arqix verify`): checkers + marker gate + informational coverage + cargo test
- Deliberately **not** in Python: parser/fmt/finalise (the lossless-CST machine is built once, in Rust), publish/render, mcp

## Phase 4 — Rust core, story by story (done)

Test-first per AGENTS.md: un-ignore, show red, implement to green. Rough order, chosen so each story stands on the previous one (the Config Resolver moved ahead of the parser: it is the smallest self-contained story and the parser only becomes observable through the store commands). Every slice landed as a `test(...)` red commit followed by a `feat(...)` green commit:

1. [x] Config Resolver (`config validate`, `config show`) — US-01-01-16
2. [x] Document Parser (frontmatter, classes/triples, body — the shared reading layer everything else needs), consumed directly by the Document Store & Catalog (`doc list`, `doc read`, `doc search`)
3. [x] Linter (`lint run`) — duplicate-id, include-target, and translation-source checks
4. [x] Formatter & Finaliser (`fmt`, `finalise` — the lossless line CST, single mutator per ADR-0004); byte-identical on the real corpus and idempotent
5. [x] Trace Engine (`trace scan/check/coverage/matrix`) — ported against the Python oracle; the un-ignored `cli_trace` skeleton tests pass 7/7 with the Rust binary, and `trace scan/coverage/matrix` are JSON-value-equal to the oracle on the real corpus
6. [x] Verification Orchestrator (`verify`, ADR-0003) — arqix now verifies its own corpus (format, lint, trace-scan pass; coverage gates honestly on the still-uncovered requirements)
7. [x] Template Engine (`doc init`, `doc new`, `unit new`)
8. [x] Assembler (`assemble build`) — include expansion into `pages/`, cycle detection (ASM-001), and a JSONL assembly log

Phase 5 is now the remaining command surface: `report bundle`, `policy check`, `mcp serve`, `publish site`, `render pdf`.

## Phase 5 — Publication and self-hosting

- CI workflow: `cargo test`, the checkers, and later `arqix verify` as the gate; structurizr-cli Mermaid export joins the first CI PR (arc42 chapter 11)
- Report & Export (`report bundle`), Policy Checker (`policy check`), MCP Server (`mcp serve`)
- Publish & Render (`publish site --lang`, `render pdf`), language-aware site, DE translations
- Self-hosting closes the loop: the Python reference checkers are demoted to cross-checks and retired per the oracle policy

## What this roadmap is not

This is not a promise. It is a direction. The project is early and solo. Priorities will shift based on what is actually useful.
