---


id: US-05-01-05
title: Detect Translation Drift for Automation
slug: detect-translation-drift-for-automation
iri: arqix:user-stories/us-05-01-05

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-05
  - predicate: arqix:properties/has-requirement
    object:
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-05-01

properties:
  priority: high
  edge-case: false

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-03-30
  updated: 2026-04-04
  lang: en
  translation-of:
  generated: false
---


## Detect Translation Drift for Automation

As a Alex AIOps, I want an i18n lint profile that detects missing translations, outdated translations, and translation metadata mismatches, so that bilingual documentation can be used reliably by agents and downstream tooling.

### Acceptance Criteria

- [ ] Running the i18n lint profile produces violations for an EN source document without a required DE translation.
- [ ] Running the i18n lint profile produces violations for a DE translation whose `translation_of` cannot be resolved.
- [ ] Running the i18n lint profile produces violations for a DE translation whose `source_updated` is older than source `updated`.
- [ ] The set of required kinds or domains is configurable in `arqix.toml`.
- [ ] Diagnostics include stable codes and source locations.
- [ ] The i18n lint profile can be used as a CI gate with exit code 1 on violations.

### Notes

The i18n lint profile should focus on repository metadata and translation state rather than translation quality. In practice, `arqix lint run --profile i18n` should verify that required translations exist for the configured target languages and kinds or domains, that translation links are resolved correctly, and that translation metadata stays in sync with the source document. Optional markup safety checks can also help ensure that arqix markers remain unchanged across languages. Diagnostics should remain machine-readable through `--format json` and deterministic across repeated runs. The main value for Alex is deterministic bilingual metadata for search, read, and automation.
