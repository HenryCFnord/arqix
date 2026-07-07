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
  updated: 2026-07-06
  lang: en
  translation-of:
  generated: false
---

## Crosscutting Concepts

Each concept below is a system-wide contract with its own cross-cutting requirement; components implement them via the shared spine (chapter 5).

- **Determinism** — byte-identical outputs for identical inputs and configuration; stable ordering, no ambient state (REQ-00-00-00-01).
- **Diagnostics contract** — every diagnostic is available as documented JSON with severity, stable code, message, and source location (REQ-00-00-00-03, REQ-04-01-10-*); coverage gaps use the same shape, and trace outputs are layered with per-layer stability promises (ADR-0006).
- **Exit codes** — `0` success, `1` findings or gate failure, `2` usage error; stable across releases (REQ-00-00-00-02, REQ-04-01-08-01).
- **Effective configuration** — one resolution path from `arqix.toml` through defaults and overrides; `config show` renders exactly what commands act on (REQ-00-00-00-06).
- **ID and slug policy** — deterministic derivation from configured policy, global duplicate detection, stable anchors (REQ-00-00-00-04).
- **i18n model** — source documents with linked translations; missing, unresolved, and outdated translations are lint findings and CI gates (REQ-00-00-00-10).
- **Guardrails** — declared change scope via policy files, no overwrites without approval, filesystem containment, no content execution (REQ-00-00-00-07/08/13/14).
- **Performance budgets** — sub-second search/read and a ten-second verification loop on a 1000-document corpus as calibratable budgets (REQ-00-00-00-11/12).
- **Checker oracle policy** — the Python reference checkers (`scripts/check_requirements.py`, `scripts/check_frontmatter.py`) are the behavioural oracle for the corresponding Rust checks until the port passes a conformance suite: identical inputs (the real corpus plus the selftest fixtures) produce identical JSON findings.
  After conformance, the scripts are demoted to a CI cross-check and removed after a grace period; the Rust implementation then owns the contract.
  The trace family extends this: `scripts/arqix_trace.py` (behind the `scripts/arqix` dispatcher) is the oracle for `trace scan/check/coverage/matrix`, and its conformance suite exists by construction — the skeleton tests in `tests/cli_trace.rs` run against either implementation via the `ARQIX_BIN` override, so "ported" means the same suite is green with the Rust binary.
  - **Status (Phase 4).**
    The Rust trace engine (`src/trace.rs`) is ported and conformant: the `cli_trace` suite passes 7/7 with the Rust binary, and `trace scan`/`coverage` are JSON-value-equal — while `trace matrix` is byte-identical CSV — to the oracle on the real corpus.
    Per the policy above, the trace oracle therefore enters its **cross-check** phase: `scripts/arqix_trace.py` is retained only as a CI cross-check against the Rust output for a grace period, after which it is retired and the Rust implementation owns the `trace` contract.
    The Rust binary also now runs the whole `verify` loop over this repository (`arqix verify`: format, lint, trace-scan, coverage), so the toolchain verifies its own corpus.
    `check_requirements.py`/`check_frontmatter.py` remain the active oracle — their Rust ports are Phase 5 work.
