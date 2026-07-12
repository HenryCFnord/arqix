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
      - arqix:requirements/req-01-01-18-04
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-01-01

properties:
  priority: high
  edge-case: false

external-references: []

meta:
  lifecycle-status: done
  owner: hcf
  created: 2026-07-11
  updated: 2026-07-11
  lang: en
  translation-of:
  generated: false
---

## Configure the ID Policy

As a maintainer, I want the document ID shapes declared in configuration and all relations resolved from the declared triples, so that a repository adopting arqix can bring its own numbering without forking the tool.

### Acceptance Criteria

- [x] `arqix.toml` declares an `id-pattern` per document family; `doc new` generates the next ID from it and the checkers validate shape and uniqueness against it.
- [x] The trace graph resolves a requirement's owning story from its first `derived-from` triple; a group-free pattern yields a complete graph.
- [x] Where the pattern declares semantic groups (`story`, `seq`), consistency checks run: encoded values against declared triples, per-story sequencing.
- [x] The Rust engine and the Python reference tools read the same configured policy; oracle conformance holds under a non-default, group-free pattern.
- [x] Without configuration, the defaults reproduce the current ID shapes and checks, and the existing corpus is unchanged.

### Notes

The model — declared triples as the source of truth for relations, the ID as an opaque label, named groups activating optional consistency checks — is decided in ADR-0012; the configuration boundary it lives inside is ADR-0011.
This story carries the audit rows C3, C4, C15, and C16 (per-kind ID scheme, IRI namespaces, requirement- and story-ID shapes).
Cross-cutting requirements become explicitly declared (ontology marker) rather than recognised by the `00-00-00` ID domain; the domain stays as the default policy's naming convention.
The one-source rule (ADR-0011) is an acceptance criterion here, not an implementation detail: a policy read by the engine but not the oracle breaks the conformance suite.
