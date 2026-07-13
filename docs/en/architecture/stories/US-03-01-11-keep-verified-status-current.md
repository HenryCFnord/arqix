---
id: US-03-01-11
title: Keep Verified Status Current
slug: keep-verified-status-current
iri: arqix:user-stories/us-03-01-11

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-10
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-03-01-11-01
      - arqix:requirements/req-03-01-11-02
      - arqix:requirements/req-03-01-11-03
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-03-01

properties:
  priority: medium
  edge-case: false

external-references: []

meta:
  lifecycle-status: done
  owner: hcf
  created: 2026-07-13
  updated: 2026-07-13
  lang: en
  translation-of:
  generated: false
---

## Keep Verified Status Current

As an assessor, I want a marker flagged as possibly stale when the requirement it verifies changed after the verifying test, so that a green "verified" reflects the current requirement and not a historical marker placement.

### Acceptance Criteria

- [x] `arqix trace freshness` reports an active `verifies` or `implements` marker as possibly stale when its target requirement was committed to version control after the marker's own file.
- [x] Freshness is computed from version-control history through a single isolated lookup, with the decision itself a pure function over commit timestamps, so unit tests inject timestamps and stay deterministic.
- [x] When version-control history is unavailable, `arqix trace freshness` reports nothing stale and exits successfully.
- [x] `arqix verify` runs freshness as an informational sub-step, so stale markers are surfaced without failing the loop.

### Notes

Freshness is git arithmetic, not code analysis: it compares the last commit that touched the marker's file against the last commit that touched the target requirement (ADR-0015).
The owning story is not compared — dogfooding showed stories churn (batch commits, lifecycle bumps, sibling-requirement edits) for reasons unrelated to a specific test, so a story-level comparison flagged 62 of 63 markers as noise while the requirement comparison left a small, reviewable signal.
Granularity is file-level; line-level attribution (`git log -L`) is named as future hardening, which is why the signal is "possibly stale" and the sub-step is informational rather than gating.
The Python trace oracle stays frozen: freshness is a new engine-only analysis (`trace freshness`), so `trace scan/coverage/matrix` output is unchanged and conformance is untouched, mirroring the `--results` and `ratchet` precedents.
