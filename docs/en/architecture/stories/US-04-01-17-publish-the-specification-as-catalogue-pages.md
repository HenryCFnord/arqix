---
id: US-04-01-17
title: Publish the Specification as Catalogue Pages
slug: publish-the-specification-as-catalogue-pages
iri: arqix:user-stories/us-04-01-17

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-10
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-04-01-17-01
      - arqix:requirements/req-04-01-17-02
      - arqix:requirements/req-04-01-17-03
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-04-01

properties:
  priority: medium
  edge-case: false

external-references: []

meta:
  lifecycle-status: done
  owner: hcf
  created: 2026-07-12
  updated: 2026-07-12
  lang: en
  translation-of:
  generated: false
---

## Publish the Specification as Catalogue Pages

As an assessor, I want the published site to carry the specification as bundled catalogue pages, so that I can read and link every story and requirement with its live coverage status without wading through hundreds of single source pages.

### Acceptance Criteria

- [x] With the catalogue enabled in the publish policy, `publish site` stages generated catalogue pages — one page per workflow group — while the requirement and story source files stay off the site.
- [x] Every story and requirement in the catalogue carries an anchor for its ID, so deep links like `.../wf-01-01/#REQ-01-01-07-02` resolve.
- [x] Every requirement entry carries its coverage status from the trace graph (verified, planned, or uncovered).
- [x] Identical corpus state produces byte-identical catalogue pages.

### Notes

This is the catalogue slice of roadmap phase 5 item 4: the spec sources were taken off the site by the publish exclude scope (owner decision 2026-07-11) and return in bundled form — one page per workflow group, not 276 single pages.
The pages are generated at staging time and never committed: the publisher already owns the artefact-ready transformation, and staging-time generation keeps the catalogue exactly as fresh as the site build that carries it.
