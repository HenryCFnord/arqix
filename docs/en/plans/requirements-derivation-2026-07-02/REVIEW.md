# Requirements review table

Generated from `docs/en/architecture/req/` — do not edit by hand; regenerate after corpus changes.

Total: 142 requirements (103 functional, 17 quality, 22 constraint). Stories column: canonical owner first, then further demanding stories (`+`).

## Coverage per persona group

Under the canonical-owner model a requirement is *owned* by the lowest-ID story that demands it, so ownership counts fall for later groups by construction. The *demands* column (requirements whose `derived-from` includes a story of the group) is the meaningful coverage view: group 08 owns only 4 requirements but demands 75 — the most of all groups — because nearly everything a coding agent needs was first demanded by lower groups. The story corpus is deliberately a personas-by-features matrix (the normalization session created per-persona views of the same features), which is why 150 of 177 distinct acceptance-criteria behaviours are shared by 2–4 stories.

| Group | Persona | Stories | Owns | Demands |
| --- | --- | --- | --- | --- |
| 01 | Mara Maintainer | 16 | 52 | 64 |
| 02 | Dan Developer | 11 | 14 | 42 |
| 03 | Quinn QA | 8 | 17 | 29 |
| 04 | Daria DevOps | 13 | 30 | 50 |
| 05 | Alex AIOps | 14 | 10 | 37 |
| 06 | Aria Architect | 11 | 1 | 38 |
| 07 | Avery Auditor | 7 | 0 | 9 |
| 08 | Casey Coding Agent | 23 | 4 | 75 |

## Cross-cutting foundation (REQ-00-00-00-NN)

