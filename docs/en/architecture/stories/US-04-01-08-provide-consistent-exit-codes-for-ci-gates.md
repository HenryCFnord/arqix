---
id: US-04-01-08
title: Provide Consistent Exit Codes for CI Gates
slug: provide-consistent-exit-codes-for-ci-gates
iri: arqix:user-stories/us-04-01-08

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-04
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-00-00-00-02
      - arqix:requirements/req-04-01-08-01
      - arqix:requirements/req-04-01-08-02
      - arqix:requirements/req-04-01-08-03
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-04-01

properties:
  priority: high
  edge-case: false

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-03-30
  updated: 2026-07-02
  lang: en
  translation-of:
  generated: false
---

## Provide Consistent Exit Codes for CI Gates

As a DevOps engineer, I want consistent exit codes and CI support, so that automation can react to arqix reliably.

### Acceptance Criteria

- [ ] Exit codes are consistent: `0` for success, `1` for lint or quality-gate failure, and `2` for usage error.
- [ ] Stable stderr messaging lets CI distinguish command errors from quality failures.
- [ ] A minimal GitHub Actions template may be provided for typical gates, aligned with supported commands only.
- [ ] Exit-code behavior is deterministic across repeated runs on the same input.

### Notes

This story is done when CI can distinguish usage errors from quality gate failures solely through exit status and stable stderr messaging.
Add tests that exercise each documented exit code and verify that lint failures do not collapse into generic command errors.
If a GitHub Actions template is shipped, keep it minimal and aligned with the supported commands only.
This is a core CI contract capability.
