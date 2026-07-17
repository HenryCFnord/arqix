---
title: "Knowledge-repository intake 2: user feedback from the PSI terminology corpus"
date: 2026-07-17
status: draft
category: docs
branch: claude/personas-user-stories-jynl8i
---

# Second intake

Structured user feedback from a second arqix-governed corpus, captured 2026-07-17 and verified by its maintainers against the arqix 0.2.0 build of the same date.
PSI language is a bounded-context terminology pipeline: `references/sources/` (source records with exact provenance) feed `contexts/<ctx>/terms/` (upstream terms bound to sources via `definedIn`), which feed `canonical/` (decisions and cross-context mappings), with OWL/SKOS/SHACL/JSON-LD/PROV-O projections planned.
The corpus is agent-driven and gate-verified; today a project wrapper (`scripts/verify_repo.py`) sequences project-specific provenance validation and a catalog drift check before `arqix verify`.
The requests are ordered by how much they would let that project rely on arqix natively instead of on project Python, and by how many silent-error classes they close.
This intake is decision input for the band-3 process/ontology ADR: it is the second real-world use case, and it independently demands configurable kinds, vocabularies, relations, namespaces, and gates.

## Empirical status against the 0.2.0 build

- Placement is partially addressed: `doc new --dir` (K2) scaffolds straight into `contexts/<ctx>/terms/`, and kinds declare a dir (K0).
- A source-record kind exists (K3), but its default placement does not match the project layout, and no provenance validation surfaces for the project's own kind.
- `doc new` fills only `id`, `title`, and `slug`; template placeholders such as `{context}` and `{source_id}` stay literal in the created file.
- `arqix lint`/`verify` do not flag an unresolved `arqix:refs/...` target (verified by injecting a bogus reference); term-level `definedIn` resolution is checked by hand.
- The 0.2.0 command surface (`config`, `unit`, `finalise`, `assemble`, `report`, `publish`, `render`, `policy`, the creation aliases, `init`) is noted but not yet assessed for that project.

## Tier A — integrity (prevents silent errors)

### FR-A1 — reference and graph validation

The corpus's value is its provenance edges, yet nothing validates them: a term may point `definedIn` at a source record that does not exist and the gate stays green.
Proposed: resolve and validate every internal reference (`arqix:refs/...`, `definedIn`, mapping objects such as `owl:sameAs`, `owl:equivalentClass`, `broader`, `narrower`) during `verify` and fail on any dangling target; add a reverse index so the same pass reports orphans (terms with no `definedIn`, source records with no linked term).
Suggested surface: `arqix trace graph` plus a gating `verify` sub-step `graph`.
Ranked highest by the user: it closes the one error class that is currently invisible.

### FR-A2 — provenance-validated source-record kind

A PSI source record binds a stable id, publisher, version, source URL, access date, local-copy path, exact-byte SHA-256, licence, section anchors, and an `analysis` block naming the analyzed representation and its hash; arqix validates none of this, so the repo reimplements it in `scripts/source_records.py`.
Proposed: a first-class source-record kind that places into a configured root, validates the required provenance fields, checks that `local-copy` exists on disk with a matching SHA-256, and validates the `analysis` block against configured enums — while keeping source binaries outside the tracked corpus.
Suggested surface: a `verify` sub-step `provenance`.

### FR-A3 — deterministic catalog projection with a drift gate

The repo maintains `references/source-catalog.csv` by hand-rolled generation plus a drift check because arqix has no catalog projection.
Proposed: `arqix report catalog --check` projecting a deterministic, sorted, one-row-per-record table from source-record frontmatter, failing when the tracked artifact drifts; the column set configurable.

## Tier B — authoring ergonomics (removes per-term hand editing)

### FR-B1 — template placeholder substitution

The `upstream-term` template still emits literal `{context}` and `{source_id}`, so every extracted term is hand-edited after creation (about 240 extractions remain).
Proposed: substitution arguments with a controlled placeholder vocabulary (for example `--context tmforum`, `--defined-in <source-id>`, `--anchor "<section>"`) filled on creation.

### FR-B2 — context-aware ids and placement declared by kind

`doc new` defaults to `contexts/<kind>/` and a generic sequential id, while a term belongs at `contexts/<context>/terms/<context>-<slug>.md` with id `<context>-<slug>`.
Proposed: let a kind declare its target path pattern and id scheme so the common case is automatic and `--dir`/`--id` stay as manual overrides.

### FR-B3 — collision and uniqueness checks

