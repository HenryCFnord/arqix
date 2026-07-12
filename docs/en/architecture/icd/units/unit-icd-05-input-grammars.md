---
id: unit-icd-05
title: Input Grammars
slug: input-grammars
iri: arqix:units/unit-icd-05

rdf:
  type:
    - arqix:classes/unit

triples: []

properties:
  section-kind: icd-input-grammars

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-06
  updated: 2026-07-06
  lang: en
  translation-of:
  generated: false
---

## Input Grammars

The inputs an author — human or AI agent — writes for arqix to read.
These are the contracts agents most need, because agents are the primary authors of markers, directives, and triples.
The parsers live in `src/trace.rs` (markers), `src/linter.rs` (include directives, consumed by the assembler), and `src/parser.rs` (frontmatter triples); the oracle `scripts/arqix_trace.py` is the conformance reference for the marker and frontmatter grammars.

### Trace markers

Markers attach a source or test line to a requirement or a document.
They are line comments, matched only when the whole comment is the marker:

- In Rust (`.rs`): `// arqix:implements REQ-XX-YY-ZZ-NN` on a code item; `// arqix:verifies REQ-XX-YY-ZZ-NN` or `// arqix:no-requirement` on a test function (exactly one of the two — both is an error, TRC-005).
- `// arqix:plans REQ-XX-YY-ZZ-NN` — the planned claim without framework skip syntax (US-03-01-10): counts as planned in coverage, never as verified; satisfies the test-marker duty like `verifies`.
- In Markdown (`.md`): the same verbs inside an HTML comment, `<!-- arqix:implements REQ-… -->`.
- **`// arqix:documented-by <unit-iri>`** (decided in ADR-0009): attaches a code item to the unit that documents it, the inverse of a unit's `documents-artefact` triple.
  The trace engine learns to parse it in the follow-up slice; the grammar is fixed here so agents can author it now.

### Assembly directives

Composition directives are whole-line HTML comments in a document body:

- `<!-- arqix:include <path> -->` — splice the file at `<path>` (relative to the including file) in place, expanded depth-first by `assemble build`.
  An include cycle is a hard error (ASM-001).
- `<!-- arqix:chapter <n> -->` — a human-facing chapter marker that travels with the following include.

The path must be a single token; prose that merely mentions the directive is not matched.

### Reference markers

<!-- arqix:references-artefact arqix:adrs/adr-0009 -->
<!-- arqix:references-artefact arqix:adrs/adr-0006 -->
A document body can also carry paragraph-level references — the doc-side analogue of a frontmatter `references-artefact` triple (ADR-0009):

- `<!-- arqix:references-artefact <arqix-iri> -->` — a whole-line HTML comment with a single `arqix:` IRI.
  It emits a `references-artefact` edge from the enclosing document to the referenced document, located at the marker's own body line.
  Place the marker on the line(s) directly above the block it annotates, like a code comment above the code.
  The target must resolve to a known document (LNT-003).
  The grammar is designed to extend to any ontology property (`<!-- arqix:<property> <iri> -->`) as a later generalisation.

### Frontmatter triples

A document declares ontology edges in its YAML frontmatter `triples:` list:

```yaml
triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-08
```

`object` may be inline (`object: arqix:…`) or a `-` list item; both are matched with the oracle's whitespace tolerance (any run of spaces after the dash or colon).
Only `arqix:`-prefixed objects become graph edges; the predicate must be a defined `arqix:properties/…` (ONT-001) and each object must resolve to a scanned document (ONT-003).
