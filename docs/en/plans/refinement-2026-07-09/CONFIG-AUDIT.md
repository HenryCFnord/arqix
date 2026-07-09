---
title: "Config audit: convention or configuration"
date: 2026-07-09
status: draft
category: docs
branch: docs/refinement-2026-07
---

# Config audit

Systematic sweep over hard-wired conventions in `src/` and `scripts/`, triggered by the six PR-#20 review comments.
Decision D2 of [PLANS.md](PLANS.md): confirm or overrule the recommendation per row.
"Copies" lists every place the convention is maintained today; the coupling column names what must read one source if it becomes configuration.

## Recommended: configuration

Sorted by leverage (double bookkeeping + user relevance first).

| # | Convention | Copies today | Proposed arqix.toml home | Coupling |
| --- | --- | --- | --- | --- |
| C1 | Canonical frontmatter key order per family | `src/rewriter.rs:17-61`, `check_frontmatter.py:72-121` (FAMILIES) | `[kinds.<family>] key-order` | fmt (Rust) and checker (Py) must read the same source or fmt formats what the checker flags |
| C2 | Family → directory mapping | `src/rewriter.rs:65-75`, `check_frontmatter.py` FAMILIES, `check_requirements.py:58-59`, `check_trace_markers.py:38-41` | `[kinds.<family>] dir` | four readers; couples to `roots` |
| C3 | Per-kind ID scheme (prefix, zero-pad width) | `src/templates.rs:16-22` (scheme), `check_frontmatter.py:124-134` (ARCH_NS), `:149-152` (ONT_*) | `[kinds.<kind>] id-prefix / id-width` | `doc new` generates what the checker validates |
| C4 | IRI namespaces per kind | `templates.rs`, `parser.rs:166,194`, `check_frontmatter.py` ARCH_NS/ONT_NS, `check_requirements.py:360,417,467` | `[kinds.<kind>] iri-namespace` | generator and validators need one namespace table |
| C5 | Document templates as string literals | `src/templates.rs:40-57,137-143` | `[templates.<kind>] path` (files; `doc init` scaffolds defaults) | template must satisfy C1 key order and C6 required meta |
| C6 | Required meta keys — **already divergent today** | `check_frontmatter.py:69` (6 keys incl. `generated`), `check_requirements.py:106` (5 keys, without), `templates.rs:52-53` | `[frontmatter] required-meta` | two checkers + template; the divergence is a live bug risk |
| C7 | Language default `en` (+ `docs/en/…` layout) | `templates.rs:52`, `check_frontmatter.py:69,299-301` (FMT-006) + FAMILIES paths, `check_requirements.py:58-59` | `[i18n] default-lang` (section already reserved) | FMT-006 expectation, template default, and path layout from one source |
| C8 | Verify-loop sub-steps | `src/verifier.rs:15-20` (STEPS) | `[policies.verify] steps` | none — and code comments already say "configured sub-steps" (strand-1 story US-04-01-14) |
| C9 | `section-kind` vocabulary | `check_frontmatter.py:139-148` (FM-007) | `[frontmatter] section-kinds` | ADR-0009 says new families "register their kinds here" — registration belongs in config, not code |
| C10 | Requirement-kind classes + short names | `parser.rs:8-12`, `trace.rs:16-20,118-130`, `arqix_trace.py:49-51,63-67`, `check_requirements.py:65-69`, `check_trace_markers.py:67-74` | `[kinds] requirement-classes` | five copies, oracle-conformance-bound: Rust and Python must load the same source |
| C11 | Assembler output root `pages/` + `assembly.jsonl` | `src/assembler.rs:21-22` | `[policies.assemble] out-root / log-name` | already annotated "configurable lands with render/publish stories" (REQ-04-01-01-03) |
| C12 | Package scaffold directories (units, pages, artefacts, logs, .arqix) | `src/templates.rs:168,137-143` | `[templates] package-dirs` | `pages` here and assembler out-root are two literals for one concept |
| C13 | EARS subject forms (`arqix`, `The arqix CLI`, backticked command) | `check_requirements.py:291-296` | `[policies.ears] subject-forms` (low priority) | a fork with another tool name is instantly wrong |
| C14 | Report output paths | `arqix_report.py:230-239,257,388` | `[policies.reports]` (paths only; Q-IDs stay ADR-0008-bound) | `--check` defaults must match generation defaults |

## Recommended: keep as convention

| # | Convention | Where | Why keep |
| --- | --- | --- | --- |
| K1 | Requirement-ID shape `REQ-XX-YY-ZZ-NN` | `parser.rs:84-94`, `trace.rs`, `arqix_trace.py:40`, both doc checkers | conformance-bound and load-bearing for the `story_of` derivation; making it configurable is a model change, not a config key — revisit only with a concrete multi-project need (review comment noted; see D2a below) |
| K2 | Story-ID shape `US-XX-YY-ZZ` + `id[4:12]` owner slice + `00-00-00` cross-cutting domain | `trace.rs:346-363`, `arqix_trace.py:274-292`, `check_requirements.py` | the canonical-owner model itself |
| K3 | Marker/CURIE prefix `arqix:` + verbs | all marker parsers (Rust + Py) | tool identity (ADR-0009); changing it breaks the whole ontology vocabulary |
| K4 | Trace-walk skip dirs (fixed set) | `trace.rs:15`, `arqix_trace.py:61` — vs configurable `config.rs:26` | deliberate per REQ-01-01-17 + oracle conformance; **but** the three identical lists should share one constant per language so they cannot drift |
| K5 | EARS patterns + RFC-2119 keyword subsets | `check_requirements.py:71-104` | normative style guide; config would hollow out the check |
| K6 | Exit codes 0/1/2/70 | tool-wide | stable contract (REQ-00-00-00-02) — must NOT be configurable |
| K7 | ISO-8601 dates, `iri = lowercase(id)`, filename `<ID>-<slug>.md`, corpus extensions `.md/.rs`, kind slug charset, `arqix.toml` itself | various | standards, safety (containment), or the config anchor itself |

## Decision guidance

- **D2a (scope):** recommend cutting the strand as three stories (PLANS.md US-01-01-18/19/20) covering C1–C7 + C10; C8 is strand 1; C9/C11–C14 ride along where they touch the same code, or wait.
- **D2b (hard boundary):** everything oracle-coupled (C10, K1–K4) must feed Rust *and* Python from the same source, or the conformance suite breaks — that is the acceptance criterion for those stories, not an afterthought.
- **D2c (immediate bug):** C6's existing divergence (`generated` required by one checker, ignored by the other) is worth a small fix ahead of the config story.
