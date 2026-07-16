---
id: US-07-01-10
title: Report the Test-Suite Code Coverage
slug: report-the-test-suite-code-coverage
iri: arqix:user-stories/us-07-01-10

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-10
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-07-01-10-01
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-07-01

properties:
  priority: medium
  edge-case: false

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-16
  updated: 2026-07-16
  lang: en
  translation-of:
  generated: false
---

## Report the Test-Suite Code Coverage

As an assessor, I want the test suite's code coverage reported as a unit, so that question Q-10 of the report catalog has a generated answer next to the requirement-coverage scoreboard.

### Acceptance Criteria

- [ ] `arqix report coverage` renders the test-coverage unit from a cargo-llvm-cov JSON summary: total line/function/region coverage plus a per-file table.
- [ ] The unit is CI-generated and stays outside the snapshot freshness gate — coverage data is toolchain-dependent and not locally regenerable without the instrumented run.
- [ ] A `just coverage` recipe runs the instrumented tests and renders the unit locally; CI refreshes the committed unit on the default branch.

### Notes

The projection is arqix's (deterministic given the input file); the measurement is cargo-llvm-cov's — the split keeps the byte-for-byte snapshot gate honest.
Joining coverage against the verifies edges (per-requirement code coverage) stays a band-3 candidate.
