---
id: REQ-04-01-01-01
title: Collect Assembly Log as CI Artefact
slug: collect-assembly-log-as-ci-artefact
iri: arqix:requirements/req-04-01-01-01

rdf:
  type:
    - arqix:classes/quality-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-04-01-01
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: A CI job can archive the log file as-is and downstream steps can consume it directly.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-02
  updated: 2026-07-10
  lang: en
  translation-of:
  generated: false
---

## Requirement

The assembly log SHOULD be collectable as a CI artefact without post-processing or field-name guessing.

### Notes

Derived from the acceptance criteria of US-04-01-01 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10).
Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
