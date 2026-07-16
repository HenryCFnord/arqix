---
title: "Roadmap"
description: "Planned work for arqix, in rough priority order"
date: 2026-07-16
status: active
---

# Roadmap

This document describes planned work for arqix.
It is intentionally rough and will evolve as the project matures.
Items are in approximate priority order, not a fixed schedule.

Progress through the implementation phases is measured by one number: the share of functional requirements referenced by `arqix:verifies` markers in the test suite (reported by `arqix trace markers`; currently 113/156 — the generated [scoreboard](../reports/units/scoreboard.md) is the always-current view).
"Done" for a story means its skeleton tests are un-ignored and green.

## Phase 0 — Foundation (done)

- [x] Rust CLI skeleton that compiles and runs
- [x] Repository layout with docs/ tree
- [x] README, roadmap, AI transparency document
- [x] Initial blog posts
- [x] Basic `.gitignore` and project metadata complete

## Phase 1 — Specification (done)

What the old phases 1–4 sketched as features is now fully specified and traceable:

- [x] 8 personas, 111 user stories (`docs/en/architecture/stories/`)
- [x] 165 requirements at the phase-1 cut — 126 functional, 17 quality, 22 constraints — under RFC 2119 subset + EARS patterns (`docs/en/architecture/req/`)
- [x] Ontology with requirement kinds and inverse properties (`docs/ontology/`)
- [x] Reference checkers: `check_requirements.py`, `check_frontmatter.py` (stdlib-only, strict, selftested)
- [x] arc42 documentation with Structurizr C4 model (`docs/en/architecture/`)
- [x] ADR-0001..0005 (agent instructions, C4 source, verification orchestrator, mechanical rewriter, command taxonomy)

## Phase 2 — Red skeleton and TDD harness (done)

- [x] clap command tree for the full surface per ADR-0005; stubs exit 70
- [x] 45 ignored command-contract tests mirroring the arc42 chapter 5 command-ownership table
- [x] `check_trace_markers.py` gate (TRC-001..005) and the normative TDD workflow in AGENTS.md
- [x] Glossary seeded: red skeleton, red phase

## Phase 3 — Trace family in Python, harness sharpening (done)

Goal: run arqix semantics on this repository daily, months before the Rust port — and turn the red skeleton into the conformance suite by construction.

- [x] Coverage report by requirement kind in the marker gate (the honest progress number is functional coverage, not total)
- [x] `ARQIX_BIN` override in the test helpers so the skeleton tests can run against any implementation
- [x] Python trace oracle `scripts/arqix_trace.py` behind the `scripts/arqix` dispatcher: `trace scan/check/coverage/matrix`, graph as JSON, coverage by kind, matrix as CSV — per the oracle policy in arc42 chapter 8; the `cli_trace` skeleton suite passes 7/7 against it
- [x] Thin Python `verify` sequencer (`scripts/arqix verify`): checkers + marker gate + informational coverage + cargo test
- Deliberately **not** in Python: parser/fmt/finalise (the lossless-CST machine is built once, in Rust), publish/render, mcp

## Phase 4 — Rust core, story by story (done)

Test-first per AGENTS.md: un-ignore, show red, implement to green.
Rough order, chosen so each story stands on the previous one (the Config Resolver moved ahead of the parser: it is the smallest self-contained story and the parser only becomes observable through the store commands).
Every slice landed as a `test(...)` red commit followed by a `feat(...)` green commit:

1. [x] Config Resolver (`config validate`, `config show`) — US-01-01-16
2. [x] Document Parser (frontmatter, classes/triples, body — the shared reading layer everything else needs), consumed directly by the Document Store & Catalog (`doc list`, `doc read`, `doc search`)
3. [x] Linter (`lint run`) — duplicate-id, include-target, and translation-source checks
4. [x] Formatter & Finaliser (`fmt`, `finalise` — the lossless line CST, single mutator per ADR-0004); byte-identical on the real corpus and idempotent
5. [x] Trace Engine (`trace scan/check/coverage/matrix`) — ported against the Python oracle; the un-ignored `cli_trace` skeleton tests pass 7/7 with the Rust binary, and `trace scan/coverage/matrix` are JSON-value-equal to the oracle on the real corpus
6. [x] Verification Orchestrator (`verify`, ADR-0003) — arqix now verifies its own corpus (format, lint, trace-scan pass; coverage gates honestly on the still-uncovered requirements)
7. [x] Template Engine (`doc init`, `doc new`, `unit new`)
8. [x] Assembler (`assemble build`) — include expansion into `pages/`, cycle detection (ASM-001), and a JSONL assembly log

