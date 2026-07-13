---
id: REQ-01-01-11-06
title: Validate Requirement Authoring Rules
slug: validate-requirement-authoring-rules
iri: arqix:requirements/req-01-01-11-06

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-11
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: A requirement whose id, iri, slug, kind, required metadata, or derivation links break the corpus scheme, or whose normative sentence departs from the RFC 2119 subset or the EARS patterns, makes the check exit non-zero and names the offending document and rule.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-13
  updated: 2026-07-13
  lang: en
  translation-of:
  generated: false
---

## Requirement

When `arqix lint requirements` runs, arqix SHALL report every requirement document whose identity, kind, required metadata, or derivation links violate the corpus scheme and every normative sentence that departs from the RFC 2119 subset and EARS patterns of the requirements style guide.

### Notes

Realises the documentation consistency check that REQ-01-01-11-05 records as an extension path, porting the reference checker `scripts/check_requirements.py` (REQ-ID, REQ-KIND, REQ-META, REQ-LNK, US-ID, EARS rule families) into the binary as a self-hosting slice of the oracle policy (arc42 chapter 8, roadmap phase 5 item 9).
Cross-cutting behaviour is linked via the story's `has-requirement`, not restated here.
