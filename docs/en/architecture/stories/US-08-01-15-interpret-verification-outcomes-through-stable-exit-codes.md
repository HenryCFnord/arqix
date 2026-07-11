---
id: US-08-01-15
title: Interpret Verification Outcomes through Stable Exit Codes
slug: interpret-verification-outcomes-through-stable-exit-codes
iri: arqix:user-stories/us-08-01-15

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-08
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-00-00-00-02
      - arqix:requirements/req-04-01-08-01
      - arqix:requirements/req-04-01-08-02
      - arqix:requirements/req-04-01-08-03
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-08-01

properties:
  priority: medium
  edge-case: false

external-references: []

meta:
  lifecycle-status: retired
  owner: hcf
  created: 2026-03-30
  updated: 2026-07-11
  lang: en
  translation-of:
  generated: false
---

## Interpret Verification Outcomes through Stable Exit Codes

As a coding agent, I want consistent exit codes and stable error signaling, so that I can react to arqix outcomes deterministically without guessing whether a failure is a usage problem or a quality-gate result.

### Acceptance Criteria

- [ ] Exit codes are consistent: `0` for success, `1` for lint or quality-gate failure, and `2` for usage error.
- [ ] Stable stderr messaging lets CI distinguish command errors from quality failures.
- [ ] A minimal GitHub Actions template may be provided for typical gates, aligned with supported commands only.
- [ ] Exit-code behavior is deterministic across repeated runs on the same input.

### Notes

This story is done when CI can distinguish usage errors from quality gate failures solely through exit status and stable stderr messaging.
Add tests that exercise each documented exit code and verify that lint failures do not collapse into generic command errors.
If a GitHub Actions template is shipped, keep it minimal and aligned with the supported commands only.
The main value for Casey is machine-safe interpretation of command outcomes.

Retired in the consolidation sweep of 2026-07-11: this story is a persona clone — its non-cross-cutting requirements are canonically owned by US-04-01-08, and the requirements' derived-from provenance keeps this story's contribution on record.
