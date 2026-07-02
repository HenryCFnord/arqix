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
- Cross-cutting concern candidates identified in [CROSS-CONCERNS.md](CROSS-CONCERNS.md) (ten candidates, EARS drafts validated against the checker)
- No requirement files exist yet; `has-requirement` is still unpopulated

## Next recommended action

- Human-review [CROSS-CONCERNS.md](CROSS-CONCERNS.md): approve, drop, or merge the ten candidates and confirm their kinds
- After approval: create the accepted `REQ-00-00-00-NN` files, then derive story-bound requirements per the execution steps in [PLANS.md](PLANS.md)

## Blockers

- CROSS-CONCERNS.md review is the gate for creating the first requirement files

## Notes

- Package files:
  - [IDEA.md](IDEA.md)
  - [PLANS.md](PLANS.md)
  - [STATUS.md](STATUS.md)
- The historic mapping report remains at `docs/en/processes/persona_us_req_mapping_report.md` and is treated as input, not as authoritative current-state
