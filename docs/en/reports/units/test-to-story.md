<!-- GENERATED SNAPSHOT — do not edit by hand.
     Question: Q-05 (see docs/en/reports/QUESTIONS.md)
     Snapshot: c5aa003, 2026-07-08
     Regenerate: python3 scripts/arqix_report.py --snapshot "<sha>, <date>" -->

# Which user story belongs to which integration test?

Joined test → requirement (`verifies`) → story (`derived-from`).

| test | story | title |
| --- | --- | --- |
| `assemble_build_fails_clearly_on_include_cycles` | US-02-01-11 | Assemble Documentation During Implementation |
| `assemble_build_generates_outputs_under_pages` | US-02-01-11 | Assemble Documentation During Implementation |
| `assemble_build_writes_a_jsonl_log` | US-04-01-01 | Emit a CI-Friendly Assembly Log |
| `assemble_build_writes_a_jsonl_log` | US-05-01-02 | Emit a Machine-Readable Assembly Log |
| `assemble_build_writes_a_jsonl_log` | US-06-01-02 | Trace Document Assembly Structure |
| `assemble_build_writes_a_jsonl_log` | US-08-01-02 | Emit a Deterministic Assembly Log for Verification |
| `config_show_renders_the_effective_configuration_as_json` | US-01-01-16 | Validate Repository Configuration and Inspect Effective Config |
| `config_show_renders_the_effective_configuration_as_json` | US-04-01-11 | Inspect Effective Config for CI Reproducibility |
| `config_show_renders_the_effective_configuration_as_json` | US-05-01-11 | Consume Effective Configuration as Automation Baseline |
| `config_show_renders_the_effective_configuration_as_json` | US-08-01-20 | Read Effective Config Deterministically Before Execution |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-01-01-01 | Initialise Standardised Doc Package |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-01-01-02 | Create Governed Units |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-01-01-03 | Format Documents Canonically |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-01-01-04 | Lint Documents Deterministically |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-01-01-05 | Create Documents from Configured Templates |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-01-01-13 | Govern Deterministic Document Creation via Templates |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-01-01-14 | Lint Translation Metadata and Drift |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-01-01-16 | Validate Repository Configuration and Inspect Effective Config |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-02-01-02 | Create Units Quickly During Implementation |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-02-01-03 | Format Documents During Implementation |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-02-01-04 | Lint Documents Before Commit |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-02-01-05 | Create Documents Quickly from Templates |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-02-01-07 | Create Conforming Documents Quickly via Templates |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-02-01-09 | Use Chapter and Include Directives During Implementation |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-02-01-11 | Assemble Documentation During Implementation |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-03-01-01 | Lint Documents for Traceability Gaps |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-03-01-05 | Scan Traceability Information |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-03-01-08 | Make Trace and Coverage Outputs Reproducible |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-04-01-01 | Emit a CI-Friendly Assembly Log |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-04-01-03 | Generate Publishing Outputs |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-04-01-04 | Gate Bilingual Documentation Quality in CI |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-04-01-05 | Run a One-Command Verification Loop |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-04-01-06 | Build Deterministic Page Artefacts in CI |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-04-01-07 | Publish Language-Aware Sites |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-04-01-11 | Inspect Effective Config for CI Reproducibility |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-05-01-01 | Create Units for Retrieval and Automation |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-05-01-02 | Emit a Machine-Readable Assembly Log |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-05-01-04 | Parse Document Structure Deterministically for Automation |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-05-01-05 | Detect Translation Drift for Automation |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-05-01-07 | Build Machine-Readable Trace Graphs |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-05-01-09 | Observe Assembled Outputs for Downstream Tooling |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-05-01-11 | Consume Effective Configuration as Automation Baseline |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-05-01-13 | Expose Language-Aware Site Outputs Deterministically |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-06-01-01 | Create Modular Document Units |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-06-01-02 | Trace Document Assembly Structure |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-06-01-03 | Create Architecture Documents from Templates |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-06-01-04 | Compose Modular Documents with Chapter and Include Directives |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-06-01-05 | Generate Publishable Documentation Outputs |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-06-01-08 | Assemble Modular Document Packages into Pages |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-07-01-04 | Review Trace Graphs as Audit Evidence |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-07-01-06 | Export Deterministic Trace and Coverage Evidence |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-08-01-01 | Initialize a Doc Package Deterministically and Safely |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-08-01-02 | Emit a Deterministic Assembly Log for Verification |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-08-01-03 | Format Documents Deterministically within Scope |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-08-01-04 | Lint Documents with Actionable Diagnostics |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-08-01-05 | Create Documents Deterministically from Templates |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-08-01-11 | Interpret i18n Lint Results Deterministically |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-08-01-13 | Run One-Command Verification in Agent Workflows |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-08-01-16 | Scan Traceability Deterministically within Verification Loops |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-08-01-20 | Read Effective Config Deterministically Before Execution |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-08-01-22 | Make Trace and Coverage Outputs Deterministic |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-08-01-23 | Create Documents without Ambiguity via Templates |
| `config_validate_accepts_a_valid_configuration` | US-01-01-16 | Validate Repository Configuration and Inspect Effective Config |
| `config_validate_accepts_a_valid_configuration` | US-04-01-11 | Inspect Effective Config for CI Reproducibility |
| `config_validate_accepts_a_valid_configuration` | US-05-01-11 | Consume Effective Configuration as Automation Baseline |
| `config_validate_accepts_a_valid_configuration` | US-08-01-20 | Read Effective Config Deterministically Before Execution |
| `config_validate_identifies_the_failing_key` | US-01-01-16 | Validate Repository Configuration and Inspect Effective Config |
| `config_validate_identifies_the_failing_key` | US-04-01-11 | Inspect Effective Config for CI Reproducibility |
| `config_validate_identifies_the_failing_key` | US-05-01-11 | Consume Effective Configuration as Automation Baseline |
| `config_validate_identifies_the_failing_key` | US-08-01-20 | Read Effective Config Deterministically Before Execution |
| `doc_init_creates_the_standard_package_scaffold` | US-01-01-01 | Initialise Standardised Doc Package |
| `doc_init_creates_the_standard_package_scaffold` | US-02-01-01 | Initialize a Doc Package with One Command |
| `doc_list_emits_a_json_document_catalog` | US-05-01-08 | Export a Deterministic Document Catalog |
| `doc_list_emits_a_json_document_catalog` | US-08-01-07 | List Documents Deterministically for Automation |
| `doc_list_filters_the_catalog_by_kind` | US-05-01-08 | Export a Deterministic Document Catalog |
| `doc_list_filters_the_catalog_by_kind` | US-08-01-07 | List Documents Deterministically for Automation |
| `doc_new_creates_a_document_from_the_configured_template` | US-01-01-05 | Create Documents from Configured Templates |
| `doc_new_creates_a_document_from_the_configured_template` | US-01-01-10 | Define Schema-Backed Metadata Contracts |
| `doc_new_creates_a_document_from_the_configured_template` | US-01-01-13 | Govern Deterministic Document Creation via Templates |
| `doc_new_creates_a_document_from_the_configured_template` | US-02-01-05 | Create Documents Quickly from Templates |
| `doc_new_creates_a_document_from_the_configured_template` | US-02-01-07 | Create Conforming Documents Quickly via Templates |
| `doc_new_creates_a_document_from_the_configured_template` | US-05-01-03 | Expose Machine-Usable Metadata Contracts |
| `doc_new_creates_a_document_from_the_configured_template` | US-06-01-03 | Create Architecture Documents from Templates |
| `doc_new_creates_a_document_from_the_configured_template` | US-08-01-05 | Create Documents Deterministically from Templates |
| `doc_new_creates_a_document_from_the_configured_template` | US-08-01-10 | Use Metadata Contracts Deterministically |
| `doc_new_creates_a_document_from_the_configured_template` | US-08-01-23 | Create Documents without Ambiguity via Templates |
| `doc_new_generates_a_unique_id_from_the_configured_policy` | US-01-01-13 | Govern Deterministic Document Creation via Templates |
| `doc_new_generates_a_unique_id_from_the_configured_policy` | US-02-01-07 | Create Conforming Documents Quickly via Templates |
| `doc_new_generates_a_unique_id_from_the_configured_policy` | US-08-01-23 | Create Documents without Ambiguity via Templates |
| `doc_new_writes_into_the_configured_kind_location` | US-01-01-13 | Govern Deterministic Document Creation via Templates |
| `doc_new_writes_into_the_configured_kind_location` | US-02-01-07 | Create Conforming Documents Quickly via Templates |
| `doc_new_writes_into_the_configured_kind_location` | US-08-01-23 | Create Documents without Ambiguity via Templates |
| `doc_read_distinguishes_a_document_miss_from_a_selector_miss` | US-05-01-10 | Read Structured Document Sections with Stable Selectors |
| `doc_read_distinguishes_a_document_miss_from_a_selector_miss` | US-08-01-09 | Read Precise Document Sections for Scoped Execution |
| `doc_read_retrieves_a_document_by_id` | US-05-01-10 | Read Structured Document Sections with Stable Selectors |
| `doc_read_retrieves_a_document_by_id` | US-08-01-09 | Read Precise Document Sections for Scoped Execution |
| `doc_search_finds_documents_by_full_text` | US-02-01-06 | Find and Read Documentation During Implementation |
| `finalise_fails_clearly_on_unsupported_frontmatter` | US-01-01-06 | Finalise Document Metadata Mechanically |
| `finalise_fails_clearly_on_unsupported_frontmatter` | US-02-01-08 | Finalise Metadata without Touching Content |
| `finalise_fails_clearly_on_unsupported_frontmatter` | US-08-01-06 | Finalise Metadata Safely and Deterministically |
| `finalise_leaves_current_metadata_untouched` | US-01-01-06 | Finalise Document Metadata Mechanically |
| `finalise_leaves_current_metadata_untouched` | US-02-01-08 | Finalise Metadata without Touching Content |
| `finalise_leaves_current_metadata_untouched` | US-08-01-06 | Finalise Metadata Safely and Deterministically |
| `finalise_sets_updated_to_the_injected_date` | US-01-01-06 | Finalise Document Metadata Mechanically |
| `finalise_sets_updated_to_the_injected_date` | US-02-01-08 | Finalise Metadata without Touching Content |
| `finalise_sets_updated_to_the_injected_date` | US-08-01-06 | Finalise Metadata Safely and Deterministically |
| `fmt_is_idempotent` | US-01-01-03 | Format Documents Canonically |
| `fmt_is_idempotent` | US-01-01-04 | Lint Documents Deterministically |
| `fmt_is_idempotent` | US-01-01-08 | Generate Governed Coverage Reports |
| `fmt_is_idempotent` | US-01-01-10 | Define Schema-Backed Metadata Contracts |
| `fmt_is_idempotent` | US-01-01-12 | Govern Glossary Term Metadata and IDs |
| `fmt_is_idempotent` | US-01-01-16 | Validate Repository Configuration and Inspect Effective Config |
| `fmt_is_idempotent` | US-02-01-03 | Format Documents During Implementation |
| `fmt_is_idempotent` | US-02-01-04 | Lint Documents Before Commit |
| `fmt_is_idempotent` | US-02-01-06 | Find and Read Documentation During Implementation |
| `fmt_is_idempotent` | US-02-01-09 | Use Chapter and Include Directives During Implementation |
| `fmt_is_idempotent` | US-02-01-11 | Assemble Documentation During Implementation |
| `fmt_is_idempotent` | US-03-01-01 | Lint Documents for Traceability Gaps |
| `fmt_is_idempotent` | US-03-01-02 | Export Trace Matrices |
| `fmt_is_idempotent` | US-03-01-03 | Generate Coverage Reports |
| `fmt_is_idempotent` | US-03-01-04 | Export Scoped Evidence Bundles for Quality Review |
| `fmt_is_idempotent` | US-03-01-07 | Filter Traceability Reports for Quality Analysis |
| `fmt_is_idempotent` | US-03-01-08 | Make Trace and Coverage Outputs Reproducible |
| `fmt_is_idempotent` | US-04-01-06 | Build Deterministic Page Artefacts in CI |
| `fmt_is_idempotent` | US-04-01-10 | Emit Machine-Readable Diagnostics for CI |
| `fmt_is_idempotent` | US-04-01-11 | Inspect Effective Config for CI Reproducibility |
| `fmt_is_idempotent` | US-05-01-03 | Expose Machine-Usable Metadata Contracts |
| `fmt_is_idempotent` | US-05-01-04 | Parse Document Structure Deterministically for Automation |
| `fmt_is_idempotent` | US-05-01-06 | Search and Read Documentation via CLI |
| `fmt_is_idempotent` | US-05-01-08 | Export a Deterministic Document Catalog |
| `fmt_is_idempotent` | US-05-01-09 | Observe Assembled Outputs for Downstream Tooling |
| `fmt_is_idempotent` | US-05-01-10 | Read Structured Document Sections with Stable Selectors |
| `fmt_is_idempotent` | US-05-01-11 | Consume Effective Configuration as Automation Baseline |
| `fmt_is_idempotent` | US-05-01-14 | Emit Machine-Readable Diagnostics for Downstream Tooling |
| `fmt_is_idempotent` | US-06-01-04 | Compose Modular Documents with Chapter and Include Directives |
| `fmt_is_idempotent` | US-06-01-08 | Assemble Modular Document Packages into Pages |
| `fmt_is_idempotent` | US-06-01-09 | Retrieve Architecture Documentation Quickly |
| `fmt_is_idempotent` | US-06-01-10 | Create Glossary Terms with Stable IDs |
| `fmt_is_idempotent` | US-07-01-01 | Review Coverage Evidence |
| `fmt_is_idempotent` | US-07-01-02 | Review Evidence Chains through Trace Matrices |
| `fmt_is_idempotent` | US-07-01-03 | Generate Audit Evidence Bundles by Scope |
| `fmt_is_idempotent` | US-07-01-05 | Filter Traceability Reports for Audit Review |
| `fmt_is_idempotent` | US-07-01-06 | Export Deterministic Trace and Coverage Evidence |
| `fmt_is_idempotent` | US-08-01-01 | Initialize a Doc Package Deterministically and Safely |
| `fmt_is_idempotent` | US-08-01-03 | Format Documents Deterministically within Scope |
| `fmt_is_idempotent` | US-08-01-04 | Lint Documents with Actionable Diagnostics |
| `fmt_is_idempotent` | US-08-01-06 | Finalise Metadata Safely and Deterministically |
| `fmt_is_idempotent` | US-08-01-07 | List Documents Deterministically for Automation |
| `fmt_is_idempotent` | US-08-01-09 | Read Precise Document Sections for Scoped Execution |
| `fmt_is_idempotent` | US-08-01-10 | Use Metadata Contracts Deterministically |
| `fmt_is_idempotent` | US-08-01-20 | Read Effective Config Deterministically Before Execution |
| `fmt_is_idempotent` | US-08-01-21 | Emit Machine-Readable Diagnostics for Agent Workflows |
| `fmt_is_idempotent` | US-08-01-22 | Make Trace and Coverage Outputs Deterministic |
| `fmt_never_changes_body_text` | US-01-01-03 | Format Documents Canonically |
| `fmt_never_changes_body_text` | US-02-01-03 | Format Documents During Implementation |
| `fmt_never_changes_body_text` | US-08-01-03 | Format Documents Deterministically within Scope |
| `fmt_sorts_frontmatter_keys_canonically` | US-01-01-03 | Format Documents Canonically |
| `fmt_sorts_frontmatter_keys_canonically` | US-02-01-03 | Format Documents During Implementation |
| `fmt_sorts_frontmatter_keys_canonically` | US-08-01-03 | Format Documents Deterministically within Scope |
| `format_option_is_accepted_globally` | US-04-01-10 | Emit Machine-Readable Diagnostics for CI |
| `format_option_is_accepted_globally` | US-05-01-14 | Emit Machine-Readable Diagnostics for Downstream Tooling |
| `format_option_is_accepted_globally` | US-08-01-21 | Emit Machine-Readable Diagnostics for Agent Workflows |
| `lint_run_checks_that_include_targets_exist` | US-01-01-04 | Lint Documents Deterministically |
| `lint_run_checks_that_include_targets_exist` | US-02-01-04 | Lint Documents Before Commit |
| `lint_run_checks_that_include_targets_exist` | US-03-01-01 | Lint Documents for Traceability Gaps |
| `lint_run_checks_that_include_targets_exist` | US-08-01-04 | Lint Documents with Actionable Diagnostics |
| `lint_run_detects_translation_drift` | US-01-01-14 | Lint Translation Metadata and Drift |
| `lint_run_detects_translation_drift` | US-04-01-04 | Gate Bilingual Documentation Quality in CI |
| `lint_run_detects_translation_drift` | US-05-01-05 | Detect Translation Drift for Automation |
| `lint_run_detects_translation_drift` | US-08-01-11 | Interpret i18n Lint Results Deterministically |
| `lint_run_reports_duplicate_ids_globally` | US-01-01-04 | Lint Documents Deterministically |
| `lint_run_reports_duplicate_ids_globally` | US-02-01-04 | Lint Documents Before Commit |
| `lint_run_reports_duplicate_ids_globally` | US-03-01-01 | Lint Documents for Traceability Gaps |
| `lint_run_reports_duplicate_ids_globally` | US-08-01-04 | Lint Documents with Actionable Diagnostics |
| `lint_run_reports_findings_with_file_and_line_context` | US-01-01-04 | Lint Documents Deterministically |
| `lint_run_reports_findings_with_file_and_line_context` | US-02-01-04 | Lint Documents Before Commit |
| `lint_run_reports_findings_with_file_and_line_context` | US-03-01-01 | Lint Documents for Traceability Gaps |
| `lint_run_reports_findings_with_file_and_line_context` | US-08-01-04 | Lint Documents with Actionable Diagnostics |
| `mcp_serve_speaks_jsonrpc_over_stdio` | US-05-01-12 | Expose Arqix via MCP over STDIO |
| `mcp_serve_speaks_jsonrpc_over_stdio` | US-08-01-12 | Use MCP Tools Deterministically in Agent Workflows |
| `policy_check_evaluates_changed_files_against_the_declared_scope` | US-01-01-07 | Enforce Scope Guardrails for Automation Agents |
| `policy_check_evaluates_changed_files_against_the_declared_scope` | US-04-01-02 | Check Scope Guardrails in CI |
| `policy_check_evaluates_changed_files_against_the_declared_scope` | US-08-01-08 | Stay within Declared Change Scope |
| `policy_check_supports_warn_only_mode` | US-01-01-07 | Enforce Scope Guardrails for Automation Agents |
| `policy_check_supports_warn_only_mode` | US-04-01-02 | Check Scope Guardrails in CI |
| `policy_check_supports_warn_only_mode` | US-08-01-08 | Stay within Declared Change Scope |
| `publish_site_publishes_per_language` | US-04-01-07 | Publish Language-Aware Sites |
| `publish_site_publishes_per_language` | US-05-01-13 | Expose Language-Aware Site Outputs Deterministically |
| `render_forwards_tool_errors_transparently` | US-04-01-03 | Generate Publishing Outputs |
| `render_forwards_tool_errors_transparently` | US-06-01-05 | Generate Publishable Documentation Outputs |
| `render_pdf_renders_via_pandoc` | US-04-01-03 | Generate Publishing Outputs |
| `render_pdf_renders_via_pandoc` | US-06-01-05 | Generate Publishable Documentation Outputs |
| `report_bundle_exports_an_evidence_bundle_by_id_scope` | US-03-01-04 | Export Scoped Evidence Bundles for Quality Review |
| `report_bundle_includes_linked_evidence` | US-03-01-04 | Export Scoped Evidence Bundles for Quality Review |
| `trace_check_reports_verifies_markers_per_requirement` | US-03-01-06 | Detect Missing Trace Markers for Quality Gaps |
| `trace_coverage_identifies_requirements_without_verifying_tests` | US-01-01-08 | Generate Governed Coverage Reports |
| `trace_coverage_identifies_requirements_without_verifying_tests` | US-03-01-03 | Generate Coverage Reports |
| `trace_coverage_identifies_requirements_without_verifying_tests` | US-07-01-01 | Review Coverage Evidence |
| `trace_coverage_output_is_deterministic` | US-01-01-03 | Format Documents Canonically |
| `trace_coverage_output_is_deterministic` | US-01-01-04 | Lint Documents Deterministically |
| `trace_coverage_output_is_deterministic` | US-01-01-08 | Generate Governed Coverage Reports |
| `trace_coverage_output_is_deterministic` | US-01-01-10 | Define Schema-Backed Metadata Contracts |
| `trace_coverage_output_is_deterministic` | US-01-01-12 | Govern Glossary Term Metadata and IDs |
| `trace_coverage_output_is_deterministic` | US-01-01-16 | Validate Repository Configuration and Inspect Effective Config |
| `trace_coverage_output_is_deterministic` | US-02-01-03 | Format Documents During Implementation |
| `trace_coverage_output_is_deterministic` | US-02-01-04 | Lint Documents Before Commit |
| `trace_coverage_output_is_deterministic` | US-02-01-06 | Find and Read Documentation During Implementation |
| `trace_coverage_output_is_deterministic` | US-02-01-09 | Use Chapter and Include Directives During Implementation |
| `trace_coverage_output_is_deterministic` | US-02-01-11 | Assemble Documentation During Implementation |
| `trace_coverage_output_is_deterministic` | US-03-01-01 | Lint Documents for Traceability Gaps |
| `trace_coverage_output_is_deterministic` | US-03-01-02 | Export Trace Matrices |
| `trace_coverage_output_is_deterministic` | US-03-01-03 | Generate Coverage Reports |
| `trace_coverage_output_is_deterministic` | US-03-01-04 | Export Scoped Evidence Bundles for Quality Review |
| `trace_coverage_output_is_deterministic` | US-03-01-07 | Filter Traceability Reports for Quality Analysis |
| `trace_coverage_output_is_deterministic` | US-03-01-08 | Make Trace and Coverage Outputs Reproducible |
| `trace_coverage_output_is_deterministic` | US-04-01-06 | Build Deterministic Page Artefacts in CI |
| `trace_coverage_output_is_deterministic` | US-04-01-10 | Emit Machine-Readable Diagnostics for CI |
| `trace_coverage_output_is_deterministic` | US-04-01-11 | Inspect Effective Config for CI Reproducibility |
| `trace_coverage_output_is_deterministic` | US-05-01-03 | Expose Machine-Usable Metadata Contracts |
| `trace_coverage_output_is_deterministic` | US-05-01-04 | Parse Document Structure Deterministically for Automation |
| `trace_coverage_output_is_deterministic` | US-05-01-06 | Search and Read Documentation via CLI |
| `trace_coverage_output_is_deterministic` | US-05-01-08 | Export a Deterministic Document Catalog |
| `trace_coverage_output_is_deterministic` | US-05-01-09 | Observe Assembled Outputs for Downstream Tooling |
| `trace_coverage_output_is_deterministic` | US-05-01-10 | Read Structured Document Sections with Stable Selectors |
| `trace_coverage_output_is_deterministic` | US-05-01-11 | Consume Effective Configuration as Automation Baseline |
| `trace_coverage_output_is_deterministic` | US-05-01-14 | Emit Machine-Readable Diagnostics for Downstream Tooling |
| `trace_coverage_output_is_deterministic` | US-06-01-04 | Compose Modular Documents with Chapter and Include Directives |
| `trace_coverage_output_is_deterministic` | US-06-01-08 | Assemble Modular Document Packages into Pages |
| `trace_coverage_output_is_deterministic` | US-06-01-09 | Retrieve Architecture Documentation Quickly |
| `trace_coverage_output_is_deterministic` | US-06-01-10 | Create Glossary Terms with Stable IDs |
| `trace_coverage_output_is_deterministic` | US-07-01-01 | Review Coverage Evidence |
| `trace_coverage_output_is_deterministic` | US-07-01-02 | Review Evidence Chains through Trace Matrices |
| `trace_coverage_output_is_deterministic` | US-07-01-03 | Generate Audit Evidence Bundles by Scope |
| `trace_coverage_output_is_deterministic` | US-07-01-05 | Filter Traceability Reports for Audit Review |
| `trace_coverage_output_is_deterministic` | US-07-01-06 | Export Deterministic Trace and Coverage Evidence |
| `trace_coverage_output_is_deterministic` | US-08-01-01 | Initialize a Doc Package Deterministically and Safely |
| `trace_coverage_output_is_deterministic` | US-08-01-03 | Format Documents Deterministically within Scope |
| `trace_coverage_output_is_deterministic` | US-08-01-04 | Lint Documents with Actionable Diagnostics |
| `trace_coverage_output_is_deterministic` | US-08-01-06 | Finalise Metadata Safely and Deterministically |
| `trace_coverage_output_is_deterministic` | US-08-01-07 | List Documents Deterministically for Automation |
| `trace_coverage_output_is_deterministic` | US-08-01-09 | Read Precise Document Sections for Scoped Execution |
| `trace_coverage_output_is_deterministic` | US-08-01-10 | Use Metadata Contracts Deterministically |
| `trace_coverage_output_is_deterministic` | US-08-01-20 | Read Effective Config Deterministically Before Execution |
| `trace_coverage_output_is_deterministic` | US-08-01-21 | Emit Machine-Readable Diagnostics for Agent Workflows |
| `trace_coverage_output_is_deterministic` | US-08-01-22 | Make Trace and Coverage Outputs Deterministic |
| `trace_coverage_supports_json_output` | US-01-01-08 | Generate Governed Coverage Reports |
| `trace_coverage_supports_json_output` | US-03-01-03 | Generate Coverage Reports |
| `trace_coverage_supports_json_output` | US-07-01-01 | Review Coverage Evidence |
| `trace_matrix_exports_csv` | US-03-01-02 | Export Trace Matrices |
| `trace_scan_detects_markers_in_rust_comments` | US-03-01-05 | Scan Traceability Information |
| `trace_scan_outputs_the_trace_graph_as_json` | US-03-01-05 | Scan Traceability Information |
| `unit_new_creates_a_unit_from_the_configured_template` | US-01-01-05 | Create Documents from Configured Templates |
| `unit_new_creates_a_unit_from_the_configured_template` | US-01-01-10 | Define Schema-Backed Metadata Contracts |
| `unit_new_creates_a_unit_from_the_configured_template` | US-01-01-13 | Govern Deterministic Document Creation via Templates |
| `unit_new_creates_a_unit_from_the_configured_template` | US-02-01-05 | Create Documents Quickly from Templates |
| `unit_new_creates_a_unit_from_the_configured_template` | US-02-01-07 | Create Conforming Documents Quickly via Templates |
| `unit_new_creates_a_unit_from_the_configured_template` | US-05-01-03 | Expose Machine-Usable Metadata Contracts |
| `unit_new_creates_a_unit_from_the_configured_template` | US-06-01-03 | Create Architecture Documents from Templates |
| `unit_new_creates_a_unit_from_the_configured_template` | US-08-01-05 | Create Documents Deterministically from Templates |
| `unit_new_creates_a_unit_from_the_configured_template` | US-08-01-10 | Use Metadata Contracts Deterministically |
| `unit_new_creates_a_unit_from_the_configured_template` | US-08-01-23 | Create Documents without Ambiguity via Templates |
| `usage_error_exits_with_code_2` | US-01-01-14 | Lint Translation Metadata and Drift |
| `usage_error_exits_with_code_2` | US-04-01-04 | Gate Bilingual Documentation Quality in CI |
| `usage_error_exits_with_code_2` | US-04-01-05 | Run a One-Command Verification Loop |
| `usage_error_exits_with_code_2` | US-04-01-07 | Publish Language-Aware Sites |
| `usage_error_exits_with_code_2` | US-04-01-08 | Provide Consistent Exit Codes for CI Gates |
| `usage_error_exits_with_code_2` | US-04-01-10 | Emit Machine-Readable Diagnostics for CI |
| `usage_error_exits_with_code_2` | US-05-01-05 | Detect Translation Drift for Automation |
| `usage_error_exits_with_code_2` | US-05-01-13 | Expose Language-Aware Site Outputs Deterministically |
| `usage_error_exits_with_code_2` | US-05-01-14 | Emit Machine-Readable Diagnostics for Downstream Tooling |
| `usage_error_exits_with_code_2` | US-08-01-11 | Interpret i18n Lint Results Deterministically |
| `usage_error_exits_with_code_2` | US-08-01-13 | Run One-Command Verification in Agent Workflows |
| `usage_error_exits_with_code_2` | US-08-01-15 | Interpret Verification Outcomes through Stable Exit Codes |
| `usage_error_exits_with_code_2` | US-08-01-21 | Emit Machine-Readable Diagnostics for Agent Workflows |
| `verify_emits_per_step_results_in_json_mode` | US-04-01-05 | Run a One-Command Verification Loop |
| `verify_emits_per_step_results_in_json_mode` | US-08-01-13 | Run One-Command Verification in Agent Workflows |
| `verify_excludes_rendering_from_the_default_loop` | US-04-01-05 | Run a One-Command Verification Loop |
| `verify_excludes_rendering_from_the_default_loop` | US-08-01-13 | Run One-Command Verification in Agent Workflows |
| `verify_runs_the_configured_sub_steps` | US-04-01-05 | Run a One-Command Verification Loop |
| `verify_runs_the_configured_sub_steps` | US-08-01-13 | Run One-Command Verification in Agent Workflows |
| `verify_supports_fail_fast_and_aggregate_modes` | US-04-01-05 | Run a One-Command Verification Loop |
| `verify_supports_fail_fast_and_aggregate_modes` | US-08-01-13 | Run One-Command Verification in Agent Workflows |
