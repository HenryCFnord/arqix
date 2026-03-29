# Persona to User Story and Requirement Mapping Report

Generated on 2026-03-25 for branch `docs/add-personas-user-stories`.

## Persona Summaries
- `PER-0001`: Mara owns the repository’s documentation standards. She cares about consistency, long-term maintainability, and predictable tooling. For Mara, arqix is not a Markdown generator. It is a process enforcer for documentation-as-code.
- `PER-0002`: Dan is a developer who writes code and documentation in the same flow. He values speed and low-friction tooling. If documentation feels like a separate project, it will not happen reliably.
- `PER-0003`: Quinn ensures traceability and measurable quality. Quinn wants evidence, not opinions. For Quinn, arqix is valuable when it produces deterministic reports that reveal gaps between requirements, Implementation, and tests.
- `PER-0004`: Daria owns CI and publishing. She values reproducibility, clear exit codes, and workflows that behave the same locally and in CI. If a tool cannot be automated, it will not scale.
- `PER-0005`: Alex builds automation and RAG-friendly workflows. Alex cares about structure, stable identifiers, and machine-readable access to documentation. For Alex, documentation is a dataset.
- `PER-0006`: Aria documents architecture, decisions, and vocabulary. She cares about clarity, structure, and traceable reasoning. Aria wants documentation to remain useful as the system evolves.
- `PER-0007`: Avery evaluates evidence chains for audits and compliance reviews. Avery is not interested in tooling details, but in reproducible proof: what was required, what was decided, what was implemented, and what was verified.
- `PER-0008`: Casey is an automation-focused coding agent that executes tasks story by story. Casey is effective when inputs, rules, and outputs are deterministic and machine-readable. Casey follows contracts, not vibes.

## User Story Mapping

| Persona | Old US | New US | Kind | Path |
| --- | --- | --- | --- | --- |
| PER-0001 | US-0001 | US-1001 | existing | `docs/us/US-1001_doc-package-init.md` |
| PER-0001 | US-0002 | US-1002 | existing | `docs/us/US-1002_chapter-include-syntax.md` |
| PER-0001 | US-0003 | US-1003 | existing | `docs/us/US-1003_units-erstellen.md` |
| PER-0001 | US-0005 | US-1004 | existing | `docs/us/US-1004_assemble-log.md` |
| PER-0001 | US-0006 | US-1005 | existing | `docs/us/US-1005_format.md` |
| PER-0001 | US-0007 | US-1006 | existing | `docs/us/US-1006_lint.md` |
| PER-0001 | US-0008 | US-1007 | existing | `docs/us/US-1007_templates.md` |
| PER-0001 | US-0009 | US-1008 | existing | `docs/us/US-1008_finalize.md` |
| PER-0001 | US-0023 | US-1009 | existing | `docs/us/US-1009_release-prozess-und-semver-operationalisieren.md` |
| PER-0001 | US-8005 | US-1010 | existing | `docs/us/US-1010_guardrails-to-keep-agent-changes-within-a-declared-scope.md` |
| PER-0001 | US-8203 | US-1011 | existing | `docs/us/US-1011_i18n-lint-profile-for-missing-and-outdated-translations.md` |
| PER-0001 | - | US-1012 | draft | `docs/us/US-1012_configuration-validation-and-effective-config-output.md` |
| PER-0001 | - | US-1013 | draft | `docs/us/US-1013_schema-backed-metadata-contracts-for-document-kinds.md` |
| PER-0002 | US-0004 | US-2001 | existing | `docs/us/US-2001_assemble-build.md` |
| PER-0002 | US-8204 | US-2002 | existing | `docs/us/US-2002_translation-scaffolding-via-doc-new-with-translation-of.md` |
| PER-0003 | US-0010 | US-3001 | existing | `docs/us/US-3001_trace-scan.md` |
| PER-0003 | US-0011 | US-3002 | existing | `docs/us/US-3002_coverage-report.md` |
| PER-0003 | US-0012 | US-3003 | existing | `docs/us/US-3003_trace-matrix.md` |
| PER-0003 | US-8002 | US-3004 | existing | `docs/us/US-3004_deterministic-trace-and-coverage-outputs.md` |
| PER-0004 | US-0015 | US-4001 | existing | `docs/us/US-4001_ci-gates.md` |
| PER-0004 | US-0016 | US-4002 | existing | `docs/us/US-4002_publish.md` |
| PER-0004 | US-8007 | US-4003 | existing | `docs/us/US-4003_one-command-verification-loop-for-agents-and-ci.md` |
| PER-0004 | US-8205 | US-4004 | existing | `docs/us/US-4004_language-aware-site-publishing-zensical-first.md` |
| PER-0005 | US-0013 | US-5001 | existing | `docs/us/US-5001_search-read.md` |
| PER-0005 | US-0014 | US-5002 | existing | `docs/us/US-5002_mcp-stdio.md` |
| PER-0005 | - | US-5003 | draft | `docs/us/US-5003_list-documents-as-deterministic-machine-readable-catalog.md` |
| PER-0005 | - | US-5004 | draft | `docs/us/US-5004_read-structured-document-sections-with-stable-selectors.md` |
| PER-0006 | US-0022 | US-6001 | existing | `docs/us/US-6001_architektur-governance-doku-prozess.md` |
| PER-0006 | - | US-6002 | draft | `docs/us/US-6002_glossary-term-scaffolding-with-cross-linkable-ids.md` |
| PER-0006 | - | US-6003 | draft | `docs/us/US-6003_architecture-narrative-assembly-with-navigable-outputs.md` |
| PER-0007 | - | US-7001 | draft | `docs/us/US-7001_generate-audit-evidence-bundles-by-requirement-scope.md` |
| PER-0007 | - | US-7002 | draft | `docs/us/US-7002_filter-traceability-reports-for-audit-review-views.md` |
| PER-0007 | - | US-7003 | draft | `docs/us/US-7003_publish-stable-compliance-ready-report-exports.md` |
| PER-0008 | US-8001 | US-8001 | existing | `docs/us/US-8001_machine-readable-diagnostics-for-arqix-commands.md` |
| PER-0008 | US-8003 | US-8002 | existing | `docs/us/US-8002_document-creation-without-ambiguity-via-templates.md` |
| PER-0008 | US-8004 | US-8003 | existing | `docs/us/US-8003_assist-command-to-detect-missing-trace-markers-in-code-and-tests.md` |
| PER-0008 | US-8006 | US-8004 | existing | `docs/us/US-8004_standardize-agent-workflow-documents-agents-md-and-plans-md.md` |

