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

The config audit recommended keeping the ID shapes a convention; the owner overruled (2026-07-10): the ID shape is the main point of configuration — a repository adopting arqix brings its own numbering.
The complication is that arqix's ID shapes are load-bearing, not cosmetic: the trace graph derives a requirement's owning story from its ID (the `story_of` slice), the canonical-owner rule ties the first `derived-from` to that story, cross-cutting requirements are recognised by their `00-00-00` domain, and per-story sequencing is validated positionally.
A plain configured regex could validate a foreign shape but could not feed these derivations — the graph would still hardcode offsets.

### Decision

<!-- arqix:references-artefact arqix:requirements/req-01-01-18-02 -->
The ID policy is a set of **per-family patterns with named groups**, and the derivation model consumes groups by name — the pattern is one artefact that both validates the shape and declares its semantics.

```toml
[kinds.requirement]
id-pattern = '^REQ-(?P<story>\d{2}-\d{2}-\d{2})-(?P<seq>\d{2})$'

[kinds.story]
id-pattern = '^US-(?P<story>\d{2}-\d{2}-\d{2})$'

[kinds]
cross-cutting-domain = "00-00-00"
```

- The `story` group carries the owner slice: a requirement's owning story is the story whose `story` group matches, and the canonical-owner rule reads it from there.
- The `seq` group carries per-story sequencing; the sequencing check validates it as a number, not as a character offset.
- The `cross-cutting-domain` value marks ownerless requirements; matching the `story` group against it replaces today's hardcoded `00-00-00` comparisons.
- Engine, oracle, and both reference checkers read the same configured policy (the one-source rule of ADR-0011); the built-in defaults are exactly the patterns above, so an unconfigured corpus behaves as before.

### Alternatives Considered

- **A plain regex without named groups:** rejected — it validates but cannot derive; every consumer of the owner slice would keep its own offset arithmetic, and the configuration would be cosmetic.
- **A template mini-language (`REQ-{story}-{seq}`):** considered — friendlier to write, but it needs its own parser, its own escaping rules, and a compilation step into a regex anyway; named groups are standard in both regex engines arqix already uses and collapse validation and derivation into one artefact.
- **Keeping the shapes hardcoded:** rejected by the owner decision — five copies across Rust and Python today, drift-prone and fork-hostile.

### Consequences

- The configuration contract gains `id-pattern` per family and `cross-cutting-domain`; `config validate` rejects patterns missing the groups the derivation model needs.
- The conformance suite gains a case with a non-default pattern, pinning that engine and oracle derive identical graphs from the same policy.
- US-01-01-18 implements this model; the current shapes become the default configuration and the corpus stays unchanged.
- Renaming an ID family in an existing corpus remains a migration, not a config flip: the policy governs validation and derivation, it does not rewrite documents.
