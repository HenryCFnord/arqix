---
title: "Knowledge-repository program status"
date: 2026-07-15
status: draft
category: docs
branch: claude/knowledge-repository-plan
---

# Status

Progress log for the knowledge-repository program (see PLANS.md; agents update this file, not the reviewed plan).

## Intake

- [x] IDEA.md captured and reviewed (owner, 2026-07-15); merged via PR #89.
- [x] PLANS.md drafted from the intake (this package).

## Band 1 — pre-v0.2.0

- [x] K0 — doc-new-kind-dir (gap G2): US-08-01-25 + REQ-08-01-25-01, spec-first red/green; merged via PR #90.
- [x] K1 — kind-template-key (gap G1): US-08-01-26 + REQ-08-01-26-01/-02, spec-first red/green; `[kinds.<family>].template` + TPL-002 for unknown placeholders.
- [x] K2 — doc-new-target-path (gap G3): US-08-01-27 + REQ-08-01-27-01, spec-first red/green; `--dir` on `doc new` and the aliases, containment-guarded.
- [x] K3 — source-record-kind (gap G5): US-08-01-28 + REQ-08-01-28-01/-02/-03, spec-first red/green; `arqix:classes/source` as a full ontology member (owner decision 2026-07-16, option a) with the SRC rule family in `lint frontmatter`, `[kinds.source]` creation surface, and SRC-0001 as the first corpus record.
- [x] K4 — normative-statement-export (P5): US-07-01-08 + REQ-07-01-08-01/-02, spec-first red/green; `arqix report statements` (CSV: id, kind, modality, EARS pattern, subject) with the committed export under the snapshot freshness gate.
- [x] R5 — checker-internal-dedup (refactor slice 5): the byte-identical checker helpers (Finding, py_repr family, IO walkers) hoisted into `src/checkers/shared.rs`, both date validations consolidated into `src/date.rs` side by side (semantics deliberately unmerged); the trace engine's divergent `py_repr` untouched.
- [x] R6 — required-meta-one-source (refactor slice 6): REQ-META-001 resolves the effective `[kinds.req].required-meta` contract (REQ-01-01-19-03, which the frontmatter checker and formatter already honoured) instead of the hardcoded six-key const — validation and configuration can no longer disagree.
- [ ] R7 — frontmatter-vocab-config (refactor slice 7; may slip past the release).
- [ ] Release v0.2.0 (RELEASING.md; the owner tags and publishes).

## Band 2 — decision gate (owner)

- [ ] D1 — slice-8 ADR scope (absorb entity/relation questions?).
- [ ] D2 — G7 lifecycle vocabularies versus status namespace.
- [x] D3 — PR #87 splitter close-out fate: option C (owner, 2026-07-17).
  PR #87 is closed, the four-splitter-contracts insight is rewritten in present tense into arc42 chapter 8, and X8 drops out of the band-3 table (consolidation without user-visible value; the chapter-8 note records why it stays unplanned).
- [x] D4 — evidence granularity: option A, statement-level anchors with sparse opt-in claims (owner, 2026-07-17).
  Owner note: paragraph- or section-level anchors (any heading depth, not only `##`) are expected to suffice in most cases, so the claim marker must anchor coarser blocks naturally, with statement precision available where a single sentence carries the evidence.
- Reframing note (owner, 2026-07-17): D1 and D2 are hard-coupled and widen into one design question — arqix as a catalog of configurable processes (hardwired rules acting as switches/gates, selected per project) plus a layered ontology (reserved core vocabulary, module vocabularies, project extension) that the checker validates.
  The A1 ADR takes that scope; analysis delivered in session, decision pending.

## Band 3 — post-v0.2.0

Pending band 2; see the PLANS.md table (A1-A4 ontology/entity, B1-B2 evidence/provenance, C5 lifecycle, D6 crosswalk, D7 queries, X8 splitter consolidation).

- Owner note (2026-07-15) for the A-slices: the story-workflow coupling lint (US-WF-001/US-PER-001, PR #94) hardcodes today's persona/workflow ontology; when the ontology becomes configuration, this linting must become configurable with it.
- X8 (splitter consolidation) is removed from the planned slices per decision D3; the rationale lives in arc42 chapter 8 (line splitting and frontmatter parsing).

## Context

The oracle retirement (refactor slice 4, task #78) completed 2026-07-15 via PR #88: the Rust engine owns every corpus check, `just verify` is the daily gate, and the pre-0.2.0 refactor program's remaining slices (R5-R7) fold into band 1 above.
