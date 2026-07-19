---
id: US-08-01-34
title: Verify the Local Copy Against Its Digest
slug: verify-the-local-copy-against-its-digest
iri: arqix:user-stories/us-08-01-34

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-08
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-08-01-28-04
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

## Verify the Local Copy Against Its Digest

As a knowledge engineer, I want the recorded digest checked against the local copy's actual bytes, so that a source record pins what was really read instead of merely carrying a well-formed hash.

### Acceptance Criteria

- [ ] A source record whose `local-copy` file is missing, or whose bytes do not hash to the recorded `sha256`, is a `lint frontmatter` finding (SRC-006) naming the path.
- [ ] A record whose copy matches its digest passes; records without the pair are untouched.
- [ ] The digest check runs only when the path shape (SRC-005) and digest format (SRC-004) are already clean — one cause, one finding.

### Notes

SRC-004 validates the digest's form; this closes the gap between a well-formed hash and the file it claims to pin.
A second arqix-governed corpus reimplements exactly this check in project Python (FR-A2 in the second intake of `docs/en/plans/knowledge-repository-2026-07-15/`).
