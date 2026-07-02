---
id: US-04-01-11
title: Inspect Effective Config for CI Reproducibility
slug: inspect-effective-config-for-ci-reproducibility
iri: arqix:user-stories/us-04-01-11

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-04
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-00-00-00-01
      - arqix:requirements/req-00-00-00-06
      - arqix:requirements/req-01-01-16-01
      - arqix:requirements/req-01-01-16-02
      - arqix:requirements/req-01-01-16-03
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


## Inspect Effective Config for CI Reproducibility

As a DevOps engineer, I want to validate repository configuration and inspect the effective config, so that CI workflows use a deterministic baseline and configuration problems fail clearly.

### Acceptance Criteria

- [ ] `arqix config validate` reports schema and contract violations deterministically.
- [ ] `arqix config show` renders the effective configuration after defaults and overrides are applied.
- [ ] Diagnostics identify the failing key and source file when possible.

### Notes

Acceptance should verify both validation and effective configuration rendering on realistic multi-layer setups with defaults and overrides. Add tests for invalid keys, invalid values, and precedence resolution. Keep diagnostics stable so automation and CI can compare failures reliably. The main value for Daria is reproducible pipeline behaviour and clear diagnostics.
