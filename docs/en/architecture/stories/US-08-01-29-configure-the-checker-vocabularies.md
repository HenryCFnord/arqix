---
id: US-08-01-29
title: Configure the Checker Vocabularies
slug: configure-the-checker-vocabularies
iri: arqix:user-stories/us-08-01-29

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-08
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-08-01-29-01
      - arqix:requirements/req-08-01-29-02
      - arqix:requirements/req-08-01-29-03
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-08-01

properties:
  priority: medium
  edge-case: false

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-19
  updated: 2026-07-19
  lang: en
  translation-of:
  generated: false
---

## Configure the Checker Vocabularies

As a repository owner, I want the checker's vocabularies declared as configuration, so that a project ontology can extend them without forking the checker.

### Acceptance Criteria

- [ ] `[frontmatter].section-kinds` in `arqix.toml` replaces the effective vocabulary behind FM-007: a configured value passes, a value outside the configured list is a finding.
- [ ] `[frontmatter].allowed-external-types` replaces the effective vocabulary behind ONT-002 for non-arqix `rdf.type` entries.
- [ ] Without configuration both checks gate against the built-in vocabularies, unchanged.
- [ ] `[kinds.<family>.vocab]` maps a `properties` field name to its allowed values; a value outside the declared vocabulary is an FM-009 finding naming the field, the value, and the vocabulary.

### Notes

The vocabulary binding of a check is configuration while the check's substance stays code (ADR-0011, ADR-0017).
These two vocabularies are the smallest hardwired ones, so they move first; the layered-ontology derivation of vocabularies (ADR-0017) later replaces the hand-maintained lists as the one source.
