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
- [ ] K1 — kind-template-key (gap G1): `[kinds.<family>].template` + validated placeholder vocabulary.
- [ ] K2 — doc-new-target-path (gap G3): explicit placement argument, containment-guarded.
- [ ] K3 — source-record-kind (gap G5): URL-plus-local-copy provenance kind.
- [ ] K4 — normative-statement-export (P5): the sentence classification as data.
- [ ] R5 — checker-internal-dedup (refactor slice 5, `src/date.rs`).
- [ ] R6 — required-meta-one-source (refactor slice 6, the high-value item).
- [ ] R7 — frontmatter-vocab-config (refactor slice 7; may slip past the release).
- [ ] Release v0.2.0 (RELEASING.md; the owner tags and publishes).

## Band 2 — decision gate (owner)

- [ ] D1 — slice-8 ADR scope (absorb entity/relation questions?).
- [ ] D2 — G7 lifecycle vocabularies versus status namespace.
- [ ] D3 — PR #87 splitter close-out fate.
- [ ] D4 — evidence granularity (statement versus section first).

## Band 3 — post-v0.2.0

Pending band 2; see the PLANS.md table (A1-A4 ontology/entity, B1-B2 evidence/provenance, C5 lifecycle, D6 crosswalk, D7 queries, X8 splitter consolidation).

- Owner note (2026-07-15) for the A-slices: the story-workflow coupling lint (US-WF-001/US-PER-001, PR #94) hardcodes today's persona/workflow ontology; when the ontology becomes configuration, this linting must become configurable with it.

## Context

The oracle retirement (refactor slice 4, task #78) completed 2026-07-15 via PR #88: the Rust engine owns every corpus check, `just verify` is the daily gate, and the pre-0.2.0 refactor program's remaining slices (R5-R7) fold into band 1 above.
