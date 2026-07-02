---
title: "Requirements derivation status"
date: 2026-07-02
status: ready-for-execution
branch: docs/add-requirements
plan_dir: docs/en/plans/requirements-derivation-2026-07-02
---

# Requirements derivation status

## Branch

`docs/add-requirements` (created from `docs/add-personas-user-stories`)

## Current state

- Planning package created under `docs/en/plans/requirements-derivation-2026-07-02/`
- Baseline verified on 2026-07-02: all 103 stories in `docs/en/architecture/stories` are structurally consistent (commit `7e3a6c8`) and all carry an empty `has-requirement` triple
- PR #5 (`docs/add-personas-user-stories` → `main`) merged on 2026-07-02 as `d1efff1`; this branch has been rebased onto `main`, so the story baseline is final
- All plan decisions are recorded in [PLANS.md](PLANS.md) ("Decisions" and "Tooling decisions")
- Foundations are in place (2026-07-02):
  - ontology subclasses `functional-requirement`, `quality-requirement`, `constraint` under `docs/ontology/classes/`
  - authoring contract in `docs/en/processes/requirements-style-guide.md` (RFC 2119 subset + EARS)
  - requirement template updated to the `REQ-xx-yy-zz-nn` scheme with `derived-from`
  - consistency checker `scripts/check_requirements.py` (selftest green; strict repo run reports 103 unlinked-story warnings as expected)
- All ten cross-cutting candidates were approved and exist as `REQ-00-00-00-01..10` under `docs/en/architecture/req/`, with full verified `derived-from` lists; 92 stories carry the corresponding `has-requirement` backlinks (see [CROSS-CONCERNS.md](CROSS-CONCERNS.md) for the curation record)
- Pilot derivation for persona group 01 is complete: 50 story-bound requirements (`REQ-01-01-ZZ-NN`; 30 functional, 7 quality, 13 constraint) derived from the acceptance criteria of all 16 Mara Maintainer stories
- `scripts/check_requirements.py --allow-unlinked-stories` passes with zero errors and zero warnings; the strict run reports exactly the 8 stories in groups 02–08 that no requirement links yet (US-04-01-09, US-04-01-13, US-05-01-12, US-06-01-07, US-06-01-11, US-08-01-12, US-08-01-17, US-08-01-18)

- Second checker added (2026-07-02): `scripts/check_frontmatter.py` validates frontmatter consistency, canonical formatting, and ontology vocabulary (predicates, classes, IRI resolution) across `docs/en/architecture/` and `docs/ontology/`; its first run surfaced and led to fixing 16 legacy inconsistencies (broken WF iris/ids, wrong `arqix:persona/` namespace, empty meta fields, plural class filenames, missing `is-part-of-workflow` property, heading levels, missing trailing newlines). Remaining known warnings: 8 undefined `owl.inverse-of` targets (deliberate, suppressed via `--allow-undefined-inverse`)
- Agent-agnostic generalisation (2026-07-02): US-01-01-09, US-08-01-18, US-01-01-15, US-04-01-09, and US-08-01-17 no longer hardcode `AGENTS.md`/`PLANS.md` or name Codex; they speak of the agent instruction document, the plan document, and coding agents. REQ-01-01-09-01..04 were reworded and renamed accordingly, REQ-01-01-09-05/-06 (extension points documented / free of process rules) were added, and ADR-0001 (`docs/en/architecture/adr/`) fixes the concrete file mapping: `AGENTS.md` canonical, `CLAUDE.md` as thin adapter, skills and prompt libraries as non-normative extension points

## Next recommended action

- Human-review the group-01 pilot in `docs/en/architecture/req/REQ-01-01-*`: granularity (1–5 requirements per story), kind assignment, sentence and fit-criterion quality
- After pilot approval: derive groups 02–08 in the same manner, one commit per group (US-08-01-17 and US-08-01-18 on the basis of their generalised acceptance criteria)

## Blockers

- Pilot review is the gate for deriving groups 02–08

## Notes

- Package files:
  - [IDEA.md](IDEA.md)
  - [PLANS.md](PLANS.md)
  - [STATUS.md](STATUS.md)
- The historic mapping report remains at `docs/en/processes/persona_us_req_mapping_report.md` and is treated as input, not as authoritative current-state
