---



id: US-01-01-15
title: Operationalise the Release Process with SemVer
slug: operationalise-the-release-process-with-semver
iri: arqix:user-stories/us-01-01-15

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
  priority: medium
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


## Operationalise the Release Process with SemVer

As a maintainer, I want a documented, Codex-friendly release process using SemVer, so that releases can be prepared traceably and executed safely.

### Acceptance Criteria

- [ ] `CHANGELOG.md` and `RELEASING.md` exist and are consistent.
- [ ] SemVer rules for product version and separate `config_version` and `schema_version` are documented.
- [ ] Codex is limited to release preparation only, with no tagging or publishing without approval.
- [ ] Breaking changes require migration notes and changelog entries.

### Notes

Acceptance should include a dry-run style checklist that a reviewer can follow from changelog preparation through release approval without hidden steps. Add documentation examples for patch, minor, and major releases, including when `config_version` or `schema_version` must change independently. Keep the process explicit about final manual approval points for tagging and publishing. The main value for Mara is governed, reviewable release preparation.
