---
id: US-08-01-35
title: Declare Vocabularies for Named Property Fields
slug: declare-vocabularies-for-named-property-fields
iri: arqix:user-stories/us-08-01-35

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-08
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-08-01-29-03
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-08-01

properties:
  priority: high
  edge-case: false

external-references: []

meta:
  lifecycle-status: retired
  owner: hcf
  created: 2026-07-19
  updated: 2026-07-19
  lang: en
  translation-of:
  generated: false
---

## Declare Vocabularies for Named Property Fields

As a repository owner, I want a kind to declare controlled vocabularies for named `properties` fields, so that domain states are validated by the gate instead of by project scripts.

### Acceptance Criteria

- [ ] `[kinds.<family>.vocab]` maps a `properties` field name to its allowed values; a value outside the declared vocabulary is a `lint frontmatter` finding (FM-009) naming the field, the value, and the vocabulary.
- [ ] Fields without a declared vocabulary and kinds without a `vocab` table stay unvalidated, exactly as before.
- [ ] The guarded lifecycle (`meta.lifecycle-status`, FM-008) is untouched — this is the orthogonal domain-state axis.

### Notes

The domain-state axis of the two-axes rule (ADR-0010 `decision-status` precedent, ADR-0017 decision 3): declared vocabularies validate intent-bearing domain fields, the guarded lifecycle stays core.
A second arqix-governed corpus validates exactly such vocabularies (`extraction-status`: extracted/proposed/decided) in project Python today (FR-C1 in the second intake of `docs/en/plans/knowledge-repository-2026-07-15/`).
