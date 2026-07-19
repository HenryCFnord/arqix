# Changelog

All notable changes to arqix are documented in this file.
The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/); versioning follows the policy in [RELEASING.md](RELEASING.md).
Breaking changes carry a **Migration** note in their entry.

## [0.3.0] — unreleased

Nothing yet.

## [0.2.0] — 2026-07-16

### Added

- Creation aliases `req new`, `us new`, and `adr new` — the same code path as `doc new <kind>`, with `--title`, `--id`, and `--dry-run`.
- Include containment tightened (`ASM-006`): an include target inside the repository but outside every configured root is refused — corpus composition only ever reads the corpus.
- `arqix:plans <REQUIREMENT-ID>`: the language-neutral planned claim — counts as planned in coverage without framework skip syntax, satisfies the test-marker duty like `verifies`; Rust's `#[ignore]` detection stays as a convenience.
- `trace coverage --results <junit.xml>`: joined test outcomes by test name — a failing or skipped test demotes its verifying claim to planned, unjoined claims stay untouched, and coverage rows carry the outcomes for downstream evidence exports.
- `mcp serve`: the corpus as MCP tools — `search`, `read`, and `list` over stdio (JSON-RPC 2.0, one message per line), answering with the same JSON as the CLI surface; the protocol subset is implemented directly per ADR-0014, no SDK dependency.
- `publish site` stages a generated specification catalogue when enabled (`[policies.publish] specification-catalogue`): one page per workflow group bundling stories and their derived requirements, an HTML anchor per ID, live coverage status from the trace graph — the excluded spec sources return to the site in bundled form.
- `doc init` scaffolds an `AGENTS.md` agent-instructions starting point at the repository root — the verification loop and the corpus entry points named from the first commit, never overwriting an authored one.
- `arqix init`: a top-level alias for `doc init` — the discoverable entry point for repository initialisation, the same code path and arguments as `doc init` (mirrors the `req new`/`us new`/`adr new` aliases).
- `trace freshness`: flags a `verifies`/`implements` marker as possibly stale when its target requirement's document was committed to version control after the marker's own file — git arithmetic, not code analysis (ADR-0015); `verify` runs it as an informational sub-step so the loop measures current verification rather than historical marker placement.
- `render pdf [<file>...] [--lang <lang>] [--out <path>]`: PDF artefacts via the configured renderer (Pandoc by default) — staged artefact-ready pages or selected files as input, `--defaults` and `--template eisvogel` passed through when configured, per-package overrides, artefact storage per the configured mode, tool errors forwarded transparently.
- `report bundle <ID>... [--out <dir>] [--stamp <text>]`: scoped evidence bundles — a story ID stands for the requirements derived from it; the bundle directory carries `bundle.json`, `evidence.md`, and the scoped `matrix.csv` with stable schemas and caller-provided generation metadata.
- `report knowledge [--out <dir>]`: the corpus as an [Open Knowledge Format](https://github.com/GoogleCloudPlatform/knowledge-catalog) bundle — one artefact-ready concept document per living corpus document (includes expanded, directives stripped), OKF fields mapped from declared metadata and never fabricated, publish scope and lifecycle honoured.
- `policy check <file>...`: changed files evaluated against the change scope declared in `[policies.change]` — an `allow` list of path prefixes (trailing slash for a subtree, exact entry for one file), violations as `POL-001` diagnostics, gate mode exiting 1 and warn-only mode reporting without failing; no declared policy means nothing to enforce.
- `doc new <family>` creates in the declared `[kinds.<family>].dir` — creation and validation read the same contract (one source, ADR-0011); unconfigured kinds keep the `<root>/<kind>/` default.
- `[kinds.<family>].template` names the family's template file directly; the placeholder vocabulary (`{id} {title} {slug} {iri_slug} {kind} {namespace} {lifecycle}`) is validated — an unknown placeholder is a `TPL-002` finding, never a silent literal.
- `doc new --dir <path>`: explicit repository-relative placement (also on the creation aliases), containment-guarded — absolute paths and `..` are usage errors.
- Source records: `arqix:classes/source` joins the ontology with its provenance contract (the `SRC` rule family in `lint frontmatter`, keyed on `rdf.type`) — uri and access date required at finalisation, `local-copy`/`sha256` as an optional pair, copies stay outside the documentation roots; `[kinds.source]` provides the creation surface.
- `report statements`: every requirement's normative-sentence classification — id, kind, modality, EARS pattern, subject — as CSV, projected from the same functions the checker enforces with; the committed export joins the snapshot freshness gate.
- Report catalog questions Q-09 and Q-10 answered: a deterministic lines-of-code unit joins the snapshots, and `report coverage` renders the test-coverage unit from a cargo-llvm-cov export (CI-generated, outside the byte-for-byte gate).
- Story-workflow coupling is machine-checked: `US-WF-001` (the story id encodes the workflow named by `is-part-of-workflow`) and `US-PER-001` (the story's persona is declared on its workflow; consolidation personas are exempt as corpus data).
- `publish site` stages every corpus CSV as a generated Markdown table page and rewrites relative CSV links to it — the trace matrices and data exports are browsable on the published site.

### Changed

- The Python reference checkers retired after conformance: the Rust engine owns every corpus check, `just verify` is the daily gate, and the mirrored oracle-selftest fixtures in the test suite own the checker contracts.
- EARS classification tries the most specific pattern first: a multi-clause sentence now classifies as `complex` instead of its leading clause's simple pattern — visible in the `report statements` export, byte-identical on single-clause corpora.
- `REQ-META-001` resolves the effective `[kinds.req].required-meta` contract (REQ-01-01-19-03) instead of a hardcoded key set — validation and configuration can no longer disagree.
- `publish site` stages into a clean tree, and an include target that a corpus page links to stays a standalone page — a page can be embedded by the landing page and still exist at its own URL.

### Fixed

- The published site lost the scoreboard unit (and any other linked include target) on fresh builds; stale local staging masked the loss.
- The Pages deploy no longer cancels a run mid-deploy when the snapshot-refresh re-dispatch follows a merge push.

## [0.1.0] — 2026-07-11

The first release: arqix verifies and publishes its own corpus.

### Added

- Command surface per ADR-0005: `config validate/show`, `doc init/new/list/read/search`, `unit new`, `fmt`, `finalise`, `lint run`, `assemble build`, `trace scan/check/coverage/matrix/ratchet`, `verify`, `publish site`; the remaining commands (`report bundle`, `policy check`, `mcp serve`, `render pdf`) are stubs with exit 70.
- Document parser, store, and catalog: frontmatter, classes and triples, full-text search, JSON output with per-interface `schema_version`.
- Linter contract checks: duplicate ids, include targets, reference markers, translation sources, lifecycle vocabulary per document nature, and the machine-checked done claim (LNT-001..005, LNT-010).
- Formatter and finaliser as the single mechanical mutator (ADR-0004), byte-identical and idempotent on the corpus.
- Trace engine as the Rust port of the Python oracle, JSON-value-equal on the corpus; coverage by requirement kind and matrices as CSV.
- Verification orchestrator with a configured sub-step policy: informational coverage by default, the coverage ratchet gating regressions against the committed matrix snapshot.
- Template engine: package scaffold, `doc new` with `--title`, `--id`, and `--dry-run`.
- Assembler with include expansion, cycle detection, and a JSONL assembly log.
- Publisher: per-language staging of artefact-ready inputs and orchestration of a configured site toolchain — arqix stages, the toolchain renders.
- Configuration schema v1 (`arqix.toml`): roots, skip-dirs, verify policy, publish policy, i18n default language.

### Changed

- License: relicensed from GPL-3.0-or-later to MIT OR Apache-2.0 (dual), the Rust ecosystem convention, ahead of the first crates.io release; sole-author relicensing, no external contributions affected.

### Security

- Containment discipline: no writes outside configured roots, no content execution, directives never reach published outputs.
