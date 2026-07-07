---
title: "Cross-cutting concern candidates"
date: 2026-07-02
status: approved
branch: docs/add-requirements
plan_dir: docs/en/plans/requirements-derivation-2026-07-02
---

# Cross-cutting concern candidates

Candidate requirements for the reserved `REQ-00-00-00-NN` domain, identified by sweeping the acceptance criteria of all 103 user stories in `docs/en/architecture/stories/`.
These are the behaviours that recur across persona groups and will form the foundation of arqix itself.

This is a review artefact: no requirement files are created from it until the candidate list is approved.
Story lists below are indicative (regex sweep over acceptance criteria); the exact `derived-from` sets are pinned during derivation, and every accepted candidate needs at least two contributing stories (checker rule `REQ-LNK-002`).

All draft sentences below have been validated against `scripts/check_requirements.py` (EARS pattern match, keyword subset, kind matrix).

## Candidates

### REQ-00-00-00-01 — Deterministic outputs

- Kind: constraint
- Draft: *The arqix CLI SHALL produce byte-identical outputs for identical inputs and configuration.*
- Evidence: 53 stories across all 8 persona groups mention deterministic, reproducible, or stably ordered outputs in their acceptance criteria.
- Representative stories: US-01-01-04, US-02-01-11, US-03-01-08, US-04-01-06, US-05-01-08, US-06-01-08, US-07-01-06, US-08-01-22

### REQ-00-00-00-02 — Stable exit codes

- Kind: functional
- Draft: *The arqix CLI SHALL signal command outcomes through documented, stable exit codes.*
- Evidence: 13 stories in groups 01, 04, 05, 08.
- Representative stories: US-04-01-08, US-05-01-14, US-08-01-15

### REQ-00-00-00-03 — Machine-readable diagnostics

- Kind: functional
- Draft: *When arqix emits a diagnostic, arqix SHALL provide it in a documented machine-readable format.*
- Evidence: 47 stories across all 8 groups reference machine-readable output, JSON/JSONL, or diagnostics contracts.
- Representative stories: US-01-01-08, US-03-01-05, US-04-01-10, US-05-01-14, US-08-01-21

### REQ-00-00-00-04 — Deterministic IDs and slugs

- Kind: functional
- Draft: *The arqix CLI SHALL derive document IDs and slugs deterministically from the configured policy.*
- Evidence: 22 stories in groups 01, 02, 03, 05, 06, 08 (unique IDs, duplicate detection, stable selectors/anchors).
- Representative stories: US-01-01-01, US-02-01-02, US-05-01-10, US-06-01-10, US-08-01-05

### REQ-00-00-00-05 — Template-governed document creation

- Kind: functional
- Draft: *When a document is created, arqix SHALL instantiate the configured template for the requested kind.*
- Evidence: 14 stories in groups 01, 02, 04, 05, 06, 08.
- Representative stories: US-01-01-13, US-02-01-07, US-06-01-03, US-08-01-23

### REQ-00-00-00-06 — Effective configuration as baseline

- Kind: functional
- Draft: *The arqix CLI SHALL resolve every command against the effective configuration.*
- Evidence: 26 stories in groups 01, 02, 04, 05, 06, 08 (config validation, effective-config inspection, configured policies).
- Representative stories: US-01-01-16, US-04-01-11, US-05-01-11, US-08-01-20

### REQ-00-00-00-07 — Change-scope guardrails

- Kind: constraint
- Draft: *The arqix CLI SHALL NOT modify files outside the declared change scope.*
- Evidence: scope-guardrail behaviour appears in groups 01, 04, 08 (US-01-01-07, US-04-01-02, US-08-01-08) and underpins agent-safe automation throughout group 08.
- Representative stories: US-01-01-07, US-04-01-02, US-08-01-08

### REQ-00-00-00-08 — No-overwrite safety

- Kind: constraint
- Draft: *The arqix CLI SHALL NOT overwrite existing files without explicit approval.*
- Evidence: 6 stories in groups 01, 02, 08 (init refusal paths, mechanical-only finalize).
- Representative stories: US-01-01-01, US-02-01-08, US-08-01-06

### REQ-00-00-00-09 — Dry-run support

- Kind: functional
- Draft: *Where a command creates or modifies files, the command SHALL support a dry-run mode that reports planned changes without writing.*
- Evidence: 6 stories in groups 01, 02, 06, 08.
- Representative stories: US-01-01-13, US-02-01-10, US-06-01-06, US-08-01-23

### REQ-00-00-00-10 — Translation drift detection

- Kind: functional
- Draft: *When translations exist for a document, arqix SHALL detect missing and outdated translations deterministically.*
- Evidence: 13 stories in groups 01, 02, 04, 05, 06, 08.
- Representative stories: US-01-01-14, US-04-01-04, US-05-01-05, US-08-01-11

## Observations

- Candidates 01, 03, and 06 have the broadest reach and are effectively the tool's core contracts; per-story requirements will frequently be refinements of them.
  During derivation, a per-story requirement that merely restates one of these SHOULD instead be replaced by a `has-requirement` link to the shared requirement.
- Candidate 10 (translation drift) sits on the boundary between a cross-cutting concern and an i18n feature cluster.
  It is included because six of eight groups depend on it; review may decide to keep it story-bound instead.
- The sweep is regex-based over acceptance criteria; it can over- or under-match.
  Final `derived-from` sets are fixed manually per candidate during derivation.

## Review decision (2026-07-02)

All ten candidates were approved by the repository owner as proposed, including the kind assignments.
Additionally decided: `derived-from` uses full verified story lists (every story whose acceptance criteria demonstrably demand the behaviour), and story-bound derivation starts with a pilot for persona group 01 before groups 02–08 follow.

The requirement files exist under `docs/en/architecture/req/` as `REQ-00-00-00-01` … `REQ-00-00-00-10`.
The final `derived-from` sets were pinned by re-running the acceptance-criteria sweep and manually verifying every matched line; notable curation against the indicative lists above:

- id/slug and placeholder-substitution criteria moved from candidate 01 to 04; exit-code determinism to 02
- "clear/actionable diagnostics" criteria excluded from 03 (clarity is not machine-readability)
- Pandoc render templates and CI workflow templates excluded from 05
- candidate 06 grew to 52 stories: every criterion demanding configuration-driven behaviour derives it
- candidate 07 pinned manually to US-01-01-07, US-04-01-02, US-08-01-08 (their criteria say `policy check`, so the regex sweep missed them); report-scoping criteria excluded
- translation scaffolding, language filtering, and i18n publishing configuration excluded from 10
