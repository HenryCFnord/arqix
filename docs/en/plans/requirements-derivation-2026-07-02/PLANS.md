---
title: "Requirements derivation plan"
date: 2026-07-02
status: reviewed
category: docs
branch: docs/add-requirements
---

# Plan

## Summary

Derive requirement documents from the 115 user stories in `docs/en/architecture/stories`, place them in a new requirements directory under the architecture documentation layout, and populate the currently empty `has-requirement` triples in the story frontmatter.

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
- The consistency fixes in commit `7e3a6c8` are the baseline: stable IDs, IRIs, slugs, persona and workflow links across all 115 stories

## Current state

- All 115 stories carry an empty `has-requirement` triple (uniform, verified 2026-07-02)
- No requirements directory exists under `docs/en/architecture/`
- `docs/en/templates/requirement.tpl.md` exists and defines the requirement frontmatter contract (`arqix:classes/requirement`, `has-verification-method` triple, `fit-criterion` property)
- The historic report `docs/en/processes/persona_us_req_mapping_report.md` maps 50 requirements to stories, but in the superseded schemes:
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

1. Derive requirements per story freshly from the current acceptance criteria (one requirement per independently verifiable behaviour; not necessarily one per checkbox).
2. Identify behaviour shared across stories and lift it into cross-cutting `REQ-00-00-00-NN` requirements instead of duplicating it per story.
3. Cross-check granularity against the historic report where old stories map to current ones (e.g. old `US-1001` → 3 requirements), without carrying over content.
4. Create the requirement files under `docs/en/architecture/req/` from `requirement.tpl.md`, with `fit-criterion` distilled from the story acceptance criteria and `derived-from` pointing at the owning story or stories.
5. Populate `has-requirement` in each story and refresh `meta.updated` on touched stories.
6. Extend the mechanical consistency check to cover requirements (ID ↔ filename ↔ IRI ↔ slug, `derived-from` ↔ `has-requirement` symmetry, no orphans, no dangling references).
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
- 115 stories will produce a large file count; the PR should stay requirements-only to remain reviewable.

## Decisions (2026-07-02)

All five open questions from the draft plan have been decided by the repository owner:

1. Requirement ID scheme: `REQ-XX-YY-ZZ-NN`, bound to the owning story ID.
2. Directory: `docs/en/architecture/req/`.
3. Backlink predicate: `derived-from`; no new ontology property is introduced.
4. Shared behaviour across personas becomes a single cross-cutting requirement in the reserved pseudo-story domain `00-00-00` (`REQ-00-00-00-NN`). Linking to the contributing stories is handled by `derived-from` with multiple objects, and each contributing story lists the shared requirement in `has-requirement`.
5. Requirements are derived freshly from the current acceptance criteria. The historic report is input only, used as a granularity cross-check: its old requirement files no longer exist in the repository, its titles are German while `docs/en/` is English, and it covers only a fraction of the current 115 stories.
