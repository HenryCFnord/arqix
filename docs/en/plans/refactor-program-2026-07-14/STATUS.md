---
title: "Pre-0.2.0 refactor program status"
date: 2026-07-14
status: draft
category: docs
branch: claude/refactor-prep
---

# Status

Tracks task #85 (pre-0.2.0 refactor program).
Phases follow the two-band structure in PLANS.md.

## Phase 0 — Prep (IN PROGRESS)

The methodology scaffold that every slice leans on.

- [ ] `docs/en/processes/refactoring-methodology.md` — the ASSESS -> STRENGTHEN TESTS -> REFACTOR CODE -> TIDY TESTS loop and its non-negotiables (characterization-before-refactor, spec-first for behaviour-visible change, the oracle-fidelity freeze, the neutral module homes, the ADR-0014 crate bar, the ADR-0011 defaults-preserve/one-source rules). Cross-links ADR-0010/0011/0012/0014 and CONFIG-AUDIT.md.
- [ ] AGENTS.md `## Refactoring` normative section, immediately after `## Test-driven implementation` — the only place gaining normative rules (the methodology doc and skills carry none, REQ-01-01-09-06).
- [ ] Skills under `.claude/skills/` deferring process to AGENTS.md: `/refactor` (drives the four-phase loop for a chosen slice) and `/test-audit` (drives the STRENGTHEN/TIDY phases in isolation; the tool slice 1 runs under).
- [ ] This plan package reviewed; deferred owner decisions made.

## Phase A — Pre-gate band (pending)

Independent of the gate; can start once Phase 0 lands.

- [x] Slice 1 — test-baseline-hygiene.
- [x] Slice 2 — markdown-scan-helpers (`src/markdown.rs`).
- [x] Slice 3 — path-and-walker-helpers (`src/util.rs`); walkdir rejection note.
- [ ] Slice 12 — splitter-contract-docs (documentation close-out; no gate dependency).
  In review; likely superseded once the oracle retires and the three splitter contracts consolidate to one.

## The gate

- [x] Slice 4 — oracle-retirement-gate (task #78, self-hosting strand). Executed as the full retirement (owner decision 2026-07-15): all five Python oracle scripts removed after conformance, their selftest fixtures mirrored into the Rust suite first, the Rust `arqix verify` is the authoritative corpus gate, and the freeze is closed. Slices 5-7 unblock.

## Phase B — Post-gate band (pending)

Blocked on the gate, or done dual-source per the gate-timing owner decision.

- [ ] Slice 5 — checker-internal-dedup (`src/date.rs`; the rewriter half may land pre-gate).
- [ ] Slice 6 — required-meta-one-source (the one high-value item).
- [ ] Slice 7 — frontmatter-vocab-config (C9 section-kinds, allowed-external-types).
- [ ] Slice 8 — ontology-as-config-adr (new ADR + ADR-0011/0012 amendment; owner review).
- [ ] Slice 9 — ontology-vocabulary-derivation.
- [ ] Slice 10 — iri-namespace-config (part a dedup unblocked; part b behind slice 8).
- [ ] Slice 11 — lifecycle-model-selector (conditional / low priority).
