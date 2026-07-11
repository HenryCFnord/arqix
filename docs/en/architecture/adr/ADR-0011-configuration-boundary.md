---
id: ADR-0011
title: Configuration Boundary
slug: configuration-boundary
iri: arqix:adrs/adr-0011

rdf:
  type:
    - arqix:classes/adr

triples:
  - predicate: arqix:properties/guides-design-of
    object:
      - arqix:requirements/req-01-01-19-02
  - predicate: arqix:properties/guides-implementation-of
    object:
      - arqix:requirements/req-01-01-18-01
      - arqix:requirements/req-01-01-20-01

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

## Configuration Boundary

### Context

The PR-#20 review produced six comments with one theme: conventions hard-wired into `src/` and `scripts/` that a repository owner could legitimately want to change.
The systematic sweep (config audit, plan package 2026-07-09) inventoried sixteen configuration candidates (C1–C16) and five deliberate keeps (K3–K7); several candidates are maintained in more than one place today, and one of those copies had already diverged (C6, fixed ahead of this strand).
Without a stated boundary, every future "should this be configurable?" question is re-litigated ad hoc — the sixteen rows, and every row after them, need one rule they can be judged against.

### Decision

A value becomes **configuration** when at least one of two conditions holds: it is maintained in more than one place today (double bookkeeping is a live drift risk, not a style question), or a repository owner can legitimately want it different — layout, naming, ID shapes, templates, language.

A value stays a **convention** when it is the tool's identity (the `arqix:` marker prefix and verb vocabulary), a stability contract (the 0/1/2/70 exit codes), the substance of a check (the EARS patterns and RFC 2119 keyword subset — configuring them would hollow the check out), or a safety rule (filename and containment discipline).

Two rules govern every configured value:

<!-- arqix:references-artefact arqix:requirements/req-01-01-19-02 -->
1. **One source.** A configured value that participates in oracle conformance feeds the Rust engine and the Python reference tools from the same source.
   This is an acceptance criterion of every configuration story, not an afterthought: two loaders of one file are conformance-testable, two copies of one table are not.
2. **Defaults preserve the present.** The built-in defaults reproduce the current conventions exactly, so shipping a configuration surface never changes an existing corpus (`REQ-00-00-00-06`: the effective configuration is the baseline).

The strand is cut as three stories: US-01-01-18 (ID policy, including the derivation model — see ADR-0012), US-01-01-19 (frontmatter contracts: key order and required meta as one source), and US-01-01-20 (template files instead of string literals).
Audit rows not covered by these stories ride along where a story touches their code (C9, C11–C14) or already landed with strand 1 (C8, verify sub-steps).

### Alternatives Considered

- **Make everything configurable:** rejected — the keeps are keeps for cause; configurable exit codes break the stable contract, configurable EARS patterns turn a normative check into an opinion, and a configurable marker prefix orphans the ontology vocabulary.
- **Keep everything a convention:** rejected — the C6 divergence (required meta keys differing between two checkers) demonstrated that double bookkeeping does not stay consistent by discipline alone.
- **Per-tool configuration files:** rejected — a Rust-side and a Python-side config split the one source this decision exists to establish, guaranteeing exactly the drift it must prevent.

### Consequences

- The `arqix.toml` schema grows `[kinds]`, `[frontmatter]`, and `[templates]` sections; the configuration contract (`config validate`) covers them.
- The Python reference checkers gain configuration loading (stdlib TOML) so both sides read the one source.
- The conformance suite pins Rust/Python parity per configured value, including at least one non-default configuration.
- The config-audit table remains the backlog for the ride-along rows; this ADR is the boundary they are judged against.
