---
title: "Requirements derivation plan"
date: 2026-07-02
status: draft
category: docs
branch: docs/add-requirements
---

# Plan

## Summary

Derive requirement documents from the 115 user stories in `docs/en/architecture/stories`, place them in a new requirements directory under the architecture documentation layout, and populate the currently empty `has-requirement` triples in the story frontmatter.

This plan is written before implementation; it records the intended approach and the open decisions that need human review first.

## Scope

- In scope:
  - a requirements directory under `docs/en/architecture/` (proposed: `docs/en/architecture/requirements/`)
  - requirement documents created from `docs/en/templates/requirement.tpl.md`
  - populating `has-requirement` triples in the story files
  - remapping the historic mapping report content to the current ID scheme
- Out of scope:
  - changes to story content (actor sentences, acceptance criteria, notes)
  - changes to personas, workflows, templates, or the ontology beyond what requirement linking strictly needs
  - implementation of any arqix tooling

## Branch context

- Branch: `docs/add-requirements`, created from `docs/add-personas-user-stories`
- Depends on PR #5 (`docs/add-personas-user-stories` → `main`); this branch should be rebased onto `main` once PR #5 merges
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

## Proposed ID and layout scheme

- Directory: `docs/en/architecture/requirements/`
- Requirement ID: `REQ-XX-YY-ZZ-NN`, where `US-XX-YY-ZZ` is the owning story and `NN` numbers the requirements of that story starting at `01`
- Filename: `REQ-XX-YY-ZZ-NN-slug.md`, mirroring the story filename convention
- IRI: `arqix:requirements/req-xx-yy-zz-nn`
- Story frontmatter: `has-requirement` lists one object per derived requirement
- Requirement frontmatter: a `derived-from` triple pointing back to the owning story IRI

This mirrors the `REQ-US-1001-01` idea from the historic report while adopting the current story ID scheme. The scheme is a proposal and needs confirmation before execution.

## Execution steps

1. Confirm the ID scheme, directory, and backlink predicate (open questions below).
2. Build a remapping table: historic report rows → current `US-XX-YY-ZZ` stories, using the story titles and slugs to match old `US-1001`-style IDs to current files.
3. For stories covered by the historic report, draft requirements from the mapped historic requirement titles.
4. For stories without historic requirements, derive requirements from the acceptance criteria (one requirement per independently verifiable behaviour; not necessarily one per checkbox).
5. Create the requirement files from `requirement.tpl.md`, with `fit-criterion` distilled from the story acceptance criteria.
6. Populate `has-requirement` in each story and refresh `meta.updated` on touched stories.
7. Extend the mechanical consistency check to cover requirements (ID ↔ filename ↔ IRI ↔ slug, `derived-from` ↔ `has-requirement` symmetry, no orphans, no dangling references).
8. Keep each pass a focused, reviewable commit.

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
- If PR #5 changes before merge, this branch must be rebased and the baseline re-verified.

## Open questions (decide before execution)

1. Requirement ID scheme: `REQ-XX-YY-ZZ-NN` as proposed, or a story-independent scheme (e.g. `REQ-NNNN`) that survives story renumbering?
2. Directory: `docs/en/architecture/requirements/` as proposed?
3. Backlink predicate: `derived-from`, or should the ontology gain a dedicated `derives-from-story` / reuse `realizes-user-story`?
4. Should shared behaviour across personas (e.g. "deterministic exit codes" appears in groups 04 and 08) produce one shared requirement referenced by several stories, or per-story requirements?
5. Does the historic report content carry over as-is where it matches, or is it only an input for fresh derivation?
