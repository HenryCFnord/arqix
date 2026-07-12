---
id: US-03-01-10
title: Track Planned and Executed Test Evidence
slug: track-planned-and-executed-test-evidence
iri: arqix:user-stories/us-03-01-10

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-10
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-03-01-10-01
      - arqix:requirements/req-03-01-10-02
      - arqix:requirements/req-03-01-10-03
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-03-01

properties:
  priority: medium
  edge-case: false

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-12
  updated: 2026-07-12
  lang: en
  translation-of:
  generated: false
---

## Track Planned and Executed Test Evidence

As an assessor, I want planned tests to be claimable in any language and executed test outcomes to join the trace graph, so that "verified" reflects a green test rather than the mere existence of a claim — also in teams that do not work test-driven.

### Acceptance Criteria

- [ ] An `arqix:plans` marker claims a requirement as planned in every scanned comment surface, without relying on framework-specific skip attributes.
- [ ] `arqix trace coverage --results <report>` joins test outcomes from a JUnit XML report to the verifying markers by test name.
- [ ] A verifying claim whose joined outcome is failed or skipped does not count as verified; the requirement stays verified only while a passing (or unjoined) active claim remains.
- [ ] Coverage rows carry the joined outcomes so downstream exports can show pass/fail evidence.

### Notes

The `verifies` marker plus Rust's `#[ignore]` covers the red-skeleton discipline for Rust only; the `plans` marker makes the planned state declarable without framework syntax (owner decision 2026-07-12).
Broadening the scanned file set beyond `.rs` and `.md` is the separate configurable-markers thread (REQ-03-01-05-01), not this story.
JUnit XML is the join format because every mainstream runner emits it (cargo-nextest, pytest, jest, gotestsum); outcomes join by the test-case `name` attribute matching the marker's attached test name.
Markers without an entry in the report stay untouched: a results file refines the picture, it never invents evidence.
The Python oracle stays at the frozen pre-results surface (`--results` is engine-only); the `plans` marker itself is ported to the oracle so `trace scan` stays value-equal.
