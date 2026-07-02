---
id: US-08-01-11
title: Interpret i18n Lint Results Deterministically
slug: interpret-i18n-lint-results-deterministically
iri: arqix:user-stories/us-08-01-11

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-08
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-00-00-00-02
      - arqix:requirements/req-00-00-00-03
      - arqix:requirements/req-00-00-00-06
      - arqix:requirements/req-00-00-00-10
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-08-01

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


## Interpret i18n Lint Results Deterministically

As a coding agent, I want an i18n lint profile that detects missing translations, outdated translations, and translation metadata mismatches, so that I can act on bilingual documentation issues without guesswork.

### Acceptance Criteria

- [ ] Running the i18n lint profile produces violations for an EN source document without a required DE translation.
- [ ] Running the i18n lint profile produces violations for a DE translation whose `translation_of` cannot be resolved.
- [ ] Running the i18n lint profile produces violations for a DE translation whose `source_updated` is older than source `updated`.
- [ ] The set of required kinds or domains is configurable in `arqix.toml`.
- [ ] Diagnostics include stable codes and source locations.
- [ ] The i18n lint profile can be used as a CI gate with exit code 1 on violations.

### Notes

The i18n lint profile should focus on repository metadata and translation state rather than translation quality. In practice, `arqix lint run --profile i18n` should verify that required translations exist for the configured target languages and kinds or domains, that translation links are resolved correctly, and that translation metadata stays in sync with the source document. Optional markup safety checks can also help ensure that arqix markers remain unchanged across languages. Diagnostics should remain machine-readable through `--format json` and deterministic across repeated runs. The main value for Casey is deterministic diagnostics and clear automation stop conditions.
