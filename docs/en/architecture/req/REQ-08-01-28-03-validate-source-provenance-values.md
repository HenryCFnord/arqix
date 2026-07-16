---
id: REQ-08-01-28-03
title: Validate Source Provenance Values
slug: validate-source-provenance-values
iri: arqix:requirements/req-08-01-28-03

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-08-01-28
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: A source document with a non-calendar accessed date, a sha256 that is not sixty-four lowercase hexadecimal characters, or a local-copy path that is absolute, contains a parent segment, or lies inside a documentation root makes the check exit non-zero, in any lifecycle state.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-16
  updated: 2026-07-16
  lang: en
  translation-of:
  generated: false
---

## Requirement

When `arqix lint frontmatter` runs, arqix SHALL report every source document whose accessed value is not a calendar date, whose sha256 value is not sixty-four lowercase hexadecimal characters, or whose local-copy path escapes the repository or lies inside a documentation root.

### Notes

Rules SRC-003 (accessed), SRC-004 (sha256), and SRC-005 (local-copy): a present-but-malformed value is a finding in every lifecycle state — only absence is excused in draft (REQ-08-01-28-02).
The local-copy containment mirrors REQ-00-00-00-13 (repository-relative, no parent segments), and the documentation-root exclusion keeps verbatim third-party copies out of the tracked corpus.
Existence of the local copy is deliberately not checked: copies may be fetched on demand, and the digest pins their content.
Derived from US-08-01-28 (knowledge-repository slice K3, gap G5).