## Requirement Mapping

| Persona | Old REQ | New REQ | Story | Path |
| --- | --- | --- | --- | --- |
| PER-0001 | REQ-0001 | REQ-US-1001-01 | US-1001 | `docs/req/REQ-US-1001-01_doc-init-standardstruktur-anlegen.md` |
| PER-0001 | REQ-0002 | REQ-US-1001-02 | US-1001 | `docs/req/REQ-US-1001-02_index-md-frontmatter-im-doc-init-erzeugen.md` |
| PER-0001 | REQ-0003 | REQ-US-1001-03 | US-1001 | `docs/req/REQ-US-1001-03_deterministische-id-slug-ableitung-aus-title.md` |
| PER-0001 | REQ-0004 | REQ-US-1002-01 | US-1002 | `docs/req/REQ-US-1002-01_chapter-include-direktiven-parsen.md` |
| PER-0001 | REQ-0005 | REQ-US-1002-02 | US-1002 | `docs/req/REQ-US-1002-02_include-targets-auf-erlaubte-roots-beschranken.md` |
| PER-0001 | REQ-0006 | REQ-US-1002-03 | US-1002 | `docs/req/REQ-US-1002-03_glob-includes-deterministisch-expandieren.md` |
| PER-0001 | REQ-0007 | REQ-US-1003-01 | US-1003 | `docs/req/REQ-US-1003-01_unit-new-zum-anlegen-von-unit-dateien.md` |
| PER-0001 | REQ-0008 | REQ-US-1003-02 | US-1003 | `docs/req/REQ-US-1003-02_units-mit-global-eindeutiger-id-unterstutzen.md` |
| PER-0001 | REQ-0012 | REQ-US-1004-01 | US-1004 | `docs/req/REQ-US-1004-01_assemble-jsonl-log-schreiben.md` |
| PER-0001 | REQ-0013 | REQ-US-1005-01 | US-1005 | `docs/req/REQ-US-1005-01_fmt-frontmatter-key-order-formatieren.md` |
| PER-0001 | REQ-0014 | REQ-US-1005-02 | US-1005 | `docs/req/REQ-US-1005-02_fmt-direktiven-normalisieren.md` |
| PER-0001 | REQ-0015 | REQ-US-1006-01 | US-1006 | `docs/req/REQ-US-1006-01_lint-include-targets-auf-existenz-prufen.md` |
| PER-0001 | REQ-0016 | REQ-US-1006-02 | US-1006 | `docs/req/REQ-US-1006-02_lint-verbotene-frontmatter-keys-in-units-melden.md` |
| PER-0001 | REQ-0017 | REQ-US-1006-03 | US-1006 | `docs/req/REQ-US-1006-03_lint-doppelte-ids-global-melden.md` |
| PER-0001 | REQ-0018 | REQ-US-1007-01 | US-1007 | `docs/req/REQ-US-1007-01_doc-new-template-basiert-fur-config-kinds.md` |
| PER-0001 | REQ-0019 | REQ-US-1007-02 | US-1007 | `docs/req/REQ-US-1007-02_aliases-fur-glossary-req-us-adr-new-unterstutzen.md` |
| PER-0001 | REQ-0020 | REQ-US-1007-03 | US-1007 | `docs/req/REQ-US-1007-03_template-platzhalter-title-slug-id-unterstutzen.md` |
| PER-0001 | REQ-0021 | REQ-US-1008-01 | US-1008 | `docs/req/REQ-US-1008-01_finalize-setzt-updated-datum.md` |
| PER-0001 | REQ-0022 | REQ-US-1008-02 | US-1008 | `docs/req/REQ-US-1008-02_finalize-nur-mechanische-anderungen.md` |
| PER-0002 | REQ-0009 | REQ-US-2001-01 | US-2001 | `docs/req/REQ-US-2001-01_assemble-build-implementieren.md` |
| PER-0002 | REQ-0010 | REQ-US-2001-02 | US-2001 | `docs/req/REQ-US-2001-02_frontmatter-beim-include-optional-entfernen.md` |
| PER-0002 | REQ-0011 | REQ-US-2001-03 | US-2001 | `docs/req/REQ-US-2001-03_include-zyklen-erkennen.md` |
| PER-0003 | REQ-0023 | REQ-US-3001-01 | US-3001 | `docs/req/REQ-US-3001-01_trace-scan-marker-in-rust-kommentaren-erkennen.md` |
| PER-0003 | REQ-0024 | REQ-US-3001-02 | US-3001 | `docs/req/REQ-US-3001-02_trace-scan-marker-in-markdown-html-comments-erkennen.md` |
| PER-0003 | REQ-0025 | REQ-US-3001-03 | US-3001 | `docs/req/REQ-US-3001-03_trace-scan-unit-frontmatter-links-einlesen.md` |
| PER-0003 | REQ-0026 | REQ-US-3001-04 | US-3001 | `docs/req/REQ-US-3001-04_trace-graph-als-json-ausgeben.md` |
| PER-0003 | REQ-0027 | REQ-US-3002-01 | US-3002 | `docs/req/REQ-US-3002-01_coverage-report-reqs-ohne-verifies-tests.md` |
| PER-0003 | REQ-0028 | REQ-US-3002-02 | US-3002 | `docs/req/REQ-US-3002-02_coverage-report-reqs-ohne-implements-code.md` |
| PER-0003 | REQ-0029 | REQ-US-3002-03 | US-3002 | `docs/req/REQ-US-3002-03_coverage-output-als-markdown-und-json.md` |
| PER-0003 | REQ-0030 | REQ-US-3003-01 | US-3003 | `docs/req/REQ-US-3003-01_trace-matrix-als-csv-exportieren.md` |
| PER-0004 | REQ-0035 | REQ-US-4001-01 | US-4001 | `docs/req/REQ-US-4001-01_konsistente-exit-codes-setzen.md` |
| PER-0004 | REQ-0036 | REQ-US-4001-02 | US-4001 | `docs/req/REQ-US-4001-02_github-actions-vorlage-optional-liefern.md` |
| PER-0004 | REQ-0037 | REQ-US-4002-01 | US-4002 | `docs/req/REQ-US-4002-01_site-build-orchestrieren-oder-artefaktfahige-pages-ausgeben.md` |
| PER-0004 | REQ-0038 | REQ-US-4002-02 | US-4002 | `docs/req/REQ-US-4002-02_render-pdf-command-mit-pandoc-bereitstellen.md` |
| PER-0004 | REQ-0039 | REQ-US-4002-03 | US-4002 | `docs/req/REQ-US-4002-03_pandoc-defaults-dateien-unterstutzen.md` |
| PER-0004 | REQ-0040 | REQ-US-4002-04 | US-4002 | `docs/req/REQ-US-4002-04_eisvogel-als-template-option-unterstutzen.md` |
| PER-0004 | REQ-0041 | REQ-US-4002-05 | US-4002 | `docs/req/REQ-US-4002-05_render-artefakte-gema-artefacts-mode-ablegen.md` |
| PER-0004 | REQ-0042 | REQ-US-4002-06 | US-4002 | `docs/req/REQ-US-4002-06_pandoc-fehler-transparent-weitergeben.md` |
| PER-0004 | REQ-0043 | REQ-US-4002-07 | US-4002 | `docs/req/REQ-US-4002-07_lokale-render-konfiguration-pro-doc-package-erlauben.md` |
| PER-0004 | REQ-0044 | REQ-US-4002-08 | US-4002 | `docs/req/REQ-US-4002-08_html-deployment-fur-github-pages-unterstutzen.md` |
| PER-0005 | REQ-0031 | REQ-US-5001-01 | US-5001 | `docs/req/REQ-US-5001-01_volltextsuche-bereitstellen.md` |
| PER-0005 | REQ-0032 | REQ-US-5001-02 | US-5001 | `docs/req/REQ-US-5001-02_read-auf-doc-id-und-section-anchor-unterstutzen.md` |
| PER-0005 | REQ-0033 | REQ-US-5002-01 | US-5002 | `docs/req/REQ-US-5002-01_mcp-serve-uber-stdio-unterstutzen.md` |
| PER-0005 | REQ-0034 | REQ-US-5002-02 | US-5002 | `docs/req/REQ-US-5002-02_mcp-tools-search-read-list-anbieten.md` |
| PER-0006 | REQ-0046 | REQ-US-6001-01 | US-6001 | `docs/req/REQ-US-6001-01_adr-workflow-im-handbuch-dokumentieren.md` |
| PER-0006 | REQ-0047 | REQ-US-6001-02 | US-6001 | `docs/req/REQ-US-6001-02_adr-cli-commands-mit-guten-help-texten.md` |
| PER-0006 | REQ-0048 | REQ-US-6001-03 | US-6001 | `docs/req/REQ-US-6001-03_manpage-oder-manpage-generierbare-quelle-bereitstellen.md` |
| PER-0006 | REQ-0049 | REQ-US-6001-04 | US-6001 | `docs/req/REQ-US-6001-04_rustdoc-fur-offentliche-apis-pflegen.md` |
| PER-0006 | REQ-0050 | REQ-US-6001-05 | US-6001 | `docs/req/REQ-US-6001-05_doku-konsistenzcheck-spater-unterstutzen.md` |