Phase 5 is now the remaining command surface: `report bundle`, `policy check`, `mcp serve`, `publish site`, `render pdf`.

## Phase 5 — Publication and self-hosting

The remaining command surface, ordered so each slice makes the previous one more valuable.
[arqix.dev](https://arqix.dev) is live with a hand-written placeholder; slice 1's visible milestone is replacing it with the first arqix-published site.

- [x] Repository public, GitHub Pages on arqix.dev (placeholder in `site/`, deployed by `.github/workflows/pages.yml`)
- [x] CI workflow (`.github/workflows/ci.yml`): the daily gate on every PR — build, cargo tests, the dogfooded `arqix verify` (format, lint, trace, requirements, frontmatter, markers, report freshness), markdownlint, and the Rust lints; `just ci` mirrors it locally, and the snapshot refresh runs on main.

1. [x] **Publish site MVP** (`publish site`): stage the corpus and orchestrate the configured site toolchain (Zensical recommended) so arqix.dev becomes the first arqix-published site; the stitching mechanics (level-parameterised includes, heading-ownership policy, split on the assembled outline — ADR-0013, US-02-01-12) follow as the assembler slice.
2. [x] **Verification process**: configurable verify sub-steps with coverage report-only by default, the coverage ratchet against the committed snapshots, the machine-checked done claim per story lifecycle — and the gate dogfoods `arqix verify` for the corpus checks, while the Python checkers stay as reference cross-checks until the self-hosting slice.
3. [x] **Configuration over convention** (from the PR-#20 config audit): configured ID policy (ADR-0012), per-family frontmatter contracts as one source, template files instead of string literals, the snapshot strategy and ratchet baseline as configuration (C17).
4. [x] **Report & Export** (`report bundle`), **Policy Checker** (`policy check`), **MCP Server** (`mcp serve`) — plus the generated requirement/story catalogue pages that return the spec to the site in bundled form (one page per workflow group, an anchor per ID, coverage status from the graph) instead of 276 single pages.
5. [ ] **Render & languages** (`render pdf`, language-aware site, DE translations).
   The `render pdf` plumbing landed — containerised Pandoc/eisvogel via `just render-pdf`, the C4 SVGs embedded — and the three pre-release quality gaps closed: faithful vector embedding for the SVGs, one PDF per document, and a title page.
   Still open: the language-aware site and the DE translations.
6. [x] **Agent onboarding**: handbook chapter, an agent-instructions scaffold in `doc init`, and a packaged skill next to `mcp serve`.
7. [x] **Marker freshness** (V&V — arqix turned on its own architecture, validating it the way an IVVQ process validates a system rather than adding command surface): a `verifies`/`implements` marker still resolves after the requirement it targets is rewritten, so coverage can count a requirement as verified on a marker placed against an older version — close the gap with git arithmetic rather than code analysis, flagging a marker as possibly-stale when its target requirement was committed after the marker's own file, exposed as `trace freshness` and folded into `verify` as an informational sub-step so the loop measures current verification and not historical marker placement (ADR-0015).
8. [x] **Render architecture views from the model** (V&V — the `derived-from` promise, made structural): the embedded C4 Mermaid views carry a `<!-- derived from workspace.dsl -->` comment that nothing enforced, and the C4 audit found that drift by hand — rather than parse and check hand-authored diagrams, generate them from the single source: a containerised renderer (Kroki) turns `workspace.dsl` into committed SVGs, embedded in the arc42 chapters and kept honest by a regenerate-and-diff freshness gate in CI (the report-snapshot pattern), run identically via `just`; a generated view cannot drift from the model, so there is no diagram left to check (ADR-0016 — the earlier in-process checker is withdrawn, a hand-rolled C4-Mermaid parser proved a standing liability).
   Residual debt: the diagrams workflow runs on manual dispatch only, not yet on every PR (arc42 chapter 11).
9. [x] **Self-hosting closes the loop**: the Python reference checkers are demoted to cross-checks and retired per the oracle policy — sequenced after the two V&V slices above, since it changes where the checkers live but adds no verification capability.
   Landed 2026-07-15 (PR #88): the repository is pure Rust, `just verify` is the daily gate, and the mirrored oracle-selftest fixtures in the Rust test suite own the checker contracts.

Landed from the deferred list: the vector mark (`assets/arqix-mark.svg`, traced from the monochrome raster; `currentColor` fill, one file for both themes) and machine-readable licensing per [REUSE](https://reuse.software) (`REUSE.toml` + `LICENSES/`, no per-file SPDX headers; `reuse lint` compliant).

## Phase 6 — Knowledge repository, band 1, and release 0.2.0

The knowledge-repository intake (owner, 2026-07-15) reframes the corpus as a knowledge repository: explicit entities, evidence and claims, provenance, and queries over the graph.
The full plan lives in `docs/en/plans/knowledge-repository-2026-07-15/` (IDEA, PLANS, STATUS); this phase is its band 1 — the independently shippable slices pulled before the release, interleaved with the remaining refactor-program slices.

1. [x] **K0 — kind-declared directory** (gap G2): `doc new` creates in the declared `[kinds.<family>].dir`, one source with validation (US-08-01-25, PR #90).
2. [x] **K1 — declared kind templates** (gap G1): `[kinds.<family>].template` names the template file, and the placeholder vocabulary is validated — an unknown placeholder is a TPL-002 finding, never a silent literal (US-08-01-26, PR #92).
3. [x] **K2 — explicit target directory** (gap G3): `--dir` on `doc new` and the creation aliases, containment-guarded (US-08-01-27, PR #93).
4. [ ] **K3 — source-record kind** (gap G5): a `source` document kind for URL-plus-local-copy provenance (uri, access date, local-copy path, sha-256, licence, anchor).
5. [ ] **K4 — normative-statement export** (P5): the requirements checker's EARS/RFC-2119 sentence classification exported as data — a projection, no new parsing.
6. [ ] **R5 — checker-internal dedup** (refactor slice 5): hoist byte-identical checker helpers, consolidate date validation into `src/date.rs`.
7. [ ] **R6 — required-meta one source** (refactor slice 6): REQ-META-001 resolves the effective `[kinds.<family>].required-meta` contract instead of a hardcoded const — the program's one high-value correctness item.
8. [ ] **R7 — frontmatter-vocab config** (refactor slice 7): `[frontmatter] section-kinds` and `allowed-external-types` become configuration with byte-identical defaults; may slip past the release.
9. [ ] **Release v0.2.0**: RELEASING.md steps, CHANGELOG stamped, tag by the owner.

Landed alongside band 1: the story-workflow coupling lint (US-WF-001/US-PER-001 with the consolidation-persona exemption as corpus data, PR #94) — the numbering scheme is now machine-enforced.

## Beyond 0.2.0

Band 2 of the knowledge-repository plan is a decision gate, not code: the owner settles the deferred questions (ontology-ADR scope, lifecycle vocabularies, the splitter-contract close-out, evidence granularity) before band 3 starts.

Band 3 then builds the knowledge layers on those decisions, in dependency order:

- **Ontology as configuration**: the structural ADR (entity identity, external vocabularies, versioning properties), vocabulary derivation from the ontology, configurable IRI namespaces, and generalized mapping-target resolution (gap G6) — the coupling lint and its persona exemption become configurable here (owner note in the plan's STATUS.md).
- **Evidence and provenance**: the evidence/claims model and a W3C-PROV-oriented provenance layer as one design space, then the implementation slices (claim/supportedBy edges, confidence vocabulary, provenance-field validation — gap G4).
- **Lifecycle, crosswalks, queries**: configurable lifecycle vocabularies (gap G7), a crosswalk report unit over the mapping edges, and a declarative query surface over the entity/triple graph with MCP as the transport — deliberately last.
- **Splitter consolidation**: one splitter contract now that the oracle constraint is gone (per the band-2 decision on PR #87).

Candidate for 0.3.0 (owner idea 2026-07-12): an interactive graph explorer over the trace graph — the corpus as a navigable node-link view with kind/status filters, in the spirit of Obsidian's graph view; likely a self-contained page generated into the published site from the `trace scan` JSON.

## What this roadmap is not

This is not a promise.
It is a direction.
The project is early and solo.
Priorities will shift based on what is actually useful.
