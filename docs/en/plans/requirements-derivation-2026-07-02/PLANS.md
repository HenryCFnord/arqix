---
title: "Requirements derivation plan"
date: 2026-07-02
status: reviewed
category: docs
branch: docs/add-requirements
---

# Plan

## Summary

Derive requirement documents from the 103 user stories in `docs/en/architecture/stories`, place them in a new requirements directory under the architecture documentation layout, and populate the currently empty `has-requirement` triples in the story frontmatter.

This plan is written before implementation; it records the intended approach and the open decisions that need human review first.

## Scope

- In scope:
  - a requirements directory under `docs/en/architecture/req/`
  - requirement documents created from `docs/en/templates/requirement.tpl.md`
  - populating `has-requirement` triples in the story files
  - remapping the historic mapping report content to the current ID scheme
- Out of scope:
  - changes to story content (actor sentences, acceptance criteria, notes)
  - changes to personas, workflows, templates, or the ontology beyond what requirement linking strictly needs
  - implementation of any arqix tooling

## Branch context

- Branch: `docs/add-requirements`, rebased onto `main` after PR #5 (`docs/add-personas-user-stories` → `main`) merged as `d1efff1`
- The consistency fixes in commit `7e3a6c8` are the baseline: stable IDs, IRIs, slugs, persona and workflow links across all 103 stories

## Current state

- All 103 stories carry an empty `has-requirement` triple (uniform, verified 2026-07-02)
- No requirements directory exists under `docs/en/architecture/`
- `docs/en/templates/requirement.tpl.md` exists and defines the requirement frontmatter contract (`arqix:classes/requirement`, `has-verification-method` triple, `fit-criterion` property)
- The historic report `docs/en/plans/persona-remapping-2026-03-25/persona_us_req_mapping_report.md` maps 50 requirements to stories, but in the superseded schemes:
  - story IDs like `US-1001` instead of `US-01-01-NN`
  - requirement IDs like `REQ-US-1001-01`
  - paths `docs/us/` and `docs/req/` that no longer exist
- Relevant ontology properties already exist: `has-requirement`, `implements-requirement`, `verifies-requirement`, `derived-from`

## ID and layout scheme (decided 2026-07-02)

- Directory: `docs/en/architecture/req/`
- Requirement ID: `REQ-XX-YY-ZZ-NN`, where `US-XX-YY-ZZ` is the owning story and `NN` numbers the requirements of that story starting at `01`
- Cross-cutting requirements shared by several stories use the reserved pseudo-story domain `00-00-00`: `REQ-00-00-00-NN`, numbered contiguously starting at `01` (persona groups start at `01`, so `00` cannot collide)
- Filename: `REQ-XX-YY-ZZ-NN-slug.md`, mirroring the story filename convention
- IRI: `arqix:requirements/req-xx-yy-zz-nn`
- Story frontmatter: `has-requirement` lists one object per derived requirement (including shared `REQ-00-00-00-NN` requirements)
- Requirement frontmatter: a `derived-from` triple pointing back to the owning story IRI; cross-cutting requirements carry one `derived-from` object per contributing story

This mirrors the `REQ-US-1001-01` idea from the historic report while adopting the current story ID scheme.

## Execution steps

0. Foundations (done 2026-07-02): ontology subclasses, requirements style guide (RFC 2119 subset + EARS), updated requirement template, and the consistency checker `scripts/check_requirements.py`.
1. Identify behaviour shared across stories and propose cross-cutting `REQ-00-00-00-NN` candidates for review (done 2026-07-02, see [CROSS-CONCERNS.md](CROSS-CONCERNS.md)); create the approved candidates as the first requirement files.
2. Derive requirements per story freshly from the current acceptance criteria (one requirement per independently verifiable behaviour; not necessarily one per checkbox), written per the style guide and typed as functional, quality, or constraint. Where a story's behaviour is already covered by a cross-cutting requirement, link it instead of duplicating it.
3. Cross-check granularity against the historic report where old stories map to current ones (e.g. old `US-1001` → 3 requirements), without carrying over content.
4. Create the requirement files under `docs/en/architecture/req/` from `requirement.tpl.md`, with `fit-criterion` distilled from the story acceptance criteria and `derived-from` pointing at the owning story or stories.
5. Populate `has-requirement` in each story and refresh `meta.updated` on touched stories.
6. Run `scripts/check_requirements.py` after each pass; it enforces ID ↔ filename ↔ IRI ↔ slug alignment, `derived-from` ↔ `has-requirement` symmetry, contiguous numbering, subclass typing, and the EARS/RFC 2119 sentence rules.
7. Keep each pass a focused, reviewable commit.

