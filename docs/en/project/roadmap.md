---
title: "Roadmap"
description: "Planned work for arqix, in rough priority order"
date: 2026-07-05
status: active
---

# Roadmap

This document describes planned work for arqix. It is intentionally rough and will evolve as the project matures. Items are in approximate priority order, not a fixed schedule.

Progress through the implementation phases is measured by one number: the share of functional requirements referenced by `arqix:verifies` markers in the test suite (reported by `scripts/check_trace_markers.py`; currently 42/103). "Done" for a story means its skeleton tests are un-ignored and green.

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

## Phase 3 — Trace family in Python, harness sharpening (next)

Goal: run arqix semantics on this repository daily, months before the Rust port — and turn the red skeleton into the conformance suite by construction.

- Coverage report by requirement kind in the marker gate (the honest progress number is functional coverage, not total)
- `ARQIX_BIN` override in the test helpers so the skeleton tests can run against any implementation
- Extend the marker checker into a Python `trace scan` / `trace coverage` oracle: trace graph as JSON, coverage by kind, matrix as CSV — per the oracle policy in arc42 chapter 8
- Thin Python `verify` sequencer over the existing checkers
- Deliberately **not** in Python: parser/fmt/finalise (the lossless-CST machine is built once, in Rust), publish/render, mcp

## Phase 4 — Rust core, story by story

Test-first per AGENTS.md: un-ignore, show red, implement to green. Rough order, chosen so each story stands on the previous one:

1. Document Parser (frontmatter, sections/anchors, directives — the shared reading layer everything else needs)
2. Config Resolver (`config validate`, `config show`)
3. Document Store & Catalog (`doc list`, `doc read`, `doc search`)
4. Linter (`lint run`)
5. Formatter & Finaliser (`fmt`, `finalise` — the lossless CST layer, single mutator per ADR-0004)
6. Trace Engine (`trace scan/check/coverage/matrix`) — ported against the Python oracle; the un-ignored skeleton tests are the conformance suite
7. Verification Orchestrator (`verify`, ADR-0003) — at this point arqix verifies its own corpus
8. Template Engine (`doc init`, `doc new`, `unit new`)
9. Assembler (`assemble build`)

## Phase 5 — Publication and self-hosting

- CI workflow: `cargo test`, the checkers, and later `arqix verify` as the gate; structurizr-cli Mermaid export joins the first CI PR (arc42 chapter 11)
- Report & Export (`report bundle`), Policy Checker (`policy check`), MCP Server (`mcp serve`)
- Publish & Render (`publish site --lang`, `render pdf`), language-aware site, DE translations
- Self-hosting closes the loop: the Python reference checkers are demoted to cross-checks and retired per the oracle policy

## What this roadmap is not

This is not a promise. It is a direction. The project is early and solo. Priorities will shift based on what is actually useful.
