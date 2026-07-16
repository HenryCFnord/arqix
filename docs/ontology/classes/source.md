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
- `properties.local-copy` — the repository-relative path of the verbatim copy; it stays outside the documentation roots, so copies never enter the tracked corpus.
- `properties.sha256` — the digest of the local copy, sixty-four lowercase hexadecimal characters.
- `properties.licence` and `properties.anchor` — optional: the licence the copy is held under, and a fragment or section anchor inside the source.

Completeness is a finalisation contract: a record still in `draft` may hold an incomplete skeleton, but once it leaves draft the four required fields must be present.
Malformed values are findings in every lifecycle state.

A local copy is committed only when its licence permits redistribution (this repository is public).
Otherwise the copy stays untracked at the recorded path, and the record still pins the uri, the access date, and the digest — whoever needs the text takes their own copy and verifies it against the digest.
The checker deliberately never requires the copy to exist, so both cases pass the same contract.