| ID | Kind | Requirement | Stories |
| --- | --- | --- | --- |
| REQ-00-00-00-01 | C | The arqix CLI SHALL produce byte-identical outputs for identical inputs and configuration. | 01-01-03 +01-01-04 +01-01-08 +01-01-10 +01-01-12 +01-01-16 +02-01-03 +02-01-04 +02-01-06 +02-01-09 +02-01-11 +03-01-01 +03-01-02 +03-01-03 +03-01-04 +03-01-07 +03-01-08 +04-01-06 +04-01-10 +04-01-11 +05-01-03 +05-01-04 +05-01-06 +05-01-08 +05-01-09 +05-01-10 +05-01-11 +05-01-14 +06-01-04 +06-01-08 +06-01-09 +06-01-10 +07-01-01 +07-01-02 +07-01-03 +07-01-05 +07-01-06 +08-01-01 +08-01-03 +08-01-04 +08-01-06 +08-01-07 +08-01-09 +08-01-10 +08-01-20 +08-01-21 +08-01-22 |
| REQ-00-00-00-02 | F | The arqix CLI SHALL signal command outcomes through documented, stable exit codes. | 01-01-14 +04-01-04 +04-01-05 +04-01-07 +04-01-08 +04-01-10 +05-01-05 +05-01-13 +05-01-14 +08-01-11 +08-01-13 +08-01-15 +08-01-21 |
| REQ-00-00-00-03 | F | When arqix emits a diagnostic, arqix SHALL provide it in a documented machine-readable format. | 01-01-07 +01-01-08 +01-01-14 +03-01-03 +03-01-05 +03-01-06 +03-01-08 +04-01-01 +04-01-02 +04-01-04 +04-01-05 +04-01-07 +04-01-10 +04-01-12 +05-01-02 +05-01-05 +05-01-07 +05-01-08 +05-01-13 +05-01-14 +06-01-02 +07-01-01 +07-01-04 +07-01-06 +07-01-07 +08-01-02 +08-01-07 +08-01-08 +08-01-11 +08-01-13 +08-01-16 +08-01-19 +08-01-21 +08-01-22 |
| REQ-00-00-00-04 | F | The arqix CLI SHALL derive document IDs and slugs deterministically from the configured policy. | 01-01-01 +01-01-04 +01-01-05 +01-01-12 +02-01-01 +02-01-04 +02-01-05 +03-01-01 +05-01-10 +06-01-03 +06-01-10 +08-01-01 +08-01-04 +08-01-05 +08-01-09 |
| REQ-00-00-00-05 | F | When a document is created, arqix SHALL instantiate the configured template for the requested kind. | 01-01-05 +01-01-10 +01-01-13 +02-01-05 +02-01-07 +05-01-03 +06-01-03 +08-01-05 +08-01-10 +08-01-23 |
| REQ-00-00-00-06 | F | The arqix CLI SHALL resolve every command against the effective configuration. | 01-01-01 +01-01-02 +01-01-03 +01-01-04 +01-01-05 +01-01-13 +01-01-14 +01-01-16 +02-01-02 +02-01-03 +02-01-04 +02-01-05 +02-01-07 +02-01-09 +02-01-11 +03-01-01 +03-01-05 +03-01-08 +04-01-01 +04-01-03 +04-01-04 +04-01-05 +04-01-06 +04-01-07 +04-01-11 +05-01-01 +05-01-02 +05-01-04 +05-01-05 +05-01-07 +05-01-09 +05-01-11 +05-01-13 +06-01-01 +06-01-02 +06-01-03 +06-01-04 +06-01-05 +06-01-08 +07-01-04 +07-01-06 +08-01-01 +08-01-02 +08-01-03 +08-01-04 +08-01-05 +08-01-11 +08-01-13 +08-01-16 +08-01-20 +08-01-22 +08-01-23 |
| REQ-00-00-00-07 | C | The arqix CLI SHALL NOT modify files outside the declared change scope. | 01-01-07 +04-01-02 +08-01-08 |
| REQ-00-00-00-08 | C | The arqix CLI SHALL NOT overwrite existing files without explicit approval. | 01-01-01 +01-01-06 +02-01-01 +02-01-08 +08-01-01 +08-01-06 |
| REQ-00-00-00-09 | F | Where a command creates or modifies files, the command SHALL support a dry-run mode that reports planned changes without writing. | 01-01-13 +02-01-07 +02-01-10 +06-01-06 +08-01-14 +08-01-23 |
| REQ-00-00-00-10 | F | When translations exist for a document, arqix SHALL detect missing and outdated translations deterministically. | 01-01-14 +04-01-04 +05-01-05 +08-01-11 |
| REQ-00-00-00-11 | Q | Search and read commands SHOULD return results within one second on a repository of one thousand documents. | 02-01-06 +05-01-06 +06-01-09 |
| REQ-00-00-00-12 | Q | The verification loop SHOULD complete within ten seconds on a repository of one thousand documents. | 04-01-05 +08-01-13 |
| REQ-00-00-00-13 | C | The arqix CLI SHALL NOT access files outside the repository root and the configured allowed roots. | 01-01-07 +02-01-09 +04-01-02 +05-01-04 +06-01-04 +08-01-08 |
| REQ-00-00-00-14 | C | The arqix CLI SHALL NOT execute code or shell commands embedded in processed documents. | 01-01-07 +04-01-02 +08-01-08 |

## Owned by group 01 stories

