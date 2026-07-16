---
id: class-source
label: source
iri: arqix:classes/source

rdf:
  type:
    - rdfs:Class

rdfs:
  sub-class-of:
    - arqix:classes/knowledge-artefact

triples: []

properties: {}

external-references: []

owl: {}

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-16
  updated: 2026-07-16
  lang: en
  generated: false
---

## Source

An external source captured as a first-class record: the URL that was read plus a local copy, so every claim that cites the source stays verifiable against what was actually read.

The class carries a provenance contract, enforced by `arqix lint frontmatter` (the SRC rule family) on every document typed `arqix:classes/source`:

- `properties.uri` — where the source lives.
- `properties.accessed` — the calendar date the copy was taken.
- `properties.local-copy` and `properties.sha256` — the repository-relative path of the verbatim copy and its digest (sixty-four lowercase hexadecimal characters), given together when a copy is held; the copy stays outside the documentation roots, so it never enters the tracked corpus.
- `properties.licence` and `properties.anchor` — optional: the licence the copy is held under, and a fragment or section anchor inside the source.

Completeness is a finalisation contract: a record still in `draft` may hold an incomplete skeleton, but once it leaves draft the uri and the access date must be present.
Malformed values are findings in every lifecycle state.

A local copy is held only when its licence permits redistribution (this repository is public).
Otherwise there is no local copy: the record carries the uri and the access date, and neither a path nor a digest pretends a copy exists — `local-copy` and `sha256` travel as a pair.
