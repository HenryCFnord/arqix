---
title: "Refinement plan: phase 5 strands"
date: 2026-07-09
status: draft
category: docs
branch: docs/refinement-2026-07
---

# Plan

## Summary

Refinement session after the full-repository review (PR #20) and the publication of the repository and [arqix.dev](https://arqix.dev).
It turns the collected review outcomes into phase-5 work: a reordered roadmap, three new story strands ready for derivation, and two decision lists (spec sweep, config audit) that need human decisions before generation.

This plan is written before any corpus generation; new stories and requirements are drafted here and only enter `docs/en/architecture/` after review, through the usual checker gate.

## Scope

- In scope:
  - the phase-5 roadmap rewrite (landed in this branch)
  - story and requirement drafts for the verification-process strand
  - story drafts for the configuration strand (final cut pending the config-audit decisions)
  - the agent-onboarding strand outline
  - the two decision lists: [SPEC-SWEEP.md](SPEC-SWEEP.md) and [CONFIG-AUDIT.md](CONFIG-AUDIT.md)
- Out of scope:
  - any Rust or checker implementation (follows story-first after derivation)
  - changes to existing stories or requirements (the spec sweep only *lists* candidates; edits are their own change)

## Strand 1 — Verification process

Principle (recorded on the roadmap): coverage measures project progress, so an absolute number must never gate a change; what gates is change quality — consistency, regression, and claims.

### US-04-01-14 — Configure the verification loop (Daria)

As Daria, I want to configure which sub-steps `arqix verify` runs and how each result is treated, so that the loop fits the repository's lifecycle stage instead of blocking healthy spec-first states.

Requirement drafts (EARS):

- REQ-04-01-14-01: When `arqix verify` is invoked, arqix SHALL run the sub-steps declared in the effective configuration in their configured order.
- REQ-04-01-14-02: Where a verify sub-step is configured as informational, arqix SHALL report its findings without affecting the exit code.
- REQ-04-01-14-03: The default verify configuration SHALL treat coverage as informational and every other sub-step as gating.

Consequence once shipped: CI switches its gate from `scripts/arqix verify` to `arqix verify` (dogfooding); the Python sequencer is demoted to a cross-check per the oracle policy.

### US-04-01-15 — Gate coverage as a ratchet (Daria)

As Daria, I want changes that reduce verified coverage to fail the gate, so that specification growth stays free while regressions cannot land silently.

Requirement drafts:

- REQ-04-01-15-01: When verified coverage decreases against the committed baseline, the coverage ratchet SHALL fail with findings naming each regressed requirement.
- REQ-04-01-15-02: When a change only adds requirements or stories, the coverage ratchet SHALL pass.

Design note: the committed report snapshots (kept fresh by the report-freshness gate) are the natural baseline; no second bookkeeping file.

### US-03-01-09 — Machine-check the done claim (Quinn)

As Quinn, I want a story marked done to be provably done, so that lifecycle states are claims the gate checks instead of hopes.

Requirement drafts:

- REQ-03-01-09-01: When a story declares `lifecycle-status: done`, arqix SHALL report an error for each of its requirements that no active test verifies.
- REQ-03-01-09-02: The story lifecycle vocabulary SHALL be a controlled set validated by the frontmatter contract.

Decision D1 (settled in review, to be recorded as ADR-0010): three vocabularies by document nature, plus the declared-versus-computed rule (declared states carry intent, computed states carry findings; progress within a state is a report number, never a state).

- Stories: `draft` → `specified` → `in-implementation` → `done`, terminal `retired`; done ⇒ every requirement verified by an active test.
- Requirements: only `active`/`retired` declared (amended 2026-07-10: `draft` replaced by `active` — the gate refutes draft, and a verified requirement declaring draft misleads in the other direction); everything else is computed from the trace graph. v1 checks test verification; the ontology's verification methods (inspection, analysis, …) are the prepared hook for non-test evidence.
- Prose documents (units, pages, …): `draft` → `final`, transition performed by `finalise` (the single mechanical mutator, ADR-0004); the publish pipeline takes only `final`; editing a final document returns it to `draft`; terminal `retired`.
- ADRs: the document text follows the prose model; `decision-status` (proposed/accepted/deprecated/superseded) stays the orthogonal decision axis.

## Strand 2 — Configuration over convention

From the six PR-#20 review comments; the full inventory with per-item recommendations is [CONFIG-AUDIT.md](CONFIG-AUDIT.md).
Story cut proposal (owners: Mara), final after the audit decisions:

- US-01-01-18 — Configure the ID policy: one configured pattern per document family that oracle, engine, and checkers all read (anchor: REQ-00-00-00-04). Per owner decision this includes the ID *shapes* (C15/C16) and therefore the derivation model — owner-story slice, cross-cutting marker, per-story sequencing — expressed through named pattern groups; the current scheme becomes the default configuration and the corpus stays unchanged. Candidate for its own ADR.
  Amended 2026-07-11 (owner decision, recorded in ADR-0012): declared `derived-from` triples are the source of truth for ownership — the derivation-from-named-groups model was rejected as too restrictive; groups are optional and activate consistency checks, cross-cutting becomes a declared ontology marker.
- US-01-01-19 — Configure frontmatter contracts: per-family canonical key order as one configured source consumed by `fmt` and the frontmatter checker (ends today's double bookkeeping).
- US-01-01-20 — Template files: `doc new`/`doc init` instantiate template files from a configured directory; `doc init` scaffolds the defaults; the string literals in `src/templates.rs` are removed (anchor: REQ-00-00-00-05).

Decision D2 (settled in review): all audit recommendations confirmed as listed, with the owner overrule already recorded (ID shapes → C15/C16); D2a story cut and D2b oracle-coupling criterion confirmed; D2c — the REQUIRED_META divergence is fixed immediately, ahead of the config strand. The D2 boundary becomes ADR-0011; the ID-policy model gets its own ADR with US-01-01-18.

## Strand 3 — Agent onboarding

Outline (owner: Casey; builds on `docs/en/processes/agent-orchestration-patterns.md`):

- A handbook chapter "Working in an arqix-governed repository" (unit family per ADR-0009's manual plan).
  The chapter recommends the default ID policy as the naming scheme for new corpora (noted 2026-07-11, with ADR-0012): semantic IDs keep ownership relations readable in place, and the consistency checks against the declared triples already exist.
- `doc init` scaffolds an agent-instructions file (AGENTS.md-shaped, pointing at the gate commands and the ICD input grammars).
- A packaged skill shipped alongside `mcp serve`, so agent frameworks can discover the arqix workflow.

Decision D3 (settled in review): the skill lands with this strand (slice 6), after MCP exists.

## Spec sweep

[SPEC-SWEEP.md](SPEC-SWEEP.md) classifies every not-yet-verified story as FITS / SUPERSEDED / UNDERSPECIFIED / DEFERRED with a one-line reason.
Decision D4 (settled in review): all classifications confirmed. For the `doc new` trio the resolution is option (b) — implement `--title`/`--id`/`--dry-run` as the next Template Engine slice (the requirements are already surface-neutral). For US-08-01-19 the resolution is option (a) — reword the acceptance to the shipped positional `trace check <requirement>`, which is the better surface.

## Process

1. This plan package + roadmap rewrite land as the refinement PR (no corpus changes yet).
2. Decisions D1–D4 are made in the PR review.
3. Generation follows as separate gated changes: strand 1 stories/requirements first (they unlock the CI dogfooding), then strand 2 per audit decision, strand 3 with its slice.