| ID | Kind | Requirement | Stories |
| --- | --- | --- | --- |
| REQ-01-01-01-01 | F | When `arqix doc init <path>` is invoked, arqix SHALL create the standard doc-package scaffold with `index.md`, `units/`, `pages/`, `artefacts/`, `logs/`, and `.arqix/`. | 01-01-01 +02-01-01 |
| REQ-01-01-01-02 | F | When a doc package is initialised, arqix SHALL write `index.md` frontmatter containing `id`, `kind=doc_index`, and `title`. | 01-01-01 +02-01-01 |
| REQ-01-01-02-01 | F | When `arqix unit new` is invoked, arqix SHALL create a unit file in the configured unit location. | 01-01-02 +02-01-02 +05-01-01 +06-01-01 |
| REQ-01-01-02-02 | F | The arqix CLI SHALL support unit files that declare a global ID in frontmatter or via a supported directive. | 01-01-02 +02-01-02 +05-01-01 +06-01-01 |
| REQ-01-01-02-03 | Q | The `unit new` command help SHOULD explain where units are created, which metadata is optional, and how IDs are supplied. | 01-01-02 +02-01-02 +05-01-01 +06-01-01 |
| REQ-01-01-03-01 | F | When `arqix fmt` runs, arqix SHALL sort frontmatter keys according to the configured `key_order`. | 01-01-03 +02-01-03 +08-01-03 |
| REQ-01-01-03-02 | F | When `arqix fmt` runs, arqix SHALL normalise directives, including attribute order and whitespace, without semantic changes. | 01-01-03 +02-01-03 +08-01-03 |
| REQ-01-01-03-03 | C | The arqix CLI SHALL NOT change document meaning during formatting beyond canonical ordering and whitespace normalisation. | 01-01-03 +08-01-03 |
| REQ-01-01-04-01 | F | When `arqix lint run` executes, arqix SHALL verify that include targets exist. | 01-01-04 +02-01-04 +03-01-01 +08-01-04 |
| REQ-01-01-04-02 | F | When `arqix lint run` executes, arqix SHALL report forbidden frontmatter keys in units according to the configured allowlist. | 01-01-04 +02-01-04 +03-01-01 +08-01-04 |
| REQ-01-01-04-03 | F | When `arqix lint run` executes, arqix SHALL report duplicate IDs globally across units, requirements, user stories, ADRs, and glossary entries. | 01-01-04 +02-01-04 +03-01-01 +08-01-04 |
| REQ-01-01-04-04 | F | When a lint check reports a finding, the diagnostic SHALL include precise file and line context. | 01-01-04 +02-01-04 +03-01-01 +08-01-04 |
| REQ-01-01-04-05 | F | If lint input is invalid, then arqix SHALL return a failing status. | 01-01-04 +02-01-04 +03-01-01 +08-01-04 |
| REQ-01-01-05-01 | F | The arqix CLI SHALL accept only `<kind>` values that are defined in configuration. | 01-01-05 +02-01-05 +06-01-03 +08-01-05 |
| REQ-01-01-05-02 | Q | The arqix CLI SHOULD provide the aliases `req new`, `us new`, and `adr new` for template-based creation. | 01-01-05 +02-01-05 +06-01-03 +08-01-05 |
| REQ-01-01-05-03 | F | When a document is created from a template, arqix SHALL substitute the placeholders `{title}`, `{slug}`, and `{id}`. | 01-01-05 +02-01-05 +06-01-03 +08-01-05 |
| REQ-01-01-06-01 | F | When `arqix finalise` runs, arqix SHALL set `updated` to an ISO-8601 date in `YYYY-MM-DD` format. | 01-01-06 +02-01-08 +08-01-06 |
| REQ-01-01-06-02 | F | If a metadata value is already current, then `arqix finalise` SHALL NOT rewrite it. | 01-01-06 +02-01-08 +08-01-06 |
| REQ-01-01-06-03 | F | If a file has no supported frontmatter, then `arqix finalise` SHALL fail with a clear diagnostic. | 01-01-06 +02-01-08 +08-01-06 |
| REQ-01-01-07-01 | F | The arqix CLI SHALL support a policy file in minimal YAML or TOML that declares the allowed change scope. | 01-01-07 +04-01-02 +08-01-08 |
| REQ-01-01-07-02 | F | When `arqix policy check` is invoked with a list of changed files, arqix SHALL evaluate them against the declared policy. | 01-01-07 +04-01-02 +08-01-08 |
| REQ-01-01-07-03 | F | Where warn-only mode is configured, `arqix policy check` SHALL report violations without failing. | 01-01-07 +04-01-02 +08-01-08 |
| REQ-01-01-08-01 | F | When `arqix trace coverage` runs, arqix SHALL identify requirements without `verifies` tests. | 01-01-08 +03-01-03 +07-01-01 |
| REQ-01-01-08-02 | F | When `arqix trace coverage` runs, arqix SHALL identify requirements without `implements` code. | 01-01-08 +03-01-03 +07-01-01 |
| REQ-01-01-08-03 | F | The coverage report SHALL support at least Markdown and JSON output formats. | 01-01-08 +03-01-03 +07-01-01 |
| REQ-01-01-09-01 | C | The agent instruction document SHALL define scope rules for story-by-story execution, including one story at a time and no opportunistic refactors. | 01-01-09 +08-01-18 |
| REQ-01-01-09-02 | C | The agent instruction document SHALL define editing constraints for the plan document and the required arqix verification loop. | 01-01-09 +08-01-18 |
| REQ-01-01-09-03 | C | The plan document SHALL include story tasks with scope boundaries, acceptance criteria, required command checks, and agent-updatable status fields. | 01-01-09 +08-01-18 |
| REQ-01-01-09-04 | Q | The agent instruction and plan document structures SHOULD be explicit enough that an agent can follow them without guessing process constraints. | 01-01-09 +08-01-18 |
| REQ-01-01-09-05 | F | When a coding agent is supported, its agent-specific extension points SHALL be documented. | 01-01-09 +08-01-18 |
| REQ-01-01-09-06 | C | Agent-specific extension points SHALL NOT carry normative process rules. | 01-01-09 +08-01-18 |
| REQ-01-01-10-01 | F | The arqix CLI SHALL support schema contracts that declare required and optional metadata fields per document kind. | 01-01-10 +05-01-03 +08-01-10 |
| REQ-01-01-10-02 | F | When lint validates metadata against a contract, arqix SHALL report missing, extra, and type-invalid fields. | 01-01-10 +05-01-03 +08-01-10 |
| REQ-01-01-10-03 | C | The template and validation subsystems SHALL use the same contract source. | 01-01-10 +05-01-03 +08-01-10 |
| REQ-01-01-11-01 | C | The arc42 architecture document SHALL be structured into units per chapter and remain assemblable into one document. | 01-01-11 +06-01-07 |
| REQ-01-01-11-02 | C | ADRs SHALL be maintained using the path model with a canonical governance language. | 01-01-11 +06-01-07 |
| REQ-01-01-11-03 | C | The documentation strategy SHALL span the handbook, CLI help, man page, and rustdoc layers. | 01-01-11 +06-01-07 |
| REQ-01-01-11-04 | Q | Architecture views SHOULD use Mermaid diagrams in a C4-oriented modelling style. | 01-01-11 +06-01-07 |
| REQ-01-01-11-05 | Q | The architecture documentation SHOULD record a future documentation consistency check as an extension path. | 01-01-11 +06-01-07 |
| REQ-01-01-12-01 | F | When `arqix doc new glossary` is invoked, arqix SHALL create a glossary term with the required metadata and route it to the configured location. | 01-01-12 +06-01-10 |
| REQ-01-01-12-02 | F | The arqix CLI SHALL support referencing glossary terms by stable ID from ADRs and other documents. | 01-01-12 +06-01-10 |
| REQ-01-01-12-03 | F | When `arqix lint run` executes, arqix SHALL detect duplicate or malformed glossary IDs. | 01-01-12 +06-01-10 |
| REQ-01-01-13-01 | F | When no `--id` is provided, arqix SHALL generate an ID from the configured policy and verify its uniqueness. | 01-01-13 +02-01-07 +08-01-23 |
| REQ-01-01-13-02 | F | When `arqix doc new <kind>` is invoked with a title, arqix SHALL create the document in the configured location for that kind. | 01-01-13 +02-01-07 +08-01-23 |
| REQ-01-01-14-01 | F | The set of translation-required kinds or domains SHALL be configurable in `arqix.toml`. | 01-01-14 +04-01-04 +05-01-05 +08-01-11 |
| REQ-01-01-15-01 | C | The repository SHALL maintain `CHANGELOG.md` and `RELEASING.md` consistently with each other. | 01-01-15 +04-01-09 +08-01-17 |
| REQ-01-01-15-02 | C | The release process SHALL document SemVer rules for the product version and the separate `config_version` and `schema_version`. | 01-01-15 +04-01-09 +08-01-17 |
| REQ-01-01-15-03 | C | Coding agents SHALL NOT tag or publish releases without explicit approval. | 01-01-15 +04-01-09 +08-01-17 |
| REQ-01-01-15-04 | C | If a release contains breaking changes, then the release preparation SHALL include migration notes and changelog entries. | 01-01-15 +04-01-09 +08-01-17 |
| REQ-01-01-16-01 | F | When `arqix config validate` runs, arqix SHALL report schema and contract violations. | 01-01-16 +04-01-11 +05-01-11 +08-01-20 |
| REQ-01-01-16-02 | F | When `arqix config show` runs, arqix SHALL render the effective configuration after defaults and overrides are applied. | 01-01-16 +04-01-11 +05-01-11 +08-01-20 |
| REQ-01-01-16-03 | Q | Configuration diagnostics SHOULD identify the failing key and source file. | 01-01-16 +04-01-11 +05-01-11 +08-01-20 |

