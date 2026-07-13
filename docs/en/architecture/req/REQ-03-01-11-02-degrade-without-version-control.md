---
id: REQ-03-01-11-02
title: Degrade Gracefully Without Version Control
slug: degrade-without-version-control
iri: arqix:requirements/req-03-01-11-02

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-03-01-11
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: Run in a directory with no reachable git history, arqix trace freshness reports zero stale markers and exits 0.

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

If `arqix trace freshness` cannot read version-control history for a marker or its target, then arqix SHALL NOT report the marker as stale.

### Notes

Derived from US-03-01-11.
A released tarball without a `.git` directory, or an untracked file, yields no history; freshness then degrades to reporting nothing stale rather than failing (ADR-0015).
