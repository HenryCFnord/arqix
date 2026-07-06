<!-- GENERATED SNAPSHOT — do not edit by hand.
     Question: Q-02 (see docs/en/reports/QUESTIONS.md)
     Snapshot: commit 2470321, 2026-07-05
     Regenerate: python3 scripts/arqix_report.py --snapshot "<sha>, <date>" -->

# Which tests verify which requirements?


| test | location | requirement | status |
| --- | --- | --- | --- |
| `assemble_build_fails_clearly_on_include_cycles` | tests/cli_assemble.rs:17 | REQ-02-01-11-03 | planned (ignored) |
| `assemble_build_generates_outputs_under_pages` | tests/cli_assemble.rs:8 | REQ-02-01-11-01 | planned (ignored) |
| `assemble_build_writes_a_jsonl_log` | tests/cli_assemble.rs:46 | REQ-04-01-01-02 | planned (ignored) |
| `config_show_renders_the_effective_configuration_as_json` | tests/cli_config.rs:49 | REQ-01-01-16-02 | active |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | tests/cli_config.rs:15 | REQ-01-01-16-01 | active |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | tests/cli_config.rs:16 | REQ-00-00-00-06 | active |
| `config_validate_accepts_a_valid_configuration` | tests/cli_config.rs:8 | REQ-01-01-16-01 | active |
| `config_validate_identifies_the_failing_key` | tests/cli_config.rs:28 | REQ-01-01-16-01 | active |
| `config_validate_identifies_the_failing_key` | tests/cli_config.rs:29 | REQ-01-01-16-03 | active |
| `doc_init_creates_the_standard_package_scaffold` | tests/cli_doc.rs:9 | REQ-01-01-01-01 | planned (ignored) |
| `doc_list_emits_a_json_document_catalog` | tests/cli_doc.rs:61 | REQ-05-01-08-01 | planned (ignored) |
| `doc_list_filters_the_catalog_by_kind` | tests/cli_doc.rs:74 | REQ-05-01-08-03 | planned (ignored) |
| `doc_new_creates_a_document_from_the_configured_template` | tests/cli_doc.rs:18 | REQ-00-00-00-05 | planned (ignored) |
| `doc_new_generates_a_unique_id_from_the_configured_policy` | tests/cli_doc.rs:30 | REQ-01-01-13-01 | planned (ignored) |
| `doc_new_writes_into_the_configured_kind_location` | tests/cli_doc.rs:44 | REQ-01-01-13-02 | planned (ignored) |
| `doc_read_distinguishes_a_document_miss_from_a_selector_miss` | tests/cli_doc.rs:98 | REQ-05-01-10-03 | planned (ignored) |
| `doc_read_retrieves_a_document_by_id` | tests/cli_doc.rs:85 | REQ-05-01-10-01 | planned (ignored) |
| `doc_search_finds_documents_by_full_text` | tests/cli_doc.rs:113 | REQ-02-01-06-01 | planned (ignored) |
| `finalise_fails_clearly_on_unsupported_frontmatter` | tests/cli_finalise.rs:43 | REQ-01-01-06-03 | planned (ignored) |
| `finalise_leaves_current_metadata_untouched` | tests/cli_finalise.rs:25 | REQ-01-01-06-02 | planned (ignored) |
| `finalise_sets_updated_to_the_injected_date` | tests/cli_finalise.rs:9 | REQ-01-01-06-01 | planned (ignored) |
| `fmt_is_idempotent` | tests/cli_fmt.rs:35 | REQ-00-00-00-01 | planned (ignored) |
| `fmt_never_changes_body_text` | tests/cli_fmt.rs:18 | REQ-01-01-03-02 | planned (ignored) |
| `fmt_never_changes_body_text` | tests/cli_fmt.rs:19 | REQ-01-01-03-03 | planned (ignored) |
| `fmt_sorts_frontmatter_keys_canonically` | tests/cli_fmt.rs:9 | REQ-01-01-03-01 | planned (ignored) |
| `format_option_is_accepted_globally` | tests/cli.rs:44 | REQ-04-01-10-01 | active |
| `lint_run_checks_that_include_targets_exist` | tests/cli_lint.rs:7 | REQ-01-01-04-01 | planned (ignored) |
| `lint_run_detects_translation_drift` | tests/cli_lint.rs:58 | REQ-00-00-00-10 | planned (ignored) |
| `lint_run_reports_duplicate_ids_globally` | tests/cli_lint.rs:15 | REQ-01-01-04-03 | planned (ignored) |
| `lint_run_reports_findings_with_file_and_line_context` | tests/cli_lint.rs:35 | REQ-01-01-04-04 | planned (ignored) |
| `mcp_serve_speaks_jsonrpc_over_stdio` | tests/cli_mcp.rs:9 | REQ-05-01-12-01 | planned (ignored) |
| `policy_check_evaluates_changed_files_against_the_declared_scope` | tests/cli_policy.rs:8 | REQ-01-01-07-02 | planned (ignored) |
| `policy_check_supports_warn_only_mode` | tests/cli_policy.rs:22 | REQ-01-01-07-03 | planned (ignored) |
| `publish_site_publishes_per_language` | tests/cli_publish.rs:9 | REQ-04-01-07-01 | planned (ignored) |
| `render_forwards_tool_errors_transparently` | tests/cli_publish.rs:29 | REQ-04-01-03-07 | planned (ignored) |
| `render_pdf_renders_via_pandoc` | tests/cli_publish.rs:18 | REQ-04-01-03-04 | planned (ignored) |
| `report_bundle_exports_an_evidence_bundle_by_id_scope` | tests/cli_report.rs:9 | REQ-03-01-04-01 | planned (ignored) |
| `report_bundle_includes_linked_evidence` | tests/cli_report.rs:21 | REQ-03-01-04-02 | planned (ignored) |
| `trace_check_reports_verifies_markers_per_requirement` | tests/cli_trace.rs:38 | REQ-03-01-06-02 | planned (ignored) |
| `trace_coverage_identifies_requirements_without_verifying_tests` | tests/cli_trace.rs:50 | REQ-01-01-08-01 | planned (ignored) |
| `trace_coverage_output_is_deterministic` | tests/cli_trace.rs:75 | REQ-00-00-00-01 | planned (ignored) |
| `trace_coverage_supports_json_output` | tests/cli_trace.rs:63 | REQ-01-01-08-03 | planned (ignored) |
| `trace_matrix_exports_csv` | tests/cli_trace.rs:93 | REQ-03-01-02-01 | planned (ignored) |
| `trace_scan_detects_markers_in_rust_comments` | tests/cli_trace.rs:9 | REQ-03-01-05-01 | planned (ignored) |
| `trace_scan_outputs_the_trace_graph_as_json` | tests/cli_trace.rs:25 | REQ-03-01-05-04 | planned (ignored) |
| `unit_new_creates_a_unit_from_the_configured_template` | tests/cli_unit.rs:8 | REQ-00-00-00-05 | planned (ignored) |
| `usage_error_exits_with_code_2` | tests/cli.rs:30 | REQ-00-00-00-02 | active |
| `verify_emits_per_step_results_in_json_mode` | tests/cli_verify.rs:29 | REQ-04-01-05-03 | planned (ignored) |
| `verify_excludes_rendering_from_the_default_loop` | tests/cli_verify.rs:41 | REQ-04-01-05-04 | planned (ignored) |
| `verify_runs_the_configured_sub_steps` | tests/cli_verify.rs:9 | REQ-04-01-05-01 | planned (ignored) |
| `verify_supports_fail_fast_and_aggregate_modes` | tests/cli_verify.rs:18 | REQ-04-01-05-02 | planned (ignored) |