## Owned by group 02 stories

| ID | Kind | Requirement | Stories |
| --- | --- | --- | --- |
| REQ-02-01-03-01 | Q | The formatting output SHOULD keep document diffs focused on content rather than incidental style changes. | 02-01-03 |
| REQ-02-01-06-01 | F | The arqix CLI SHALL provide full-text search over the documentation. | 02-01-06 |
| REQ-02-01-06-02 | F | When `arqix doc read` is invoked with a document ID, arqix SHALL return the document, optionally scoped to a section or anchor. | 02-01-06 |
| REQ-02-01-06-03 | F | If a document or anchor cannot be found, then arqix SHALL fail with a diagnostic naming the missing element. | 02-01-06 |
| REQ-02-01-09-01 | F | The arqix CLI SHALL parse `<!-- arqix:chapter ... -->` and `<!-- arqix:include ... -->` directives. | 02-01-09 |
| REQ-02-01-09-02 | C | The arqix CLI SHALL NOT resolve include targets outside the configured allowed roots. | 02-01-09 |
| REQ-02-01-09-03 | F | When a glob include is expanded, arqix SHALL apply the configured sorting. | 02-01-09 |
| REQ-02-01-10-01 | F | When a translation is scaffolded, arqix SHALL create the translation file at the location the chosen i18n layout defines. | 02-01-10 |
| REQ-02-01-10-02 | F | When a translation is scaffolded, arqix SHALL write metadata linking the translation to its source document ID. | 02-01-10 |
| REQ-02-01-10-03 | F | If the source document cannot be found, then the translation scaffolding SHALL fail with a clear diagnostic. | 02-01-10 |
| REQ-02-01-10-04 | F | When a translation is scaffolded, arqix SHALL preserve arqix markup directives and structural elements according to the scaffold strategy. | 02-01-10 |
| REQ-02-01-11-01 | F | When `arqix assemble build` runs for a doc package, arqix SHALL generate the assembled outputs under `pages/`. | 02-01-11 |
| REQ-02-01-11-02 | F | Where `strip_frontmatter_on_include` is enabled, arqix SHALL strip frontmatter from included content. | 02-01-11 |
| REQ-02-01-11-03 | F | If an include cycle is detected, then arqix SHALL fail with a clear error message. | 02-01-11 |

