---
id: US-08-01-41
title: Export Claims and Report Evidence Coverage
slug: export-claims-and-report-evidence-coverage
iri: arqix:user-stories/us-08-01-41

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-08
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-08-01-41-01
      - arqix:requirements/req-08-01-41-02
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

## Export Claims and Report Evidence Coverage

As a knowledge engineer, I want the corpus's claims exported as data and their coverage reported as numbers, so that evidence is queryable and its growth observable without ever becoming a gate.

### Acceptance Criteria

- [ ] `arqix report claims` prints one CSV row per claim marker — file, supported-by target, confidence, anchor — deterministically ordered; the committed export sits under the snapshot freshness gate.
- [ ] The evidence-coverage unit reports the totals: claims, documents carrying claims, distinct sources cited — numbers, never a gate.
- [ ] A corpus without claim markers yields the header-only export and zero counts, not an error.

### Notes

The data side of ADR-0018: the anchors live in the text, the edges in the frontmatter, and this projection makes both countable — the export follows the normative-statements pattern, the unit follows the question catalog.
