<!-- GENERATED SNAPSHOT — do not edit by hand.
     Question: Q-02 (see docs/en/reports/QUESTIONS.md)
     Snapshot: c8598c5, 2026-07-11
     Regenerate: python3 scripts/arqix_report.py --snapshot "<sha>, <date>" -->

# Which tests verify which requirements?

| test | location | requirement | status |
| --- | --- | --- | --- |
| `assemble_build_fails_clearly_on_include_cycles` | tests/cli_assemble.rs:16 | REQ-02-01-11-03 | active |
| `assemble_build_fails_on_output_collisions_across_roots` | tests/cli_assemble.rs:44 | REQ-02-01-11-01 | active |
| `assemble_build_generates_outputs_under_pages` | tests/cli_assemble.rs:8 | REQ-02-01-11-01 | active |
| `assemble_build_refuses_includes_outside_the_repository` | tests/cli_assemble.rs:74 | REQ-00-00-00-13 | active |
| `assemble_build_writes_a_jsonl_log` | tests/cli_assemble.rs:105 | REQ-04-01-01-02 | active |
| `config_show_renders_the_effective_configuration_as_json` | tests/cli_config.rs:49 | REQ-01-01-16-02 | active |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | tests/cli_config.rs:15 | REQ-01-01-16-01 | active |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | tests/cli_config.rs:16 | REQ-00-00-00-06 | active |
| `config_validate_accepts_a_valid_configuration` | tests/cli_config.rs:8 | REQ-01-01-16-01 | active |
| `config_validate_identifies_the_failing_key` | tests/cli_config.rs:28 | REQ-01-01-16-01 | active |
| `config_validate_identifies_the_failing_key` | tests/cli_config.rs:29 | REQ-01-01-16-03 | active |
| `doc_init_creates_the_standard_package_scaffold` | tests/cli_doc.rs:9 | REQ-01-01-01-01 | active |
| `doc_init_scaffolds_an_explicit_path` | tests/cli_doc.rs:38 | REQ-01-01-01-01 | active |
| `doc_init_writes_doc_index_frontmatter` | tests/cli_doc.rs:49 | REQ-01-01-01-02 | active |
| `doc_list_does_not_follow_directory_symlinks` | tests/cli_doc.rs:280 | REQ-00-00-00-01 | active |
| `doc_list_emits_a_json_document_catalog` | tests/cli_doc.rs:198 | REQ-05-01-08-01 | active |
| `doc_list_filters_the_catalog_by_kind` | tests/cli_doc.rs:210 | REQ-05-01-08-03 | active |
| `doc_list_honours_configured_skip_dirs` | tests/cli_doc.rs:258 | REQ-01-01-17-01 | active |
| `doc_list_lists_each_document_once_under_overlapping_roots` | tests/cli_doc.rs:303 | REQ-05-01-08-01 | active |
| `doc_list_skips_the_default_directories_without_an_override` | tests/cli_doc.rs:338 | REQ-01-01-17-02 | active |
| `doc_new_accepts_an_explicit_id_and_rejects_a_duplicate` | tests/cli_doc.rs:154 | REQ-01-01-13-01 | active |
| `doc_new_creates_a_document_from_the_configured_template` | tests/cli_doc.rs:83 | REQ-00-00-00-05 | active |
| `doc_new_dry_run_reports_the_plan_without_writing` | tests/cli_doc.rs:123 | REQ-00-00-00-09 | active |
| `doc_new_generates_a_unique_id_from_the_configured_policy` | tests/cli_doc.rs:94 | REQ-01-01-13-01 | active |
| `doc_new_rejects_a_kind_that_escapes_the_root` | tests/cli_doc.rs:67 | REQ-00-00-00-13 | active |
| `doc_new_substitutes_the_title_into_the_template` | tests/cli_doc.rs:174 | REQ-00-00-00-05 | active |
| `doc_new_writes_into_the_configured_kind_location` | tests/cli_doc.rs:107 | REQ-01-01-13-02 | active |
| `doc_read_distinguishes_a_document_miss_from_a_selector_miss` | tests/cli_doc.rs:232 | REQ-05-01-10-03 | active |
| `doc_read_retrieves_a_document_by_id` | tests/cli_doc.rs:220 | REQ-05-01-10-01 | active |
| `doc_search_finds_documents_by_full_text` | tests/cli_doc.rs:246 | REQ-02-01-06-01 | active |
| `finalise_fails_clearly_on_unsupported_frontmatter` | tests/cli_finalise.rs:90 | REQ-01-01-06-03 | active |
| `finalise_leaves_current_metadata_untouched` | tests/cli_finalise.rs:24 | REQ-01-01-06-02 | active |
| `finalise_rejects_a_non_iso_date` | tests/cli_finalise.rs:41 | REQ-01-01-06-01 | active |
| `finalise_sets_updated_to_the_injected_date` | tests/cli_finalise.rs:9 | REQ-01-01-06-01 | active |
| `finalise_touches_only_the_meta_updated_field` | tests/cli_finalise.rs:65 | REQ-01-01-06-01 | active |
| `fmt_is_idempotent` | tests/cli_fmt.rs:62 | REQ-00-00-00-01 | active |
| `fmt_never_changes_body_text` | tests/cli_fmt.rs:17 | REQ-01-01-03-02 | active |
| `fmt_never_changes_body_text` | tests/cli_fmt.rs:18 | REQ-01-01-03-03 | active |
| `fmt_orders_ontology_frontmatter_by_family` | tests/cli_fmt.rs:33 | REQ-01-01-03-01 | active |
| `fmt_sorts_frontmatter_keys_canonically` | tests/cli_fmt.rs:9 | REQ-01-01-03-01 | active |
| `format_option_is_accepted_globally` | tests/cli.rs:44 | REQ-04-01-10-01 | active |
| `lint_run_checks_that_include_targets_exist` | tests/cli_lint.rs:7 | REQ-01-01-04-01 | active |
| `lint_run_detects_translation_drift` | tests/cli_lint.rs:55 | REQ-00-00-00-10 | active |
| `lint_run_reports_duplicate_ids_globally` | tests/cli_lint.rs:14 | REQ-01-01-04-03 | active |
| `lint_run_reports_findings_with_file_and_line_context` | tests/cli_lint.rs:33 | REQ-01-01-04-04 | active |
| `mcp_serve_speaks_jsonrpc_over_stdio` | tests/cli_mcp.rs:9 | REQ-05-01-12-01 | planned (ignored) |
| `policy_check_evaluates_changed_files_against_the_declared_scope` | tests/cli_policy.rs:8 | REQ-01-01-07-02 | planned (ignored) |
| `policy_check_supports_warn_only_mode` | tests/cli_policy.rs:22 | REQ-01-01-07-03 | planned (ignored) |
| `publish_site_publishes_per_language` | tests/cli_publish.rs:9 | REQ-04-01-07-01 | planned (ignored) |
| `render_forwards_tool_errors_transparently` | tests/cli_publish.rs:29 | REQ-04-01-03-07 | planned (ignored) |
| `render_pdf_renders_via_pandoc` | tests/cli_publish.rs:18 | REQ-04-01-03-04 | planned (ignored) |
| `report_bundle_exports_an_evidence_bundle_by_id_scope` | tests/cli_report.rs:9 | REQ-03-01-04-01 | planned (ignored) |
| `report_bundle_includes_linked_evidence` | tests/cli_report.rs:21 | REQ-03-01-04-02 | planned (ignored) |
| `trace_check_reports_verifies_markers_per_requirement` | tests/cli_trace.rs:36 | REQ-03-01-06-02 | active |
| `trace_coverage_identifies_requirements_without_verifying_tests` | tests/cli_trace.rs:47 | REQ-01-01-08-01 | active |
| `trace_coverage_output_is_deterministic` | tests/cli_trace.rs:70 | REQ-00-00-00-01 | active |
| `trace_coverage_supports_json_output` | tests/cli_trace.rs:59 | REQ-01-01-08-03 | active |
| `trace_matrix_exports_csv` | tests/cli_trace.rs:107 | REQ-03-01-02-01 | active |
| `trace_scan_detects_markers_in_rust_comments` | tests/cli_trace.rs:9 | REQ-03-01-05-01 | active |
| `trace_scan_does_not_follow_directory_symlinks` | tests/cli_trace.rs:87 | REQ-00-00-00-01 | active |
| `trace_scan_outputs_the_trace_graph_as_json` | tests/cli_trace.rs:24 | REQ-03-01-05-04 | active |
| `unit_new_creates_a_unit_from_the_configured_template` | tests/cli_unit.rs:8 | REQ-00-00-00-05 | active |
| `usage_error_exits_with_code_2` | tests/cli.rs:30 | REQ-00-00-00-02 | active |
| `verify_emits_per_step_results_in_json_mode` | tests/cli_verify.rs:27 | REQ-04-01-05-03 | active |
| `verify_excludes_rendering_from_the_default_loop` | tests/cli_verify.rs:38 | REQ-04-01-05-04 | active |
| `verify_runs_the_configured_sub_steps` | tests/cli_verify.rs:9 | REQ-04-01-05-01 | active |
| `verify_supports_fail_fast_and_aggregate_modes` | tests/cli_verify.rs:17 | REQ-04-01-05-02 | active |