## Owned by group 03 stories

| ID | Kind | Requirement | Stories |
| --- | --- | --- | --- |
| REQ-03-01-02-01 | F | When `arqix trace matrix` runs, arqix SHALL export the selected matrix as CSV. | 03-01-02 |
| REQ-03-01-02-02 | F | The arqix CLI SHALL support at least the `REQ×Test` and `US×REQ` matrix types. | 03-01-02 |
| REQ-03-01-02-03 | F | The exported CSV SHALL use stable headers and a deterministic row model for each supported matrix type. | 03-01-02 |
| REQ-03-01-02-04 | Q | Empty-link cases SHOULD remain visible in the exported matrix in a reviewer-friendly form. | 03-01-02 |
| REQ-03-01-04-01 | F | When an evidence bundle export is invoked with one or more requirement or story IDs, arqix SHALL export a bundle scoped to those IDs. | 03-01-04 |
| REQ-03-01-04-02 | F | The exported bundle SHALL include the linked requirements, stories, diagnostics, and trace outputs relevant to the chosen scope. | 03-01-04 |
| REQ-03-01-04-03 | Q | The exported bundle SHOULD be reviewable without manual reshaping of the source evidence. | 03-01-04 |
| REQ-03-01-05-01 | F | When `arqix trace scan` runs, arqix SHALL detect configurable trace markers in Rust comments. | 03-01-05 |
| REQ-03-01-05-02 | F | When `arqix trace scan` runs, arqix SHALL detect trace markers in Markdown HTML comments. | 03-01-05 |
| REQ-03-01-05-03 | F | When `arqix trace scan` runs, arqix SHALL read unit frontmatter links such as requirements, stories, ADRs, and refs. | 03-01-05 |
| REQ-03-01-05-04 | F | When `arqix trace scan` completes, arqix SHALL output a graph of nodes and edges as JSON. | 03-01-05 |
| REQ-03-01-05-05 | C | The arqix CLI SHALL NOT silently drop unresolved references from trace reports. | 03-01-05 |
| REQ-03-01-06-01 | F | When `arqix trace check` is invoked for a requirement, arqix SHALL report whether `implements` markers exist for it. | 03-01-06 |
| REQ-03-01-06-02 | F | When `arqix trace check` is invoked for a requirement, arqix SHALL report whether `verifies` markers exist for it. | 03-01-06 |
| REQ-03-01-06-03 | F | The trace check report SHALL include the locations of existing markers with path and line context. | 03-01-06 |
| REQ-03-01-07-01 | F | When a trace or coverage report is generated, arqix SHALL support filtering by document kind, status, and missing-link category. | 03-01-07 |
| REQ-03-01-07-02 | F | Each report finding SHALL link back to the originating document or file location. | 03-01-07 |

