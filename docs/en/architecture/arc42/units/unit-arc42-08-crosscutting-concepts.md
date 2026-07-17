---
id: unit-arc42-08
title: Crosscutting Concepts
slug: crosscutting-concepts
iri: arqix:units/unit-arc42-08

rdf:
  type:
    - arqix:classes/unit

triples: []

properties:
  section-kind: arc42-chapter

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-03
  updated: 2026-07-17
  lang: en
  translation-of:
  generated: false
---

## Crosscutting Concepts

Each concept below is a system-wide contract with its own cross-cutting requirement; components implement them via the shared spine (chapter 5).

- **Determinism** — byte-identical outputs for identical inputs and configuration; stable ordering, no ambient state (REQ-00-00-00-01).
  - **Corpus traversal** — the markdown corpus walk reproduces Python `sorted(dir.rglob('*.md'))` byte for byte: each directory level is sorted, directory symlinks are never followed, and `*.tpl.md` templates are excluded.
    It is one shared internal helper (`src/util.rs` `collect_markdown`, consumed by the store and the publisher); the trace engine and the checkers keep their own walks, and consolidating them is planned refactor work.
    `walkdir` is deliberately not adopted (ADR-0014 dependency posture): it would still need the same manual sort, skip-directory, and extension filtering wrapped around it to match the oracle, so it removes almost no code while adding a supply-chain dependency and a silent ordering- or symlink-drift risk.
- **Diagnostics contract** — every diagnostic is available as documented JSON with severity, stable code, message, and source location (REQ-00-00-00-03, REQ-04-01-10-*); coverage gaps use the same shape, and trace outputs are layered with per-layer stability promises (ADR-0006).
- **Exit codes** — `0` success, `1` findings or gate failure, `2` usage error; stable across releases (REQ-00-00-00-02, REQ-04-01-08-01).
- **Effective configuration** — one resolution path from `arqix.toml` through defaults and overrides; `config show` renders exactly what commands act on (REQ-00-00-00-06).
- **ID and slug policy** — deterministic derivation from configured policy, global duplicate detection, stable anchors (REQ-00-00-00-04).
- **i18n model** — source documents with linked translations; missing, unresolved, and outdated translations are lint findings and CI gates (REQ-00-00-00-10).
- **Guardrails** — declared change scope via policy files, no overwrites without approval, filesystem containment, no content execution (REQ-00-00-00-07/08/13/14).
- **Performance budgets** — sub-second search/read and a ten-second verification loop on a 1000-document corpus as calibratable budgets (REQ-00-00-00-11/12).
- **Line splitting and frontmatter parsing** — four deliberately separate splitter contracts, not one shared splitter (REQ-00-00-00-01, ADR-0004).
  The parser's semantic split (`parser::py_splitlines`) mirrors Python `str.splitlines` over its full boundary set (form feed, NEL, the Unicode line and paragraph separators), so every `body_offset`, trace, and lint line number is indexed in that one line space.
  The rewriter's frontmatter split (`rewriter::split_frontmatter`) is byte-lossless instead: it breaks only on `\n` and preserves the exact fence bytes and terminator, so `fmt` leaves an already-conforming document byte-identical (ADR-0004).
  The checkers read through their own reference-faithful reader (`checkers::shared::read_universal`, one shared copy since refactor slice R5), pinned by the mirrored selftest cases now that the Python reference implementations are retired.
  The assembler's `frontmatter_line_count` stays a fourth local counter over `str::lines()`: folding it into the parser's `body_offset` would mix two line-index spaces and force a re-parse of the fragment.
  Consolidating these contracts is deliberately unplanned work: they differ in meaning, not by accident, and a shared splitter would have to serve all four meanings at once (owner decision 2026-07-17, knowledge-repository band 2, D3).
- **Checker contract ownership** — the Rust engine owns every corpus check: the frontmatter and requirements checkers (`lint frontmatter` / `lint requirements`), the trace engine, the marker gate, and the report units.
  The executable specification is the test suite: each contract's reference fixtures are mirrored case by case in its test module (`selftest_cases_match_the_oracle` and its siblings), so a regression against a pinned behaviour fails `cargo test`.
  The dogfooded `arqix verify` is the authoritative corpus gate: format, lint including the frontmatter/requirements checkers, trace-scan, coverage/ratchet/freshness per the configured `[policies.verify]`, the marker gate, and the strategy-aware report-freshness check.
  The provenance of each port (the retired Python reference implementations and their conformance suites) is recorded in chapter 11 and the refactor-program plan package.
