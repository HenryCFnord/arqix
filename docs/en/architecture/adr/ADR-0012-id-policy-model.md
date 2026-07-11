---
id: ADR-0012
title: ID Policy Model
slug: id-policy-model
iri: arqix:adrs/adr-0012

rdf:
  type:
    - arqix:classes/adr

triples:
  - predicate: arqix:properties/guides-design-of
    object:
      - arqix:requirements/req-01-01-18-02
  - predicate: arqix:properties/guides-implementation-of
    object:
      - arqix:requirements/req-01-01-18-01
      - arqix:requirements/req-01-01-18-03
      - arqix:requirements/req-01-01-18-04

properties:
  decision-status: accepted

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-11
  updated: 2026-07-11
  lang: en
  translation-of:
  generated: false
---

## ID Policy Model

### Context

The document ID shapes are hard-wired in five places across the Rust engine and the Python reference tools, and a repository adopting arqix brings its own numbering — the shapes belong in configuration (audit rows C3/C4/C15/C16, boundary per ADR-0011).
The complication: the shapes are load-bearing today — the trace graph derives a requirement's owning story from an ID slice, cross-cutting requirements are recognised by their `00-00-00` domain, and per-story sequencing is validated positionally.
A configurable pattern therefore faces a dilemma: if it must carry these semantics, every adopting repository is forced into semantic IDs; if it is a plain shape check, the derivations the tooling depends on lose their source.
The dilemma is false: the corpus already declares the relation the ID encodes — every requirement names its owning story as its first `derived-from` triple — so the ID-slice derivation is a redundant encoding of a declared fact, and redundant encodings drift.

### Decision

<!-- arqix:references-artefact arqix:requirements/req-01-01-18-02 -->
**Declared triples are the source of truth for relations; the ID is an opaque label.**
The trace graph resolves a requirement's owning story from its first `derived-from` triple — declared, not derived.
This is the same rule the lifecycle decision rests on (ADR-0010: declared carries intent) applied to relations, and it is what "traceability as a graph" means: the relation lives in the graph, not hidden in a naming scheme.

The ID policy then governs only what is genuinely the ID's job:

```toml
[kinds.requirement]
id-pattern = '^REQ-(?P<story>\d{2}-\d{2}-\d{2})-(?P<seq>\d{2})$'

[kinds.adr]
id-pattern = '^ADR-(?P<seq>\d{4})$'
```

- **Shape and uniqueness:** a document ID must match its family's configured pattern and be unique in the corpus.
- **Generation:** `doc new` mints the next ID from the pattern (the `seq` group tells it what to count); any other groups are irrelevant to generation.
- **Consistency checks, where groups exist:** named groups are optional and activate checks, not derivation.
  Where a pattern declares a `story` group, the checker reports an ID whose encoded slice contradicts the declared owner triple; where it declares `seq`, per-story sequencing is validated.
  arqix's own corpus keeps its strict discipline through exactly these checks in the default policy.
- A pattern with no semantic groups is fully supported: a repository using `SRS-1234` gets shape validation, uniqueness, and generation, and its trace graph works entirely from the declared triples.

Cross-cutting requirements follow the same inversion: instead of being recognised by an ID domain, they are declared — the ontology gains an explicit cross-cutting marker, and the `00-00-00` domain remains merely the default policy's naming convention for them.
Engine, oracle, and both reference checkers read the same configured policy (the one-source rule of ADR-0011); the built-in defaults reproduce the current shapes and checks, so an unconfigured corpus behaves as before.

### Alternatives Considered

- **Named groups as the derivation source (the first draft):** rejected as too restrictive — it forces every adopting repository into semantic IDs and duplicates a relation the corpus already declares; redundant encodings drift, declared ones are checked.
- **A plain regex with no group semantics at all:** rejected — generation needs to know what to count (`seq`), and dropping the optional consistency checks would cost arqix's own corpus its sequencing and owner-slice discipline for nothing.
- **A template mini-language (`REQ-{story}-{seq}`):** considered — friendlier to write, but it needs its own parser and escaping rules and compiles into a regex anyway; named groups are standard in both regex engines arqix already uses.
- **Keeping the shapes hardcoded:** rejected — five copies across Rust and Python today, drift-prone and fork-hostile; making the shapes configurable is the point of this decision.

### Consequences

- The configuration contract gains `id-pattern` per family; `config validate` requires a `seq` group where the family is created via `doc new` and accepts patterns without any semantic groups.
- The trace engine and the oracle move `story_of` from ID slicing to the declared `derived-from` triples — part of the US-01-01-18 implementation, conformance-checked on both sides.
- The ontology gains a declared cross-cutting marker; the checkers' `00-00-00` recognition becomes a default-policy consistency check instead of the definition.
- The conformance suite gains a case with a non-default, group-free pattern, pinning that both sides derive identical graphs from triples alone.
- The current shapes and checks become the default configuration; the corpus stays unchanged.
- The default policy doubles as the recommended naming scheme for new corpora, to be carried into the handbook (agent-onboarding strand): semantic IDs keep the ownership relations readable in place, and the consistency checks that hold them against the declared triples already exist.