## Owned by group 04 stories

| ID | Kind | Requirement | Stories |
| --- | --- | --- | --- |
| REQ-04-01-01-01 | Q | The assembly log SHOULD be collectable as a CI artefact without post-processing or field-name guessing. | 04-01-01 |
| REQ-04-01-01-02 | F | When `arqix assemble build` runs, arqix SHALL write a JSONL log during assembly. | 04-01-01 +05-01-02 +06-01-02 +08-01-02 |
| REQ-04-01-01-03 | F | The assembly log path SHALL be configurable. | 04-01-01 +05-01-02 +06-01-02 +08-01-02 |
| REQ-04-01-01-04 | F | When an assembly step executes, arqix SHALL emit exactly one stable JSONL record for it. | 04-01-01 +05-01-02 +06-01-02 +08-01-02 |
| REQ-04-01-01-05 | F | Each assembly log record SHALL contain at least `doc`, `chapter_id`, `out`, `include`, `sha256`, `bytes`, and `at_line`. | 04-01-01 +05-01-02 +06-01-02 +08-01-02 |
| REQ-04-01-03-01 | F | When `arqix publish` runs, arqix SHALL generate publishing outputs for the configured PDF or website targets. | 04-01-03 +06-01-05 |
| REQ-04-01-03-02 | F | The assembled pages SHALL be artefact-ready for downstream publishing. | 04-01-03 +06-01-05 |
| REQ-04-01-03-03 | F | Where site build orchestration is configured, arqix SHALL orchestrate the site build. | 04-01-03 +06-01-05 |
| REQ-04-01-03-04 | F | When `arqix render pdf` runs, arqix SHALL invoke Pandoc on the assembled pages or the selected Markdown files. | 04-01-03 +06-01-05 |
| REQ-04-01-03-05 | F | The arqix CLI SHALL support Pandoc `--defaults` files and the eisvogel template option when configured. | 04-01-03 +06-01-05 |
| REQ-04-01-03-06 | F | When render artefacts are produced, arqix SHALL store them according to the configured artefact mode. | 04-01-03 +06-01-05 |
| REQ-04-01-03-07 | F | If an external rendering tool fails, then arqix SHALL forward the tool error transparently. | 04-01-03 +06-01-05 |
| REQ-04-01-03-08 | F | The arqix CLI SHALL support per-doc-package render configuration and overrides. | 04-01-03 +06-01-05 |
| REQ-04-01-05-01 | F | When `arqix verify` runs, arqix SHALL execute the configured sub-steps of format, lint, trace scan, and coverage. | 04-01-05 +08-01-13 |
| REQ-04-01-05-02 | F | The verification loop SHALL support fail-fast and aggregate result modes, selected by configuration. | 04-01-05 +08-01-13 |
| REQ-04-01-05-03 | F | Where JSON mode is enabled, the verification loop SHALL emit per-step results and diagnostic references. | 04-01-05 +08-01-13 |
| REQ-04-01-05-04 | C | The default verification loop SHALL NOT include rendering. | 04-01-05 +08-01-13 |
| REQ-04-01-07-01 | F | When `arqix publish site` is invoked with a language, arqix SHALL build the site from that language's configured root and write outputs to that language's artefact target. | 04-01-07 +05-01-13 |
| REQ-04-01-07-02 | F | If the site toolchain fails, then arqix SHALL return exit code 2 with diagnostics identifying the failing tool invocation context. | 04-01-07 +05-01-13 |
| REQ-04-01-08-01 | F | The arqix CLI SHALL use exit code `0` for success, `1` for lint or quality-gate failure, and `2` for usage error. | 04-01-08 +08-01-15 |
| REQ-04-01-08-02 | F | The stderr messaging SHALL let CI distinguish command errors from quality failures. | 04-01-08 +08-01-15 |
| REQ-04-01-08-03 | F | The arqix CLI MAY ship a minimal GitHub Actions template for typical gates, aligned with supported commands only. | 04-01-08 +08-01-15 |
| REQ-04-01-10-01 | F | Each supported command SHALL accept `--format json` or an equivalent option to emit JSON diagnostics. | 04-01-10 +05-01-14 +08-01-21 |
| REQ-04-01-10-02 | F | JSON diagnostics SHALL include at least `severity`, `code`, `message`, `source.path`, and `source.line` where available. | 04-01-10 +05-01-14 +08-01-21 |
| REQ-04-01-12-01 | F | Audit-oriented report exports SHALL support at least Markdown, CSV, and JSON where applicable. | 04-01-12 +07-01-07 |
| REQ-04-01-12-02 | F | Export schemas and column ordering SHALL remain stable across runs. | 04-01-12 +07-01-07 |
| REQ-04-01-12-03 | F | Report metadata SHALL record the generation time, scope, and source inputs. | 04-01-12 +07-01-07 |
| REQ-04-01-13-01 | F | When architecture source units are assembled, arqix SHALL produce a predictable chapter order. | 04-01-13 +06-01-11 |
| REQ-04-01-13-02 | F | Cross-links between ADRs, glossary terms, and architecture pages SHALL resolve consistently in assembled outputs. | 04-01-13 +06-01-11 |
| REQ-04-01-13-03 | F | When publish validation runs, arqix SHALL report broken architecture navigation paths. | 04-01-13 +06-01-11 |

