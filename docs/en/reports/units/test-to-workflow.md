<!-- GENERATED SNAPSHOT — do not edit by hand.
     Question: Q-06 (see docs/en/reports/QUESTIONS.md)
     Snapshot: 886a491, 2026-07-11
     Regenerate: python3 scripts/arqix_report.py --snapshot "<sha>, <date>" -->

# Which workflow belongs to which integration test?

Joined test → requirement → story → workflow (`is-part-of-workflow`).

| test | workflow | title |
| --- | --- | --- |
| `assemble_build_fails_clearly_on_include_cycles` | WF-02-01 | Write Docs Alongside Implementation |
| `assemble_build_fails_on_output_collisions_across_roots` | WF-02-01 | Write Docs Alongside Implementation |
| `assemble_build_generates_outputs_under_pages` | WF-02-01 | Write Docs Alongside Implementation |
| `assemble_build_refuses_includes_outside_the_repository` | WF-01-01 | Establish Standards and Repository Hygiene |
| `assemble_build_refuses_includes_outside_the_repository` | WF-02-01 | Write Docs Alongside Implementation |
| `assemble_build_refuses_includes_outside_the_repository` | WF-04-01 | Run CI Gates and Publish artefacts |
| `assemble_build_refuses_includes_outside_the_repository` | WF-05-01 | Use Documentation for Agents and RAG |
| `assemble_build_refuses_includes_outside_the_repository` | WF-06-01 | Maintain architecture, ADRs, and glossary |
| `assemble_build_refuses_includes_outside_the_repository` | WF-08-01 | Automation Agent: Story-by-story Implementation with arqix |
| `assemble_build_writes_a_jsonl_log` | WF-04-01 | Run CI Gates and Publish artefacts |
| `assemble_build_writes_a_jsonl_log` | WF-05-01 | Use Documentation for Agents and RAG |
| `assemble_build_writes_a_jsonl_log` | WF-06-01 | Maintain architecture, ADRs, and glossary |
| `assemble_build_writes_a_jsonl_log` | WF-08-01 | Automation Agent: Story-by-story Implementation with arqix |
| `config_show_renders_the_effective_configuration_as_json` | WF-01-01 | Establish Standards and Repository Hygiene |
| `config_show_renders_the_effective_configuration_as_json` | WF-04-01 | Run CI Gates and Publish artefacts |
| `config_show_renders_the_effective_configuration_as_json` | WF-05-01 | Use Documentation for Agents and RAG |
| `config_show_renders_the_effective_configuration_as_json` | WF-08-01 | Automation Agent: Story-by-story Implementation with arqix |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | WF-01-01 | Establish Standards and Repository Hygiene |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | WF-02-01 | Write Docs Alongside Implementation |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | WF-03-01 | Validate Traceability and Coverage |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | WF-04-01 | Run CI Gates and Publish artefacts |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | WF-05-01 | Use Documentation for Agents and RAG |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | WF-06-01 | Maintain architecture, ADRs, and glossary |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | WF-07-01 | Review Evidence Chains and Exports |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | WF-08-01 | Automation Agent: Story-by-story Implementation with arqix |
| `config_validate_accepts_a_valid_configuration` | WF-01-01 | Establish Standards and Repository Hygiene |
| `config_validate_accepts_a_valid_configuration` | WF-04-01 | Run CI Gates and Publish artefacts |
| `config_validate_accepts_a_valid_configuration` | WF-05-01 | Use Documentation for Agents and RAG |
| `config_validate_accepts_a_valid_configuration` | WF-08-01 | Automation Agent: Story-by-story Implementation with arqix |
| `config_validate_identifies_the_failing_key` | WF-01-01 | Establish Standards and Repository Hygiene |
| `config_validate_identifies_the_failing_key` | WF-04-01 | Run CI Gates and Publish artefacts |
| `config_validate_identifies_the_failing_key` | WF-05-01 | Use Documentation for Agents and RAG |
| `config_validate_identifies_the_failing_key` | WF-08-01 | Automation Agent: Story-by-story Implementation with arqix |
| `doc_init_creates_the_standard_package_scaffold` | WF-01-01 | Establish Standards and Repository Hygiene |
| `doc_init_creates_the_standard_package_scaffold` | WF-02-01 | Write Docs Alongside Implementation |
| `doc_init_scaffolds_an_explicit_path` | WF-01-01 | Establish Standards and Repository Hygiene |
| `doc_init_scaffolds_an_explicit_path` | WF-02-01 | Write Docs Alongside Implementation |
| `doc_init_writes_doc_index_frontmatter` | WF-01-01 | Establish Standards and Repository Hygiene |
| `doc_init_writes_doc_index_frontmatter` | WF-02-01 | Write Docs Alongside Implementation |
| `doc_list_does_not_follow_directory_symlinks` | WF-01-01 | Establish Standards and Repository Hygiene |
| `doc_list_does_not_follow_directory_symlinks` | WF-02-01 | Write Docs Alongside Implementation |
| `doc_list_does_not_follow_directory_symlinks` | WF-03-01 | Validate Traceability and Coverage |
| `doc_list_does_not_follow_directory_symlinks` | WF-04-01 | Run CI Gates and Publish artefacts |
| `doc_list_does_not_follow_directory_symlinks` | WF-05-01 | Use Documentation for Agents and RAG |
| `doc_list_does_not_follow_directory_symlinks` | WF-06-01 | Maintain architecture, ADRs, and glossary |
| `doc_list_does_not_follow_directory_symlinks` | WF-07-01 | Review Evidence Chains and Exports |
| `doc_list_does_not_follow_directory_symlinks` | WF-08-01 | Automation Agent: Story-by-story Implementation with arqix |
| `doc_list_emits_a_json_document_catalog` | WF-05-01 | Use Documentation for Agents and RAG |
| `doc_list_emits_a_json_document_catalog` | WF-08-01 | Automation Agent: Story-by-story Implementation with arqix |
| `doc_list_filters_the_catalog_by_kind` | WF-05-01 | Use Documentation for Agents and RAG |
| `doc_list_filters_the_catalog_by_kind` | WF-08-01 | Automation Agent: Story-by-story Implementation with arqix |
| `doc_list_honours_configured_skip_dirs` | WF-01-01 | Establish Standards and Repository Hygiene |
| `doc_list_lists_each_document_once_under_overlapping_roots` | WF-05-01 | Use Documentation for Agents and RAG |
| `doc_list_lists_each_document_once_under_overlapping_roots` | WF-08-01 | Automation Agent: Story-by-story Implementation with arqix |
| `doc_list_skips_the_default_directories_without_an_override` | WF-01-01 | Establish Standards and Repository Hygiene |
| `doc_new_accepts_an_explicit_id_and_rejects_a_duplicate` | WF-01-01 | Establish Standards and Repository Hygiene |
| `doc_new_accepts_an_explicit_id_and_rejects_a_duplicate` | WF-02-01 | Write Docs Alongside Implementation |
| `doc_new_accepts_an_explicit_id_and_rejects_a_duplicate` | WF-08-01 | Automation Agent: Story-by-story Implementation with arqix |
| `doc_new_creates_a_document_from_the_configured_template` | WF-01-01 | Establish Standards and Repository Hygiene |
| `doc_new_creates_a_document_from_the_configured_template` | WF-02-01 | Write Docs Alongside Implementation |
| `doc_new_creates_a_document_from_the_configured_template` | WF-05-01 | Use Documentation for Agents and RAG |
| `doc_new_creates_a_document_from_the_configured_template` | WF-06-01 | Maintain architecture, ADRs, and glossary |
| `doc_new_creates_a_document_from_the_configured_template` | WF-08-01 | Automation Agent: Story-by-story Implementation with arqix |
| `doc_new_dry_run_reports_the_plan_without_writing` | WF-01-01 | Establish Standards and Repository Hygiene |
| `doc_new_dry_run_reports_the_plan_without_writing` | WF-02-01 | Write Docs Alongside Implementation |
| `doc_new_dry_run_reports_the_plan_without_writing` | WF-06-01 | Maintain architecture, ADRs, and glossary |
| `doc_new_dry_run_reports_the_plan_without_writing` | WF-08-01 | Automation Agent: Story-by-story Implementation with arqix |
| `doc_new_generates_a_unique_id_from_the_configured_policy` | WF-01-01 | Establish Standards and Repository Hygiene |
| `doc_new_generates_a_unique_id_from_the_configured_policy` | WF-02-01 | Write Docs Alongside Implementation |
| `doc_new_generates_a_unique_id_from_the_configured_policy` | WF-08-01 | Automation Agent: Story-by-story Implementation with arqix |
| `doc_new_rejects_a_kind_that_escapes_the_root` | WF-01-01 | Establish Standards and Repository Hygiene |
| `doc_new_rejects_a_kind_that_escapes_the_root` | WF-02-01 | Write Docs Alongside Implementation |
| `doc_new_rejects_a_kind_that_escapes_the_root` | WF-04-01 | Run CI Gates and Publish artefacts |
| `doc_new_rejects_a_kind_that_escapes_the_root` | WF-05-01 | Use Documentation for Agents and RAG |
| `doc_new_rejects_a_kind_that_escapes_the_root` | WF-06-01 | Maintain architecture, ADRs, and glossary |
| `doc_new_rejects_a_kind_that_escapes_the_root` | WF-08-01 | Automation Agent: Story-by-story Implementation with arqix |
| `doc_new_substitutes_the_title_into_the_template` | WF-01-01 | Establish Standards and Repository Hygiene |
| `doc_new_substitutes_the_title_into_the_template` | WF-02-01 | Write Docs Alongside Implementation |
| `doc_new_substitutes_the_title_into_the_template` | WF-05-01 | Use Documentation for Agents and RAG |
| `doc_new_substitutes_the_title_into_the_template` | WF-06-01 | Maintain architecture, ADRs, and glossary |
| `doc_new_substitutes_the_title_into_the_template` | WF-08-01 | Automation Agent: Story-by-story Implementation with arqix |
| `doc_new_writes_into_the_configured_kind_location` | WF-01-01 | Establish Standards and Repository Hygiene |
| `doc_new_writes_into_the_configured_kind_location` | WF-02-01 | Write Docs Alongside Implementation |
| `doc_new_writes_into_the_configured_kind_location` | WF-08-01 | Automation Agent: Story-by-story Implementation with arqix |
| `doc_read_distinguishes_a_document_miss_from_a_selector_miss` | WF-05-01 | Use Documentation for Agents and RAG |
| `doc_read_distinguishes_a_document_miss_from_a_selector_miss` | WF-08-01 | Automation Agent: Story-by-story Implementation with arqix |
| `doc_read_retrieves_a_document_by_id` | WF-05-01 | Use Documentation for Agents and RAG |
| `doc_read_retrieves_a_document_by_id` | WF-08-01 | Automation Agent: Story-by-story Implementation with arqix |
| `doc_search_finds_documents_by_full_text` | WF-02-01 | Write Docs Alongside Implementation |
| `finalise_fails_clearly_on_unsupported_frontmatter` | WF-01-01 | Establish Standards and Repository Hygiene |
| `finalise_fails_clearly_on_unsupported_frontmatter` | WF-02-01 | Write Docs Alongside Implementation |
| `finalise_fails_clearly_on_unsupported_frontmatter` | WF-08-01 | Automation Agent: Story-by-story Implementation with arqix |
| `finalise_leaves_current_metadata_untouched` | WF-01-01 | Establish Standards and Repository Hygiene |
| `finalise_leaves_current_metadata_untouched` | WF-02-01 | Write Docs Alongside Implementation |
| `finalise_leaves_current_metadata_untouched` | WF-08-01 | Automation Agent: Story-by-story Implementation with arqix |
| `finalise_rejects_a_non_iso_date` | WF-01-01 | Establish Standards and Repository Hygiene |
| `finalise_rejects_a_non_iso_date` | WF-02-01 | Write Docs Alongside Implementation |
| `finalise_rejects_a_non_iso_date` | WF-08-01 | Automation Agent: Story-by-story Implementation with arqix |
| `finalise_sets_updated_to_the_injected_date` | WF-01-01 | Establish Standards and Repository Hygiene |
| `finalise_sets_updated_to_the_injected_date` | WF-02-01 | Write Docs Alongside Implementation |
| `finalise_sets_updated_to_the_injected_date` | WF-08-01 | Automation Agent: Story-by-story Implementation with arqix |
| `finalise_touches_only_the_meta_updated_field` | WF-01-01 | Establish Standards and Repository Hygiene |
| `finalise_touches_only_the_meta_updated_field` | WF-02-01 | Write Docs Alongside Implementation |
| `finalise_touches_only_the_meta_updated_field` | WF-08-01 | Automation Agent: Story-by-story Implementation with arqix |
| `fmt_is_idempotent` | WF-01-01 | Establish Standards and Repository Hygiene |
| `fmt_is_idempotent` | WF-02-01 | Write Docs Alongside Implementation |
| `fmt_is_idempotent` | WF-03-01 | Validate Traceability and Coverage |
| `fmt_is_idempotent` | WF-04-01 | Run CI Gates and Publish artefacts |
| `fmt_is_idempotent` | WF-05-01 | Use Documentation for Agents and RAG |
| `fmt_is_idempotent` | WF-06-01 | Maintain architecture, ADRs, and glossary |
| `fmt_is_idempotent` | WF-07-01 | Review Evidence Chains and Exports |
| `fmt_is_idempotent` | WF-08-01 | Automation Agent: Story-by-story Implementation with arqix |
| `fmt_never_changes_body_text` | WF-01-01 | Establish Standards and Repository Hygiene |
| `fmt_never_changes_body_text` | WF-02-01 | Write Docs Alongside Implementation |
| `fmt_never_changes_body_text` | WF-08-01 | Automation Agent: Story-by-story Implementation with arqix |
| `fmt_orders_ontology_frontmatter_by_family` | WF-01-01 | Establish Standards and Repository Hygiene |
| `fmt_orders_ontology_frontmatter_by_family` | WF-02-01 | Write Docs Alongside Implementation |
| `fmt_orders_ontology_frontmatter_by_family` | WF-08-01 | Automation Agent: Story-by-story Implementation with arqix |
| `fmt_sorts_frontmatter_keys_canonically` | WF-01-01 | Establish Standards and Repository Hygiene |
| `fmt_sorts_frontmatter_keys_canonically` | WF-02-01 | Write Docs Alongside Implementation |
| `fmt_sorts_frontmatter_keys_canonically` | WF-08-01 | Automation Agent: Story-by-story Implementation with arqix |
| `format_option_is_accepted_globally` | WF-04-01 | Run CI Gates and Publish artefacts |
| `format_option_is_accepted_globally` | WF-05-01 | Use Documentation for Agents and RAG |
| `format_option_is_accepted_globally` | WF-08-01 | Automation Agent: Story-by-story Implementation with arqix |
| `lint_run_checks_that_include_targets_exist` | WF-01-01 | Establish Standards and Repository Hygiene |
| `lint_run_checks_that_include_targets_exist` | WF-02-01 | Write Docs Alongside Implementation |
| `lint_run_checks_that_include_targets_exist` | WF-03-01 | Validate Traceability and Coverage |
| `lint_run_checks_that_include_targets_exist` | WF-08-01 | Automation Agent: Story-by-story Implementation with arqix |
| `lint_run_detects_translation_drift` | WF-01-01 | Establish Standards and Repository Hygiene |
| `lint_run_detects_translation_drift` | WF-04-01 | Run CI Gates and Publish artefacts |
| `lint_run_detects_translation_drift` | WF-05-01 | Use Documentation for Agents and RAG |
| `lint_run_detects_translation_drift` | WF-08-01 | Automation Agent: Story-by-story Implementation with arqix |
| `lint_run_reports_duplicate_ids_globally` | WF-01-01 | Establish Standards and Repository Hygiene |
| `lint_run_reports_duplicate_ids_globally` | WF-02-01 | Write Docs Alongside Implementation |
| `lint_run_reports_duplicate_ids_globally` | WF-03-01 | Validate Traceability and Coverage |
| `lint_run_reports_duplicate_ids_globally` | WF-08-01 | Automation Agent: Story-by-story Implementation with arqix |
| `lint_run_reports_findings_with_file_and_line_context` | WF-01-01 | Establish Standards and Repository Hygiene |
| `lint_run_reports_findings_with_file_and_line_context` | WF-02-01 | Write Docs Alongside Implementation |
| `lint_run_reports_findings_with_file_and_line_context` | WF-03-01 | Validate Traceability and Coverage |
| `lint_run_reports_findings_with_file_and_line_context` | WF-08-01 | Automation Agent: Story-by-story Implementation with arqix |
| `mcp_serve_speaks_jsonrpc_over_stdio` | WF-05-01 | Use Documentation for Agents and RAG |
| `mcp_serve_speaks_jsonrpc_over_stdio` | WF-08-01 | Automation Agent: Story-by-story Implementation with arqix |
| `policy_check_evaluates_changed_files_against_the_declared_scope` | WF-01-01 | Establish Standards and Repository Hygiene |
| `policy_check_evaluates_changed_files_against_the_declared_scope` | WF-04-01 | Run CI Gates and Publish artefacts |
| `policy_check_evaluates_changed_files_against_the_declared_scope` | WF-08-01 | Automation Agent: Story-by-story Implementation with arqix |
| `policy_check_supports_warn_only_mode` | WF-01-01 | Establish Standards and Repository Hygiene |
| `policy_check_supports_warn_only_mode` | WF-04-01 | Run CI Gates and Publish artefacts |
| `policy_check_supports_warn_only_mode` | WF-08-01 | Automation Agent: Story-by-story Implementation with arqix |
| `publish_site_publishes_per_language` | WF-04-01 | Run CI Gates and Publish artefacts |
| `publish_site_publishes_per_language` | WF-05-01 | Use Documentation for Agents and RAG |
| `render_forwards_tool_errors_transparently` | WF-04-01 | Run CI Gates and Publish artefacts |
| `render_forwards_tool_errors_transparently` | WF-06-01 | Maintain architecture, ADRs, and glossary |
| `render_pdf_renders_via_pandoc` | WF-04-01 | Run CI Gates and Publish artefacts |
| `render_pdf_renders_via_pandoc` | WF-06-01 | Maintain architecture, ADRs, and glossary |
| `report_bundle_exports_an_evidence_bundle_by_id_scope` | WF-03-01 | Validate Traceability and Coverage |
| `report_bundle_includes_linked_evidence` | WF-03-01 | Validate Traceability and Coverage |
| `trace_check_reports_verifies_markers_per_requirement` | WF-03-01 | Validate Traceability and Coverage |
| `trace_coverage_identifies_requirements_without_verifying_tests` | WF-01-01 | Establish Standards and Repository Hygiene |
| `trace_coverage_identifies_requirements_without_verifying_tests` | WF-03-01 | Validate Traceability and Coverage |
| `trace_coverage_identifies_requirements_without_verifying_tests` | WF-07-01 | Review Evidence Chains and Exports |
| `trace_coverage_output_is_deterministic` | WF-01-01 | Establish Standards and Repository Hygiene |
| `trace_coverage_output_is_deterministic` | WF-02-01 | Write Docs Alongside Implementation |
| `trace_coverage_output_is_deterministic` | WF-03-01 | Validate Traceability and Coverage |
| `trace_coverage_output_is_deterministic` | WF-04-01 | Run CI Gates and Publish artefacts |
| `trace_coverage_output_is_deterministic` | WF-05-01 | Use Documentation for Agents and RAG |
| `trace_coverage_output_is_deterministic` | WF-06-01 | Maintain architecture, ADRs, and glossary |
| `trace_coverage_output_is_deterministic` | WF-07-01 | Review Evidence Chains and Exports |
| `trace_coverage_output_is_deterministic` | WF-08-01 | Automation Agent: Story-by-story Implementation with arqix |
| `trace_coverage_supports_json_output` | WF-01-01 | Establish Standards and Repository Hygiene |
| `trace_coverage_supports_json_output` | WF-03-01 | Validate Traceability and Coverage |
| `trace_coverage_supports_json_output` | WF-07-01 | Review Evidence Chains and Exports |
| `trace_matrix_exports_csv` | WF-03-01 | Validate Traceability and Coverage |
| `trace_ratchet_fails_on_a_coverage_regression` | WF-04-01 | Run CI Gates and Publish artefacts |
| `trace_ratchet_passes_without_regression` | WF-04-01 | Run CI Gates and Publish artefacts |
| `trace_scan_detects_markers_in_rust_comments` | WF-03-01 | Validate Traceability and Coverage |
| `trace_scan_does_not_follow_directory_symlinks` | WF-01-01 | Establish Standards and Repository Hygiene |
| `trace_scan_does_not_follow_directory_symlinks` | WF-02-01 | Write Docs Alongside Implementation |
| `trace_scan_does_not_follow_directory_symlinks` | WF-03-01 | Validate Traceability and Coverage |
| `trace_scan_does_not_follow_directory_symlinks` | WF-04-01 | Run CI Gates and Publish artefacts |
| `trace_scan_does_not_follow_directory_symlinks` | WF-05-01 | Use Documentation for Agents and RAG |
| `trace_scan_does_not_follow_directory_symlinks` | WF-06-01 | Maintain architecture, ADRs, and glossary |
| `trace_scan_does_not_follow_directory_symlinks` | WF-07-01 | Review Evidence Chains and Exports |
| `trace_scan_does_not_follow_directory_symlinks` | WF-08-01 | Automation Agent: Story-by-story Implementation with arqix |
| `trace_scan_outputs_the_trace_graph_as_json` | WF-03-01 | Validate Traceability and Coverage |
| `unit_new_creates_a_unit_from_the_configured_template` | WF-01-01 | Establish Standards and Repository Hygiene |
| `unit_new_creates_a_unit_from_the_configured_template` | WF-02-01 | Write Docs Alongside Implementation |
| `unit_new_creates_a_unit_from_the_configured_template` | WF-05-01 | Use Documentation for Agents and RAG |
| `unit_new_creates_a_unit_from_the_configured_template` | WF-06-01 | Maintain architecture, ADRs, and glossary |
| `unit_new_creates_a_unit_from_the_configured_template` | WF-08-01 | Automation Agent: Story-by-story Implementation with arqix |
| `usage_error_exits_with_code_2` | WF-01-01 | Establish Standards and Repository Hygiene |
| `usage_error_exits_with_code_2` | WF-04-01 | Run CI Gates and Publish artefacts |
| `usage_error_exits_with_code_2` | WF-05-01 | Use Documentation for Agents and RAG |
| `usage_error_exits_with_code_2` | WF-08-01 | Automation Agent: Story-by-story Implementation with arqix |
| `verify_defaults_to_informational_coverage_and_gating_rest` | WF-04-01 | Run CI Gates and Publish artefacts |
| `verify_emits_per_step_results_in_json_mode` | WF-04-01 | Run CI Gates and Publish artefacts |
| `verify_emits_per_step_results_in_json_mode` | WF-08-01 | Automation Agent: Story-by-story Implementation with arqix |
| `verify_excludes_rendering_from_the_default_loop` | WF-04-01 | Run CI Gates and Publish artefacts |
| `verify_excludes_rendering_from_the_default_loop` | WF-08-01 | Automation Agent: Story-by-story Implementation with arqix |
| `verify_reports_informational_findings_without_gating` | WF-04-01 | Run CI Gates and Publish artefacts |
| `verify_runs_exactly_the_configured_steps_in_order` | WF-04-01 | Run CI Gates and Publish artefacts |
| `verify_runs_the_configured_sub_steps` | WF-04-01 | Run CI Gates and Publish artefacts |
| `verify_runs_the_configured_sub_steps` | WF-08-01 | Automation Agent: Story-by-story Implementation with arqix |
| `verify_supports_fail_fast_and_aggregate_modes` | WF-04-01 | Run CI Gates and Publish artefacts |
| `verify_supports_fail_fast_and_aggregate_modes` | WF-08-01 | Automation Agent: Story-by-story Implementation with arqix |
