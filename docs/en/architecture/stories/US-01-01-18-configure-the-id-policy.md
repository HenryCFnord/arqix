---
id: US-01-01-18
title: Configure the ID Policy
slug: configure-the-id-policy
iri: arqix:user-stories/us-01-01-18

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-01
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-00-00-00-04
      - arqix:requirements/req-01-01-18-01
      - arqix:requirements/req-01-01-18-02
      - arqix:requirements/req-01-01-18-03
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-01-01

properties:
  priority: high
  edge-case: false

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-11
  updated: 2026-07-11
  lang: en
  translation-of:
  generated: false
---

## Configure the ID Policy

As a maintainer, I want the document ID shapes and their meaning declared in configuration, so that a repository adopting arqix can bring its own numbering without forking the tool.

### Acceptance Criteria

- [ ] `arqix.toml` declares an `id-pattern` with named groups per document family; `doc new` generates and the checkers validate against it.
- [ ] The trace graph derives the owner-story slice, the cross-cutting domain, and per-story sequencing from the configured named groups, not from fixed offsets.
- [ ] The Rust engine and the Python reference tools read the same configured policy; oracle conformance holds under a non-default pattern.
- [ ] Without configuration, the defaults reproduce the current ID shapes and the existing corpus is unchanged.

### Notes

The model — named-group patterns as the single artefact for validation and derivation — is decided in ADR-0012; the configuration boundary it lives inside is ADR-0011.
This story carries the audit rows C3, C4, C15, and C16 (per-kind ID scheme, IRI namespaces, requirement- and story-ID shapes with the derivation model).
The one-source rule (ADR-0011) is an acceptance criterion here, not an implementation detail: a policy read by the engine but not the oracle breaks the conformance suite.