## Expected validation checks

- every story has at least one non-empty `has-requirement` object, or a documented reason why not
- every referenced requirement file exists; no dangling IRIs in either direction
- requirement IDs are unique, contiguous per story, and match filenames and IRIs
- requirement frontmatter validates against the template contract
- `has-requirement` / `derived-from` pairs are symmetric
- touched story files remain valid against the story template contract

## Risks and caveats

- The historic report is 2026-03-25 state; stories were renumbered and extended since, so matching by title/slug needs human spot-checks.
- Deriving requirements from acceptance criteria is judgement work, not mechanical; granularity decisions need review.
- 103 stories will produce a large file count; the PR should stay requirements-only to remain reviewable.

## Decisions (2026-07-02)

All five open questions from the draft plan have been decided by the repository owner:

1. Requirement ID scheme: `REQ-XX-YY-ZZ-NN`, bound to the owning story ID.
2. Directory: `docs/en/architecture/req/`.
3. Backlink predicate: `derived-from`; no new ontology property is introduced.
4. Shared behaviour across personas becomes a single cross-cutting requirement in the reserved pseudo-story domain `00-00-00` (`REQ-00-00-00-NN`). Linking to the contributing stories is handled by `derived-from` with multiple objects, and each contributing story lists the shared requirement in `has-requirement`.
5. Requirements are derived freshly from the current acceptance criteria. The historic report is input only, used as a granularity cross-check: its old requirement files no longer exist in the repository, its titles are German while `docs/en/` is English, and it covers only a fraction of the current 103 stories.

## Tooling decisions (2026-07-02, second round)

Decided by the repository owner before derivation starts:

6. Requirement sentences follow a strict RFC 2119 subset (`SHALL`, `SHALL NOT`, `SHOULD`, `SHOULD NOT`, `MAY`, uppercase) combined with the EARS sentence patterns. The authoring contract lives in `docs/en/processes/requirements-style-guide.md`.
7. Requirements are typed by verification approach through three ontology subclasses of `arqix:classes/requirement`: `functional-requirement` (directly testable, linked to tests), `quality-requirement` (verified via acceptance criteria and review), and `constraint` (not directly testable, frames other requirements). Each requirement document sets `rdf.type` to exactly one subclass.
8. A regex-based consistency checker (`scripts/check_requirements.py`, Python 3 stdlib-only) enforces the structure, link symmetry, and sentence rules with stable rule IDs, `--json` output, and `--selftest`. It is the reference specification for the later arqix (Rust) implementation.
9. Cross-cutting concerns are identified and reviewed before per-story derivation; the candidate list lives in [CROSS-CONCERNS.md](CROSS-CONCERNS.md).
10. Canonical-owner model (decided 2026-07-02 before deriving groups 02–08): one behaviour is specified by exactly one requirement, owned by the lowest-ID story that demands it; further demanding stories extend `derived-from` and link via `has-requirement`. Rationale: 150 of 177 distinct acceptance-criteria behaviours are shared by 2–4 persona stories (the normalization created per-persona copies of the same features); per-story duplication would create 200+ parallel requirements, and lifting everything into the `00-00-00` domain would dissolve both the foundation semantics and the story-anchored ID scheme. Checker rule REQ-LNK-001 was relaxed accordingly (owner must be the first `derived-from` object).
