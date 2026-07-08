---
id: ADR-0008
title: Question-Driven Report Units
slug: question-driven-report-units
iri: arqix:adrs/adr-0008

rdf:
  type:
    - arqix:classes/adr

triples:
  - predicate: arqix:properties/guides-design-of
    object:
  - predicate: arqix:properties/guides-implementation-of
    object:
      - arqix:requirements/req-04-01-12-01
      - arqix:requirements/req-04-01-12-03
      - arqix:requirements/req-01-01-08-03
  - predicate: arqix:properties/guides-verification-of
    object:

properties:
  decision-status: accepted

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-05
  updated: 2026-07-05
  lang: en
  translation-of:
  generated: false
---

## Question-Driven Report Units

### Context

The first human-facing trace snapshot was a 142-row coverage table — a dump of the model, and unreadable.
The review conclusion: the design question was asked in the wrong order.
An artefact's presentation cannot be derived from the data structure; it must be derived from the *question* the artefact answers, and the questions are many: story progress, test↔requirement evidence, verified-implementation quota, code↔requirement, story↔test, workflow↔test, ADR↔requirement, docs↔code, plus classic metrics like lines of code and code coverage.

### Decision

- **One unit, one question.**
  Every human-facing report artefact answers exactly one named question.
  The presentation (table, scoreboard, list, prose) follows the question, not the data structure.
- **Units are projections.**
  Each unit is a deterministic projection of the trace graph (ADR-0006 layer 1) — or, for metrics like LoC and code coverage, of an external data source joined against the graph.
- **A report is an assembly of units**, mirroring the arc42 unit/page model: pages assemble chapters, reports assemble question units.
- **Raw model dumps are not human artefacts.**
  The full graph and the full coverage table remain machine-layer outputs; a human artefact that merely reprints them is a design error, not a report.
- **The question catalog is a living document** (`docs/en/reports/QUESTIONS.md`).
  Each question graduates into a user story for the report family before it becomes Rust command surface — the catalog is the story backlog for reporting.

### Alternatives Considered

- **One big report:** rejected — it converges on the dump that prompted this ADR; every reader scrolls past everyone else's answers.
- **One output per data structure (the previous coverage.md):** rejected empirically — a table per model entity answers no one's question.
- **Dashboard tooling first:** rejected — tooling choices before the question catalog repeats the same mistake with more colours.

### Consequences

- `docs/en/reports/units/` holds one generated file per answerable question; the generator (`scripts/arqix_report.py`, later the Rust report family) is the only writer.
- New questions enter the catalog first, get a unit design, and only then command surface — via story and requirements, like every other feature.
- Audit CSVs (matrices, bundles) are unaffected: they are ADR-0006 layer 3 machine artefacts, not human units.
