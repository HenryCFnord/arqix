---
id: ADR-0005
title: Command Taxonomy
slug: command-taxonomy
iri: arqix:adrs/adr-0005

rdf:
  type:
    - arqix:classes/adr

triples:
  - predicate: arqix:properties/guides-design-of
    object:
  - predicate: arqix:properties/guides-implementation-of
    object:
      - arqix:requirements/req-01-01-08-03
      - arqix:requirements/req-03-01-02-01
      - arqix:requirements/req-03-01-04-01
      - arqix:requirements/req-04-01-10-01
  - predicate: arqix:properties/guides-verification-of
    object:

properties:
  decision-status: accepted

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-04
  updated: 2026-07-04
  lang: en
  translation-of:
  generated: false
---

## Command Taxonomy

### Context

The corpus wavers between `trace coverage` (requirements, workflows) and `report coverage` (three story acceptance criteria) for the coverage report.
Implementation needs one fixed command surface, and the command-ownership table in arc42 chapter 5 needs a normative naming rule behind it.

### Decision

- Commands follow the **noun–verb scheme** already dominant in the corpus: `doc new`, `doc list`, `unit new`, `config show`, `trace scan`, `lint run`, `policy check`, `publish site`, `mcp serve`.
- Each noun family maps to one owning component (see the command-ownership table in arc42 chapter 5); the table is the normative command map.
- **Every analysis exists exactly once.**
  Coverage is `trace coverage`; there is no separate `report coverage` command.
  Output formats are selected via the global `--format` option that every supported command carries (REQ-04-01-10-01) — serialisation is the Report & Export component acting as a shared library, not a second command.
- The `report` verb is reserved for export *products*: `report bundle` (evidence bundles, REQ-03-01-04-*) and, since US-05-01-15, `report knowledge` (OKF knowledge bundles, REQ-05-01-15-*).
- `verify` is the deliberate top-level exception to the noun–verb scheme: the one-command loop is the product promise for agents and CI (ADR-0003), and `arqix verify` is its ergonomic form.
- The three acceptance criteria that said `report coverage` (US-03-01-08, US-07-01-06, US-08-01-22) are reworded to `trace coverage`; they were determinism evidence lines and their requirement links are unaffected.

### Alternatives Considered

- **Two-stage pipeline (`trace coverage` computes, `report coverage` renders):** rejected — two commands over the same data invite drift, double the test surface, and contradict the global `--format` mechanism; unnecessary ceremony for a single binary.
- **Everything under `report`:** rejected — it would tear `scan`/`check` out of the trace family and require broad corpus rewording for no gain.

### Consequences

- The command surface is fixed before implementation; clap subcommand structure follows the noun families.
- The chapter 5 command-ownership table gains normative status for naming; new commands must join an existing noun family or justify a new one here.
- The chapter 11 naming debt item is resolved.
