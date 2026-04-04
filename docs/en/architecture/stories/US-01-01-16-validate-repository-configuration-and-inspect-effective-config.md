---

id: US-01-01-16
title: Validate repository configuration and inspect effective config
slug: validate-repository-configuration-and-inspect-effective-config
iri: arqix:user-stories/us-01-01-16

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-01
  - predicate: arqix:properties/has-requirement
    object:
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-01-01

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

## User-story

As a Mara Maintainer, I want to validate repository configuration and inspect the effective config, so that documentation rules stay consistent and automation has a deterministic baseline.

### Acceptance Criteria

- [ ] `arqix config validate` reports schema and contract violations deterministically.
- [ ] `arqix config show` renders the effective configuration after defaults and overrides are applied.
- [ ] Diagnostics identify the failing key and source file when possible.

### Notes

Acceptance should verify both validation and effective configuration rendering on realistic multi-layer setups with defaults and overrides. Add tests for invalid keys, invalid values, and precedence resolution. Keep diagnostics stable so automation and CI can compare failures reliably. The main value for Mara is baseline governance and deterministic configuration management.