## Added Draft Stories

- `PER-0001` -> `US-1012` `Configuration validation and effective config output` at `docs/us/US-1012_configuration-validation-and-effective-config-output.md`
- `PER-0001` -> `US-1013` `Schema-backed metadata contracts for document kinds` at `docs/us/US-1013_schema-backed-metadata-contracts-for-document-kinds.md`
- `PER-0005` -> `US-5003` `List documents as deterministic machine-readable catalog` at `docs/us/US-5003_list-documents-as-deterministic-machine-readable-catalog.md`
- `PER-0005` -> `US-5004` `Read structured document sections with stable selectors` at `docs/us/US-5004_read-structured-document-sections-with-stable-selectors.md`
- `PER-0006` -> `US-6002` `Glossary term scaffolding with cross-linkable IDs` at `docs/us/US-6002_glossary-term-scaffolding-with-cross-linkable-ids.md`
- `PER-0006` -> `US-6003` `Architecture narrative assembly with navigable outputs` at `docs/us/US-6003_architecture-narrative-assembly-with-navigable-outputs.md`
- `PER-0007` -> `US-7001` `Generate audit evidence bundles by requirement scope` at `docs/us/US-7001_generate-audit-evidence-bundles-by-requirement-scope.md`
- `PER-0007` -> `US-7002` `Filter traceability reports for audit review views` at `docs/us/US-7002_filter-traceability-reports-for-audit-review-views.md`
- `PER-0007` -> `US-7003` `Publish stable compliance-ready report exports` at `docs/us/US-7003_publish-stable-compliance-ready-report-exports.md`

## Ambiguous Items Requiring Human Review

- None. Primary persona assignment followed story actor text first, then persona-specific capability language.
