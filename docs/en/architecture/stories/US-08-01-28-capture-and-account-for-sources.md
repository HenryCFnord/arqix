---
id: US-08-01-28
title: Capture and Account for Sources
slug: capture-and-account-for-sources
iri: arqix:user-stories/us-08-01-28

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-08
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-08-01-28-01
      - arqix:requirements/req-08-01-28-02
      - arqix:requirements/req-08-01-28-03
      - arqix:requirements/req-08-01-28-04
      - arqix:requirements/req-08-01-28-05
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-08-01

properties:
  priority: medium
  edge-case: false

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-16
  updated: 2026-07-19
  lang: en
  translation-of:
  generated: false
---

## Capture and Account for Sources

As a knowledge engineer, I want external sources captured as verified first-class records and accounted for in one catalog, so that every claim the corpus derives from a source stays checkable against what was actually read.

### Acceptance Criteria

- [ ] The ontology defines `arqix:classes/source` as a knowledge artefact, and `lint frontmatter` enforces its provenance contract (the SRC rule family) on every document of that class.
- [ ] A source record carries `uri` and `accessed` in its properties (enforced once the record leaves draft) and, when a copy is held, `local-copy` plus `sha256` as a pair (`licence` and `anchor` optional).
- [ ] Malformed values — a non-calendar `accessed`, a digest that is not sixty-four lowercase hex characters, a `local-copy` that escapes the repository or lies inside a documentation root — are findings in every lifecycle state.
- [ ] `arqix doc new source` creates a conforming draft record from the declared `[kinds.source]` contract (directory, template, id-pattern).
- [ ] A source record whose local copy is missing, or whose bytes do not hash to the recorded `sha256`, is an SRC-006 finding naming the path.
- [ ] `report snapshot` renders the source catalog: one row per source record, sorted by id, provenance columns from the frontmatter, under the snapshot drift gate.

### Notes

Slice K3 of the knowledge-repository program (`docs/en/plans/knowledge-repository-2026-07-15/`, gap G5); owner decision 2026-07-16: the source kind is a full ontology member with its own checker family, not a configuration-only kind.
The class contract keys on `rdf.type`, so any repository that types a document `arqix:classes/source` gets the provenance checks — the `[kinds.source]` entry only adds the creation surface (K0/K1 machinery) and the directory-based format contract.
Local copies stay outside the documentation roots so verbatim third-party artefacts never enter the tracked corpus; the record points at them, and the digest pins what was read.
The evidence model of band 3 (claims, supportedBy, confidence) builds on this vocabulary.
