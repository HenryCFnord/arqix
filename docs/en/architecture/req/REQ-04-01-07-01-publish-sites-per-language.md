---
id: REQ-04-01-07-01
title: Publish Sites per Language
slug: publish-sites-per-language
iri: arqix:requirements/req-04-01-07-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-04-01-07
      - arqix:user-stories/us-05-01-13
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: `--lang en` builds from the EN root into the EN target; `--lang de` builds from the DE root into the DE target.

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

When `arqix publish site` is invoked with a language, arqix SHALL build the site from that language's configured root and write outputs to that language's artefact target.

### Notes

Derived from the acceptance criteria of US-04-01-07, US-05-01-13 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10).
Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
