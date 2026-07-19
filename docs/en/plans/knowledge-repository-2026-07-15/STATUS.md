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
- [x] R7 — frontmatter-vocab-config (refactor slice 7): slipped past the release as planned and landed in band 3 below.
- [x] Release v0.2.0 — tagged and published by the owner 2026-07-16; band 1 closes, band 2 (the owner decision gate D1-D4) is next.

## Band 2 — decision gate (owner)

- [x] D1 — slice-8 ADR scope: merged with D2 into ADR-0017 (process profiles and the layered ontology); accepted by the owner 2026-07-19.
- [x] D2 — G7 lifecycle vocabularies versus status namespace: resolved by ADR-0017 — the guarded lifecycle stays core, domain states become declared controlled vocabularies.
- [x] D3 — PR #87 splitter close-out fate: option C (owner, 2026-07-17).
  PR #87 is closed, the four-splitter-contracts insight is rewritten in present tense into arc42 chapter 8, and X8 drops out of the band-3 table (consolidation without user-visible value; the chapter-8 note records why it stays unplanned).
- [x] D4 — evidence granularity: option A, statement-level anchors with sparse opt-in claims (owner, 2026-07-17).
  Owner note: paragraph- or section-level anchors (any heading depth, not only `##`) are expected to suffice in most cases, so the claim marker must anchor coarser blocks naturally, with statement precision available where a single sentence carries the evidence.
- Reframing note (owner, 2026-07-17): D1 and D2 are hard-coupled and widen into one design question — arqix as a catalog of configurable processes (hardwired rules acting as switches/gates, selected per project) plus a layered ontology (reserved core vocabulary, module vocabularies, project extension) that the checker validates.
  The A1 ADR takes that scope; ADR-0017 carries it, accepted 2026-07-19; band 3 is open.
- External decision input (2026-07-17): a second arqix-governed corpus (PSI language) filed structured feedback against the 0.2.0 build, recorded as the second intake in this package (FEEDBACK-2026-07-17-psi.md) and mapped into the roadmap ("Beyond 0.2.0 — toward 0.3.0").
  It supplies the second real-world use case whose absence was the D1 briefing's strongest contra, and it independently demands the configurable kinds, vocabularies, relations, namespaces, and gates the reframed ADR is about.

## Band 3 — post-v0.2.0

- [x] A1 — process-profiles/layered-ontology ADR: ADR-0017 accepted 2026-07-19.
- [x] R7 — frontmatter-vocab-config: US-08-01-29 + REQ-08-01-29-01/-02, spec-first red/green; `[frontmatter].section-kinds` and `.allowed-external-types` bind FM-007/ONT-002, built-in defaults unchanged — the first ADR-0017 implementation slice.
- [x] FR-A1 close-out: US-08-01-30 + REQ-08-01-30-01 pin the existing ONT-003 triple-object resolution (both directions); the reported gap turned out to be scanning scope — documents outside every configured `[kinds.<family>].dir` are not walked — recorded in the spec and the intake triage.
- [x] Module binding 1: US-08-01-31 + REQ-08-01-31-01, spec-first red/green; `[process].modules` selects the effective process modules, the coupling lint (US-WF-001/US-PER-001) runs exactly when story-driven is effective — the 2026-07-15 owner note on the A-slices is resolved.
- [x] FR-B1: US-08-01-32 + REQ-08-01-32-01, spec-first red/green; repeatable `--set key=value` on `doc new` and the aliases fills template placeholders, TPL-003 for unused keys.
- [x] FR-B2: US-08-01-33 + REQ-08-01-33-01/-02, spec-first red/green; `[kinds.<family>].id-template` and `dir-template` derive id and placement from `--set` values and the slug, `--id`/`--dir` stay overrides.
- [x] FR-A2 (digest half): US-08-01-34 + REQ-08-01-34-01, spec-first red/green; SRC-006 verifies the local copy's bytes against the recorded sha256 (missing copy or stale digest, one finding each); the configurable field set stays with the ontology work.
- [x] FR-C1: US-08-01-35 + REQ-08-01-35-01, spec-first red/green; `[kinds.<family>.vocab]` declares controlled vocabularies for named `properties` fields, FM-009 validates them — the ADR-0017 domain-state axis is machine-checked.
- [x] ONT meta rules: US-08-01-36 + REQ-08-01-36-01/-02, spec-first red/green; ONT-007 checks every edge against the declared rdfs.domain/range (subclass closure, declaration opts in), ONT-008 reports sub-class-of cycles beyond the root self-reference — the corpus's 33 declared domains/ranges hold over all 1476 edges.
- [x] FR-A3: US-08-01-37 + REQ-08-01-37-01, spec-first red/green; source-catalog.md as the tenth report unit (Q-11) under the snapshot drift gate.
- [x] FR-C3: US-08-01-38 + REQ-08-01-38-01, spec-first red/green; FM-010 renders the kind's dir-template back from each document's own properties and reports path/frontmatter disagreement — bounded contexts as one declared source, checked in both directions.
- [x] Rule hygiene: US-08-01-39 + REQ-08-01-39-01, spec-first red/green; taken ids are TPL-004, TPL-002 keeps unknown placeholders only; the rule catalog covers 79 rules.
- [x] B1 core: ADR-0018 (evidence anchors and derived triples) accepted 2026-07-19; US-08-01-40 + REQ-08-01-40-01/-02/-03, spec-first red/green — claim markers with CLM validation, the fmt-owned derived-triples lifting with drift reporting, and full graph participation of the derived supported-by edges (range: source).
- [x] FR-B3 close-out by investigation: duplicate id and iri are corpus-wide findings (FM-006, linter duplicate-id) and `doc new` checks id uniqueness at creation; the missing slug-per-context check needs the bounded-context concept (FR-C3) and moves there.

Band 2 is decided; the remaining PLANS.md table slices are A2-A4 (ontology/entity), B1-B2 (evidence/provenance), C5 (lifecycle), D6 (crosswalk), and D7 (queries).

- Owner note (2026-07-15) for the A-slices: the story-workflow coupling lint (US-WF-001/US-PER-001, PR #94) hardcodes today's persona/workflow ontology; when the ontology becomes configuration, this linting must become configurable with it.
- X8 (splitter consolidation) is removed from the planned slices per decision D3; the rationale lives in arc42 chapter 8 (line splitting and frontmatter parsing).

## Context

The oracle retirement (refactor slice 4, task #78) completed 2026-07-15 via PR #88: the Rust engine owns every corpus check, `just verify` is the daily gate, and the pre-0.2.0 refactor program's remaining slices (R5-R7) fold into band 1 above.
