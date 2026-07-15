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
  updated: 2026-07-15
  lang: en
  translation-of:
  generated: false
---

## Crosscutting Concepts

Each concept below is a system-wide contract with its own cross-cutting requirement; components implement them via the shared spine (chapter 5).

- **Determinism** — byte-identical outputs for identical inputs and configuration; stable ordering, no ambient state (REQ-00-00-00-01).
  - **Corpus traversal** — the markdown corpus walk reproduces Python `sorted(dir.rglob('*.md'))` byte for byte: each directory level is sorted, directory symlinks are never followed, and `*.tpl.md` templates are excluded.
    It is one shared internal helper (`src/util.rs` `collect_markdown`, consumed by the store and the publisher); the trace engine and the checkers still keep their own walks — consolidating them is post-retirement refactor work (slice 5), now that the oracle retirement closed the freeze.
    `walkdir` is deliberately not adopted (ADR-0014 dependency posture): it would still need the same manual sort, skip-directory, and extension filtering wrapped around it to match the oracle, so it removes almost no code while adding a supply-chain dependency and a silent ordering- or symlink-drift risk.
- **Diagnostics contract** — every diagnostic is available as documented JSON with severity, stable code, message, and source location (REQ-00-00-00-03, REQ-04-01-10-*); coverage gaps use the same shape, and trace outputs are layered with per-layer stability promises (ADR-0006).
- **Exit codes** — `0` success, `1` findings or gate failure, `2` usage error; stable across releases (REQ-00-00-00-02, REQ-04-01-08-01).
- **Effective configuration** — one resolution path from `arqix.toml` through defaults and overrides; `config show` renders exactly what commands act on (REQ-00-00-00-06).
- **ID and slug policy** — deterministic derivation from configured policy, global duplicate detection, stable anchors (REQ-00-00-00-04).
- **i18n model** — source documents with linked translations; missing, unresolved, and outdated translations are lint findings and CI gates (REQ-00-00-00-10).
- **Guardrails** — declared change scope via policy files, no overwrites without approval, filesystem containment, no content execution (REQ-00-00-00-07/08/13/14).
- **Performance budgets** — sub-second search/read and a ten-second verification loop on a 1000-document corpus as calibratable budgets (REQ-00-00-00-11/12).
- **Checker oracle policy** — the Python reference checkers (`scripts/check_requirements.py`, `scripts/check_frontmatter.py`) were the behavioural oracle for the corresponding Rust checks until each port passed its conformance suite: identical inputs (the real corpus plus the selftest fixtures) produce identical JSON findings.
  After conformance, an oracle is retired (the policy allowed a CI cross-check grace period first); the Rust implementation then owns the contract.
  The trace family extended this: `scripts/arqix_trace.py` (behind the `scripts/arqix` dispatcher) was the oracle for `trace scan/check/coverage/matrix`, and its conformance suite existed by construction — the skeleton tests in `tests/cli_trace.rs` ran against either implementation via the `ARQIX_BIN` override, so "ported" meant the same suite green with the Rust binary.
  - **Status: retired (2026-07-15).**
    Every Python oracle passed its conformance suite — the trace engine (`cli_trace` green with the Rust binary; scan/coverage JSON-value-equal, matrix byte-identical CSV), the marker gate, the report units (byte-identical under one stamp), and the frontmatter/requirements checkers (JSON value-equal on the real corpus) — and the oracle selftest fixtures are mirrored in the Rust test suite (`selftest_cases_match_the_oracle` and its siblings), so the executable specification survives the scripts.
    The owner closed the grace period and retired all five scripts directly (task #78); they remain in git history as the provenance record of each port.
    The Rust engine owns every contract, and the dogfooded `arqix verify` is the authoritative corpus gate: format, lint including the frontmatter/requirements checkers, trace-scan, coverage/ratchet/freshness, the marker gate, and the strategy-aware report-freshness check.