Nothing prevents a duplicate slug within a context, a duplicate IRI, or a duplicate id at creation time; the agent greps manually before creating a term.
Proposed: uniqueness of id and IRI corpus-wide and of slug within a context, both at `doc new` time and as a `verify` sub-step.

## Tier C — modeling fidelity (bounded-context design)

### FR-C1 — project-defined controlled vocabularies

Status and role vocabularies (`extraction-status`: extracted/proposed/decided; review status; roadmap roles) are validated only by project Python.
Proposed: declare controlled vocabularies for named frontmatter fields and validate them natively in `verify`.

### FR-C2 — typed, resolved cross-context relations

Beyond `definedIn`, PSI needs typed relations for canonical mappings (`mapsTo`, `owl:sameAs`, `owl:equivalentClass`, `broader`, `narrower`) and for edition lineage (`supersedes`, `replaces`, `edition-covered`); today an edition relation is prose in a plan note, unvalidated and unqueryable.
Proposed: a configurable set of typed relation predicates whose objects are resolved and validated like FR-A1, so mappings and lineage become first-class, checkable edges.

### FR-C3 — bounded context as a first-class concept

`context: tmforum` in frontmatter and the `contexts/tmforum/` path can disagree with nothing to catch it.
Proposed: treat `contexts/<ctx>/` as a namespace — require a term's `context` to match its path, and allow per-context configuration (id prefix, allowed kinds).

## Tier D — semantic projection (future)

### FR-D1 — projection to OWL/SKOS/SHACL/JSON-LD/PROV-O

Proposed: a `render`/`report` target projecting the corpus triples into the semantic serializations, with a consistency or round-trip check, so ontologies become generation from the single source of truth instead of hand-authoring.

### FR-D2 — competency-question traceability for terms

Proposed: extend traceability so a canonical term links to competency questions and to valid/invalid example data, with coverage reporting — the existing ratchet extended to the semantic layer.

## Tier E — agent and automation ergonomics

### FR-E1 — stable JSON on every command

`--format json` exists but is not uniformly rich across `new`, `verify`, `lint`, and `trace`.
Proposed: stable, documented JSON schemas on all of them, with machine-readable findings (file, line, rule id, severity), so an orchestrating agent branches on results without parsing prose.

### FR-E2 — a composed verify that subsumes the wrapper

`scripts/verify_repo.py` exists only to sequence provenance validation, catalog drift, and `arqix verify`.
Proposed: once FR-A1/FR-A2/FR-A3 land as `verify` sub-steps, `verify` alone is the whole gate and the wrapper disappears.

### FR-E3 — optional semantic-line-break normalization in fmt

`arqix fmt` is neutral on prose wrapping (verified: a one-sentence-per-line body is left untouched) — it does not fight a chosen convention, but it cannot enforce one either.
Proposed: an optional, configurable `fmt` mode normalizing body prose to one sentence per line, aware of abbreviations, version numbers, code fences, tables, and lists; off by default.

## Triage against what arqix has

- FR-A1 is gap G6 (generalized reference-target resolution), now with a reproduction and a priority ranking from a real corpus; body markers already resolve (LNT-003), frontmatter triple objects do not.
- FR-A2 confirms the K3 direction and exposes its two limits: the SRC family is hardwired to this repository's `arqix:classes/source` and field set, and SRC-004 validates the digest's format, not the local copy's actual bytes.
- FR-A3 generalizes the report-unit machinery (nine snapshot units exist, all hardcoded) into configurable projections.
- FR-B1 extends K1: the placeholder vocabulary is validated (TPL-002), but only `id`/`title`/`slug` have values; user-supplied substitutions are new.
- FR-B2 extends K0 and the ADR-0012 id policy from flat dirs and sequential ids to path patterns and derived ids.
- FR-B3 is covered corpus-wide at lint time (duplicate-id checks) but not at creation time and not per-context.
- FR-C1 is R7 plus decision D2 — direct external demand for project vocabularies on named fields.
- FR-C2 and FR-C3 are the layered-ontology core: project-defined, checker-validated properties and namespaces.
- FR-E1 extends the diagnostics contract (REQ-00-00-00-03) to the creation and verify surfaces.
- FR-E2 is the process-catalog thesis stated by a user: the configured `verify` steps should be able to express the whole project gate.
- FR-D1/FR-D2 sit behind the evidence layer (B-slices) and the query surface (D7).

## What this changes for band 2

The D1 briefing's strongest contra against deciding entity/relation questions now was "one real use case, no second datapoint".
This intake is the second datapoint, and it points the same way as the owner's reframing: processes as configuration over hardwired gates, a reserved core vocabulary, project vocabularies and relations that the checker resolves and validates.
