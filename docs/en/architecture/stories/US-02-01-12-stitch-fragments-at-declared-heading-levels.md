---
id: US-02-01-12
title: Stitch Fragments at Declared Heading Levels
slug: stitch-fragments-at-declared-heading-levels
iri: arqix:user-stories/us-02-01-12

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-09
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-02-01-12-01
      - arqix:requirements/req-02-01-12-02
      - arqix:requirements/req-02-01-12-03
      - arqix:requirements/req-02-01-12-04
      - arqix:requirements/req-02-01-12-05
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-02-01

properties:
  priority: medium
  edge-case: false

external-references: []

meta:
  lifecycle-status: done
  owner: hcf
  created: 2026-07-11
  updated: 2026-07-12
  lang: en
  translation-of:
  generated: false
---

## Stitch Fragments at Declared Heading Levels

As a developer, I want an include directive to place the fragment's headings at a declared level, absolute or relative, so that units of any granularity compose into a coherent outline wherever they are included.

### Acceptance Criteria

- [x] `<!-- arqix:include <path> level=N -->` places the fragment's first heading at level N; every heading in the fragment shifts by the same delta.
- [x] `level=+N` resolves against the heading level in effect at the include position; moving the include re-levels the fragment without editing it.
- [x] Without a level argument, the configured `heading-ownership` default applies (`child`: behaves as `level=+1`; `parent`: fragments are headingless, the argument governs internal headings only).
- [x] A shift beyond h6 fails assembly with a diagnostic naming the fragment and the heading (ASM-005); no partial page is written.

### Notes

The model — declared levels, ownership as corpus policy, split on the assembled outline, `arqix:chapter` retired — is ADR-0013.
The assembly log records the resolved level per step, so a review can see the outline decision that was made.
The publisher's `split` stitching mode and Pandoc's `--toc` both consume the outline this story makes reliable.