## Owned by group 05 stories

| ID | Kind | Requirement | Stories |
| --- | --- | --- | --- |
| REQ-05-01-02-01 | Q | The assembly log records SHOULD be parseable by downstream tooling without guessing field names. | 05-01-02 |
| REQ-05-01-08-01 | F | When `arqix doc list` runs, arqix SHALL emit a JSON catalog with stable ordering and core metadata for each document. | 05-01-08 +08-01-07 |
| REQ-05-01-08-02 | F | Each catalog entry SHALL include at least `id`, `kind`, `title`, `lang`, and the source path. | 05-01-08 +08-01-07 |
| REQ-05-01-08-03 | F | When `arqix doc list` is invoked with kind or language filters, arqix SHALL filter the catalog accordingly. | 05-01-08 +08-01-07 |
| REQ-05-01-10-01 | F | When `arqix doc read` is invoked with a heading slug or explicit anchor, arqix SHALL return the selected section. | 05-01-10 +08-01-09 |
| REQ-05-01-10-02 | F | The structured read output SHALL include the resolved document metadata and selector details. | 05-01-10 +08-01-09 |
| REQ-05-01-10-03 | F | If a read fails, then the diagnostic SHALL identify whether the document or the selector was not found. | 05-01-10 +08-01-09 |
| REQ-05-01-12-01 | F | When `arqix mcp serve` runs, arqix SHALL serve MCP over stdio transport. | 05-01-12 +08-01-12 |
| REQ-05-01-12-02 | F | The MCP server SHALL expose at least the tools `search`, `read`, and `list`. | 05-01-12 +08-01-12 |
| REQ-05-01-12-03 | C | The MCP transport handling SHALL remain separate from the tool logic. | 05-01-12 +08-01-12 |

## Owned by group 06 stories

| ID | Kind | Requirement | Stories |
| --- | --- | --- | --- |
| REQ-06-01-02-01 | Q | The assembly log SHOULD allow document composition to be reviewed without inferring hidden assembly steps. | 06-01-02 |

## Owned by group 08 stories

| ID | Kind | Requirement | Stories |
| --- | --- | --- | --- |
| REQ-08-01-01-01 | Q | Failure diagnostics SHOULD make the stop condition clear enough to act on without reading source code. | 08-01-01 |
| REQ-08-01-01-02 | Q | The generated doc package SHOULD be usable directly in the verification loop without manual interpretation or repair. | 08-01-01 |
| REQ-08-01-02-01 | Q | Assembly outcomes SHOULD be interpretable from the log and command result without human guesswork. | 08-01-02 |
| REQ-08-01-05-01 | F | If an unknown document kind is requested, then arqix SHALL fail with a clear, actionable error. | 08-01-05 |

