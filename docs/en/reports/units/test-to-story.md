<!-- GENERATED SNAPSHOT — do not edit by hand.
     Question: Q-05 (see docs/en/reports/QUESTIONS.md)
     Snapshot: 16cefb7, 2026-07-14
     Regenerate: python3 scripts/arqix_report.py --snapshot "<sha>, <date>" -->

# Which user story belongs to which integration test?

Joined test → requirement (`verifies`) → story (`derived-from`).

| test | story | title |
| --- | --- | --- |
| `a_fresh_package_passes_the_verification_loop_directly` | US-08-01-01 | Initialize a Doc Package Deterministically and Safely |
| `adrs_follow_the_path_model_in_the_canonical_governance_language` | US-01-01-11 | Govern Architecture Documentation Standards |
| `adrs_follow_the_path_model_in_the_canonical_governance_language` | US-06-01-07 | Maintain Architecture and Governance Documentation Consistently |
| `agent_extension_points_carry_no_normative_process_rules` | US-01-01-09 | Govern Agent Workflow Document Standards |
| `agent_extension_points_carry_no_normative_process_rules` | US-08-01-18 | Standardize Agent Workflow Documents |
| `agent_instructions_define_plan_editing_constraints_and_the_verification_loop` | US-01-01-09 | Govern Agent Workflow Document Standards |
| `agent_instructions_define_plan_editing_constraints_and_the_verification_loop` | US-08-01-18 | Standardize Agent Workflow Documents |
| `agent_instructions_define_story_by_story_scope_rules` | US-01-01-09 | Govern Agent Workflow Document Standards |
| `agent_instructions_define_story_by_story_scope_rules` | US-08-01-18 | Standardize Agent Workflow Documents |
| `agent_instructions_restrict_agents_to_release_preparation` | US-01-01-15 | Operationalise the Release Process with SemVer |
| `agent_instructions_restrict_agents_to_release_preparation` | US-04-01-09 | Run Governed Release Preparation Workflows |
| `agent_instructions_restrict_agents_to_release_preparation` | US-08-01-17 | Prepare Releases within Explicit Automation Boundaries |
| `arc42_documentation_is_structured_into_assemblable_units_per_chapter` | US-01-01-11 | Govern Architecture Documentation Standards |
| `arc42_documentation_is_structured_into_assemblable_units_per_chapter` | US-06-01-07 | Maintain Architecture and Governance Documentation Consistently |
| `architecture_documentation_records_the_consistency_check_extension_path` | US-01-01-11 | Govern Architecture Documentation Standards |
| `architecture_documentation_records_the_consistency_check_extension_path` | US-06-01-07 | Maintain Architecture and Governance Documentation Consistently |
| `architecture_views_are_generated_from_the_c4_model` | US-01-01-11 | Govern Architecture Documentation Standards |
| `architecture_views_are_generated_from_the_c4_model` | US-04-01-18 | Render Architecture Views from the Model |
| `architecture_views_are_generated_from_the_c4_model` | US-06-01-07 | Maintain Architecture and Governance Documentation Consistently |
| `assemble_applies_the_configured_heading_ownership_default` | US-02-01-12 | Stitch Fragments at Declared Heading Levels |
| `assemble_build_fails_clearly_on_include_cycles` | US-02-01-11 | Assemble Documentation During Implementation |
| `assemble_build_fails_on_output_collisions_across_roots` | US-02-01-11 | Assemble Documentation During Implementation |
| `assemble_build_generates_outputs_under_pages` | US-02-01-11 | Assemble Documentation During Implementation |
| `assemble_build_refuses_includes_outside_the_repository` | US-01-01-07 | Enforce Scope Guardrails for Automation Agents |
| `assemble_build_refuses_includes_outside_the_repository` | US-02-01-09 | Use Include Directives During Implementation |
| `assemble_build_refuses_includes_outside_the_repository` | US-04-01-02 | Check Scope Guardrails in CI |
| `assemble_build_refuses_includes_outside_the_repository` | US-05-01-04 | Parse Document Structure Deterministically for Automation |
| `assemble_build_refuses_includes_outside_the_repository` | US-06-01-04 | Compose Modular Documents with Chapter and Include Directives |
| `assemble_build_refuses_includes_outside_the_repository` | US-08-01-08 | Stay within Declared Change Scope |
| `assemble_build_writes_a_jsonl_log` | US-04-01-01 | Emit a CI-Friendly Assembly Log |
| `assemble_build_writes_a_jsonl_log` | US-05-01-02 | Emit a Machine-Readable Assembly Log |
| `assemble_build_writes_a_jsonl_log` | US-06-01-02 | Trace Document Assembly Structure |
| `assemble_build_writes_a_jsonl_log` | US-08-01-02 | Emit a Deterministic Assembly Log for Verification |
| `assemble_fails_on_heading_overflow` | US-02-01-12 | Stitch Fragments at Declared Heading Levels |
| `assemble_rebases_relative_links_from_included_fragments` | US-04-01-03 | Generate Publishing Outputs |
| `assemble_rebases_relative_links_from_included_fragments` | US-06-01-05 | Generate Publishable Documentation Outputs |
| `assemble_resolves_relative_levels_at_the_include_position` | US-02-01-12 | Stitch Fragments at Declared Heading Levels |
| `assemble_shifts_included_headings_to_the_declared_level` | US-02-01-12 | Stitch Fragments at Declared Heading Levels |
| `assembly_log_records_carry_stable_field_names` | US-05-01-02 | Emit a Machine-Readable Assembly Log |
| `assembly_outcomes_are_reviewable_from_log_and_exit_code` | US-06-01-02 | Trace Document Assembly Structure |
| `assembly_outcomes_are_reviewable_from_log_and_exit_code` | US-08-01-02 | Emit a Deterministic Assembly Log for Verification |
| `breaking_releases_require_migration_notes_and_changelog_entries` | US-01-01-15 | Operationalise the Release Process with SemVer |
| `breaking_releases_require_migration_notes_and_changelog_entries` | US-04-01-09 | Run Governed Release Preparation Workflows |
| `breaking_releases_require_migration_notes_and_changelog_entries` | US-08-01-17 | Prepare Releases within Explicit Automation Boundaries |
| `catalogue_entries_carry_anchors_and_coverage_status` | US-04-01-17 | Publish the Specification as Catalogue Pages |
| `catalogue_pages_are_deterministic` | US-04-01-17 | Publish the Specification as Catalogue Pages |
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
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-01-01-17 | Configure Discovery Scope |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-01-01-19 | Configure Frontmatter Contracts |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-02-01-02 | Create Units Quickly During Implementation |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-02-01-03 | Format Documents During Implementation |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-02-01-04 | Lint Documents Before Commit |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-02-01-05 | Create Documents Quickly from Templates |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-02-01-07 | Create Conforming Documents Quickly via Templates |
| `config_validate_accepts_a_missing_file_as_pure_defaults` | US-02-01-09 | Use Include Directives During Implementation |
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
| `coverage_joins_junit_outcomes_by_test_name` | US-03-01-10 | Track Planned and Executed Test Evidence |
| `coverage_without_results_is_byte_identical_to_before` | US-03-01-10 | Track Planned and Executed Test Evidence |
| `creation_aliases_mirror_doc_new` | US-01-01-05 | Create Documents from Configured Templates |
| `creation_aliases_mirror_doc_new` | US-02-01-05 | Create Documents Quickly from Templates |
| `creation_aliases_mirror_doc_new` | US-06-01-03 | Create Architecture Documents from Templates |
| `creation_aliases_mirror_doc_new` | US-08-01-05 | Create Documents Deterministically from Templates |
| `creation_never_overwrites_an_existing_document` | US-01-01-01 | Initialise Standardised Doc Package |
| `creation_never_overwrites_an_existing_document` | US-01-01-06 | Finalise Document Metadata Mechanically |
| `creation_never_overwrites_an_existing_document` | US-02-01-01 | Initialize a Doc Package with One Command |
| `creation_never_overwrites_an_existing_document` | US-02-01-08 | Finalise Metadata without Touching Content |
| `creation_never_overwrites_an_existing_document` | US-08-01-01 | Initialize a Doc Package Deterministically and Safely |
| `creation_never_overwrites_an_existing_document` | US-08-01-06 | Finalise Metadata Safely and Deterministically |
| `diagnostics_are_machine_readable_with_the_tool_wide_shape` | US-01-01-07 | Enforce Scope Guardrails for Automation Agents |
| `diagnostics_are_machine_readable_with_the_tool_wide_shape` | US-01-01-08 | Generate Governed Coverage Reports |
| `diagnostics_are_machine_readable_with_the_tool_wide_shape` | US-01-01-14 | Lint Translation Metadata and Drift |
| `diagnostics_are_machine_readable_with_the_tool_wide_shape` | US-03-01-03 | Generate Coverage Reports |
| `diagnostics_are_machine_readable_with_the_tool_wide_shape` | US-03-01-05 | Scan Traceability Information |
| `diagnostics_are_machine_readable_with_the_tool_wide_shape` | US-03-01-06 | Detect Missing Trace Markers for Quality Gaps |
| `diagnostics_are_machine_readable_with_the_tool_wide_shape` | US-03-01-08 | Make Trace and Coverage Outputs Reproducible |
| `diagnostics_are_machine_readable_with_the_tool_wide_shape` | US-04-01-01 | Emit a CI-Friendly Assembly Log |
| `diagnostics_are_machine_readable_with_the_tool_wide_shape` | US-04-01-02 | Check Scope Guardrails in CI |
| `diagnostics_are_machine_readable_with_the_tool_wide_shape` | US-04-01-04 | Gate Bilingual Documentation Quality in CI |
| `diagnostics_are_machine_readable_with_the_tool_wide_shape` | US-04-01-05 | Run a One-Command Verification Loop |
| `diagnostics_are_machine_readable_with_the_tool_wide_shape` | US-04-01-07 | Publish Language-Aware Sites |
| `diagnostics_are_machine_readable_with_the_tool_wide_shape` | US-04-01-10 | Emit Machine-Readable Diagnostics for CI |
| `diagnostics_are_machine_readable_with_the_tool_wide_shape` | US-04-01-12 | Publish Stable Report Exports for Automation |
| `diagnostics_are_machine_readable_with_the_tool_wide_shape` | US-05-01-02 | Emit a Machine-Readable Assembly Log |
| `diagnostics_are_machine_readable_with_the_tool_wide_shape` | US-05-01-05 | Detect Translation Drift for Automation |
| `diagnostics_are_machine_readable_with_the_tool_wide_shape` | US-05-01-07 | Build Machine-Readable Trace Graphs |
| `diagnostics_are_machine_readable_with_the_tool_wide_shape` | US-05-01-08 | Export a Deterministic Document Catalog |
| `diagnostics_are_machine_readable_with_the_tool_wide_shape` | US-05-01-13 | Expose Language-Aware Site Outputs Deterministically |
| `diagnostics_are_machine_readable_with_the_tool_wide_shape` | US-05-01-14 | Emit Machine-Readable Diagnostics for Downstream Tooling |
| `diagnostics_are_machine_readable_with_the_tool_wide_shape` | US-06-01-02 | Trace Document Assembly Structure |
| `diagnostics_are_machine_readable_with_the_tool_wide_shape` | US-07-01-01 | Review Coverage Evidence |
| `diagnostics_are_machine_readable_with_the_tool_wide_shape` | US-07-01-04 | Review Trace Graphs as Audit Evidence |
| `diagnostics_are_machine_readable_with_the_tool_wide_shape` | US-07-01-06 | Export Deterministic Trace and Coverage Evidence |
| `diagnostics_are_machine_readable_with_the_tool_wide_shape` | US-07-01-07 | Publish Stable Compliance-Ready Report Exports |
| `diagnostics_are_machine_readable_with_the_tool_wide_shape` | US-08-01-02 | Emit a Deterministic Assembly Log for Verification |
| `diagnostics_are_machine_readable_with_the_tool_wide_shape` | US-08-01-07 | List Documents Deterministically for Automation |
| `diagnostics_are_machine_readable_with_the_tool_wide_shape` | US-08-01-08 | Stay within Declared Change Scope |
| `diagnostics_are_machine_readable_with_the_tool_wide_shape` | US-08-01-11 | Interpret i18n Lint Results Deterministically |
| `diagnostics_are_machine_readable_with_the_tool_wide_shape` | US-08-01-13 | Run One-Command Verification in Agent Workflows |
| `diagnostics_are_machine_readable_with_the_tool_wide_shape` | US-08-01-16 | Scan Traceability Deterministically within Verification Loops |
| `diagnostics_are_machine_readable_with_the_tool_wide_shape` | US-08-01-19 | Detect Missing Trace Markers for a Requirement |
| `diagnostics_are_machine_readable_with_the_tool_wide_shape` | US-08-01-21 | Emit Machine-Readable Diagnostics for Agent Workflows |
| `diagnostics_are_machine_readable_with_the_tool_wide_shape` | US-08-01-22 | Make Trace and Coverage Outputs Deterministic |
| `doc_init_creates_the_standard_package_scaffold` | US-01-01-01 | Initialise Standardised Doc Package |
| `doc_init_creates_the_standard_package_scaffold` | US-02-01-01 | Initialize a Doc Package with One Command |
| `doc_init_never_overwrites_agent_instructions` | US-01-01-21 | Scaffold Agent Instructions on Init |
| `doc_init_scaffolds_agent_instructions` | US-01-01-21 | Scaffold Agent Instructions on Init |
| `doc_init_scaffolds_an_explicit_path` | US-01-01-01 | Initialise Standardised Doc Package |
| `doc_init_scaffolds_an_explicit_path` | US-02-01-01 | Initialize a Doc Package with One Command |
| `doc_init_scaffolds_the_default_template_files` | US-01-01-20 | Create Documents from Template Files |
| `doc_init_writes_doc_index_frontmatter` | US-01-01-01 | Initialise Standardised Doc Package |
| `doc_init_writes_doc_index_frontmatter` | US-02-01-01 | Initialize a Doc Package with One Command |
| `doc_list_does_not_follow_directory_symlinks` | US-01-01-03 | Format Documents Canonically |
| `doc_list_does_not_follow_directory_symlinks` | US-01-01-04 | Lint Documents Deterministically |
| `doc_list_does_not_follow_directory_symlinks` | US-01-01-08 | Generate Governed Coverage Reports |
| `doc_list_does_not_follow_directory_symlinks` | US-01-01-10 | Define Schema-Backed Metadata Contracts |
| `doc_list_does_not_follow_directory_symlinks` | US-01-01-12 | Govern Glossary Term Metadata and IDs |
| `doc_list_does_not_follow_directory_symlinks` | US-01-01-16 | Validate Repository Configuration and Inspect Effective Config |
| `doc_list_does_not_follow_directory_symlinks` | US-02-01-03 | Format Documents During Implementation |
| `doc_list_does_not_follow_directory_symlinks` | US-02-01-04 | Lint Documents Before Commit |
| `doc_list_does_not_follow_directory_symlinks` | US-02-01-06 | Find and Read Documentation During Implementation |
| `doc_list_does_not_follow_directory_symlinks` | US-02-01-09 | Use Include Directives During Implementation |
| `doc_list_does_not_follow_directory_symlinks` | US-02-01-11 | Assemble Documentation During Implementation |
| `doc_list_does_not_follow_directory_symlinks` | US-03-01-01 | Lint Documents for Traceability Gaps |
| `doc_list_does_not_follow_directory_symlinks` | US-03-01-02 | Export Trace Matrices |
| `doc_list_does_not_follow_directory_symlinks` | US-03-01-03 | Generate Coverage Reports |
| `doc_list_does_not_follow_directory_symlinks` | US-03-01-04 | Export Scoped Evidence Bundles for Quality Review |
| `doc_list_does_not_follow_directory_symlinks` | US-03-01-07 | Filter Traceability Reports for Quality Analysis |
| `doc_list_does_not_follow_directory_symlinks` | US-03-01-08 | Make Trace and Coverage Outputs Reproducible |
| `doc_list_does_not_follow_directory_symlinks` | US-04-01-06 | Build Deterministic Page Artefacts in CI |
| `doc_list_does_not_follow_directory_symlinks` | US-04-01-10 | Emit Machine-Readable Diagnostics for CI |
| `doc_list_does_not_follow_directory_symlinks` | US-04-01-11 | Inspect Effective Config for CI Reproducibility |
| `doc_list_does_not_follow_directory_symlinks` | US-05-01-03 | Expose Machine-Usable Metadata Contracts |
| `doc_list_does_not_follow_directory_symlinks` | US-05-01-04 | Parse Document Structure Deterministically for Automation |
| `doc_list_does_not_follow_directory_symlinks` | US-05-01-06 | Search and Read Documentation via CLI |
| `doc_list_does_not_follow_directory_symlinks` | US-05-01-08 | Export a Deterministic Document Catalog |
| `doc_list_does_not_follow_directory_symlinks` | US-05-01-09 | Observe Assembled Outputs for Downstream Tooling |
| `doc_list_does_not_follow_directory_symlinks` | US-05-01-10 | Read Structured Document Sections with Stable Selectors |
| `doc_list_does_not_follow_directory_symlinks` | US-05-01-11 | Consume Effective Configuration as Automation Baseline |
| `doc_list_does_not_follow_directory_symlinks` | US-05-01-14 | Emit Machine-Readable Diagnostics for Downstream Tooling |
| `doc_list_does_not_follow_directory_symlinks` | US-06-01-04 | Compose Modular Documents with Chapter and Include Directives |
| `doc_list_does_not_follow_directory_symlinks` | US-06-01-08 | Assemble Modular Document Packages into Pages |
| `doc_list_does_not_follow_directory_symlinks` | US-06-01-09 | Retrieve Architecture Documentation Quickly |
| `doc_list_does_not_follow_directory_symlinks` | US-06-01-10 | Create Glossary Terms with Stable IDs |
| `doc_list_does_not_follow_directory_symlinks` | US-07-01-01 | Review Coverage Evidence |
| `doc_list_does_not_follow_directory_symlinks` | US-07-01-02 | Review Evidence Chains through Trace Matrices |
| `doc_list_does_not_follow_directory_symlinks` | US-07-01-03 | Generate Audit Evidence Bundles by Scope |
| `doc_list_does_not_follow_directory_symlinks` | US-07-01-05 | Filter Traceability Reports for Audit Review |
| `doc_list_does_not_follow_directory_symlinks` | US-07-01-06 | Export Deterministic Trace and Coverage Evidence |
| `doc_list_does_not_follow_directory_symlinks` | US-08-01-01 | Initialize a Doc Package Deterministically and Safely |
| `doc_list_does_not_follow_directory_symlinks` | US-08-01-03 | Format Documents Deterministically within Scope |
| `doc_list_does_not_follow_directory_symlinks` | US-08-01-04 | Lint Documents with Actionable Diagnostics |
| `doc_list_does_not_follow_directory_symlinks` | US-08-01-06 | Finalise Metadata Safely and Deterministically |
| `doc_list_does_not_follow_directory_symlinks` | US-08-01-07 | List Documents Deterministically for Automation |
| `doc_list_does_not_follow_directory_symlinks` | US-08-01-09 | Read Precise Document Sections for Scoped Execution |
| `doc_list_does_not_follow_directory_symlinks` | US-08-01-10 | Use Metadata Contracts Deterministically |
| `doc_list_does_not_follow_directory_symlinks` | US-08-01-20 | Read Effective Config Deterministically Before Execution |
| `doc_list_does_not_follow_directory_symlinks` | US-08-01-21 | Emit Machine-Readable Diagnostics for Agent Workflows |
| `doc_list_does_not_follow_directory_symlinks` | US-08-01-22 | Make Trace and Coverage Outputs Deterministic |
| `doc_list_emits_a_json_document_catalog` | US-05-01-08 | Export a Deterministic Document Catalog |
| `doc_list_emits_a_json_document_catalog` | US-08-01-07 | List Documents Deterministically for Automation |
| `doc_list_filters_the_catalog_by_kind` | US-05-01-08 | Export a Deterministic Document Catalog |
| `doc_list_filters_the_catalog_by_kind` | US-08-01-07 | List Documents Deterministically for Automation |
| `doc_list_honours_configured_skip_dirs` | US-01-01-17 | Configure Discovery Scope |
| `doc_list_lists_each_document_once_under_overlapping_roots` | US-05-01-08 | Export a Deterministic Document Catalog |
| `doc_list_lists_each_document_once_under_overlapping_roots` | US-08-01-07 | List Documents Deterministically for Automation |
| `doc_list_skips_the_default_directories_without_an_override` | US-01-01-17 | Configure Discovery Scope |
| `doc_new_accepts_an_explicit_id_and_rejects_a_duplicate` | US-01-01-13 | Govern Deterministic Document Creation via Templates |
| `doc_new_accepts_an_explicit_id_and_rejects_a_duplicate` | US-02-01-07 | Create Conforming Documents Quickly via Templates |
| `doc_new_accepts_an_explicit_id_and_rejects_a_duplicate` | US-08-01-23 | Create Documents without Ambiguity via Templates |
| `doc_new_creates_a_document_from_the_configured_template` | US-01-01-05 | Create Documents from Configured Templates |
| `doc_new_creates_a_document_from_the_configured_template` | US-01-01-10 | Define Schema-Backed Metadata Contracts |
| `doc_new_creates_a_document_from_the_configured_template` | US-01-01-13 | Govern Deterministic Document Creation via Templates |
| `doc_new_creates_a_document_from_the_configured_template` | US-01-01-20 | Create Documents from Template Files |
| `doc_new_creates_a_document_from_the_configured_template` | US-02-01-05 | Create Documents Quickly from Templates |
| `doc_new_creates_a_document_from_the_configured_template` | US-02-01-07 | Create Conforming Documents Quickly via Templates |
| `doc_new_creates_a_document_from_the_configured_template` | US-05-01-03 | Expose Machine-Usable Metadata Contracts |
| `doc_new_creates_a_document_from_the_configured_template` | US-06-01-03 | Create Architecture Documents from Templates |
| `doc_new_creates_a_document_from_the_configured_template` | US-08-01-05 | Create Documents Deterministically from Templates |
| `doc_new_creates_a_document_from_the_configured_template` | US-08-01-10 | Use Metadata Contracts Deterministically |
| `doc_new_creates_a_document_from_the_configured_template` | US-08-01-23 | Create Documents without Ambiguity via Templates |
| `doc_new_defaults_keep_the_current_id_shapes` | US-01-01-18 | Configure the ID Policy |
| `doc_new_dry_run_reports_the_plan_without_writing` | US-01-01-13 | Govern Deterministic Document Creation via Templates |
| `doc_new_dry_run_reports_the_plan_without_writing` | US-02-01-07 | Create Conforming Documents Quickly via Templates |
| `doc_new_dry_run_reports_the_plan_without_writing` | US-02-01-10 | Scaffold Translations During Implementation |
| `doc_new_dry_run_reports_the_plan_without_writing` | US-06-01-06 | Create Linked Translation Documents for Architecture Content |
| `doc_new_dry_run_reports_the_plan_without_writing` | US-08-01-14 | Scaffold Translations Deterministically from Source IDs |
| `doc_new_dry_run_reports_the_plan_without_writing` | US-08-01-23 | Create Documents without Ambiguity via Templates |
| `doc_new_fails_clearly_on_a_missing_template_file` | US-01-01-20 | Create Documents from Template Files |
| `doc_new_generates_a_unique_id_from_the_configured_policy` | US-01-01-13 | Govern Deterministic Document Creation via Templates |
| `doc_new_generates_a_unique_id_from_the_configured_policy` | US-02-01-07 | Create Conforming Documents Quickly via Templates |
| `doc_new_generates_a_unique_id_from_the_configured_policy` | US-08-01-23 | Create Documents without Ambiguity via Templates |
| `doc_new_generates_ids_from_the_configured_pattern` | US-01-01-18 | Configure the ID Policy |
| `doc_new_instantiates_the_configured_template_file` | US-01-01-20 | Create Documents from Template Files |
| `doc_new_rejects_a_kind_that_escapes_the_root` | US-01-01-07 | Enforce Scope Guardrails for Automation Agents |
| `doc_new_rejects_a_kind_that_escapes_the_root` | US-02-01-09 | Use Include Directives During Implementation |
| `doc_new_rejects_a_kind_that_escapes_the_root` | US-04-01-02 | Check Scope Guardrails in CI |
| `doc_new_rejects_a_kind_that_escapes_the_root` | US-05-01-04 | Parse Document Structure Deterministically for Automation |
| `doc_new_rejects_a_kind_that_escapes_the_root` | US-06-01-04 | Compose Modular Documents with Chapter and Include Directives |
| `doc_new_rejects_a_kind_that_escapes_the_root` | US-08-01-08 | Stay within Declared Change Scope |
| `doc_new_substitutes_the_title_into_the_template` | US-01-01-05 | Create Documents from Configured Templates |
| `doc_new_substitutes_the_title_into_the_template` | US-01-01-10 | Define Schema-Backed Metadata Contracts |
| `doc_new_substitutes_the_title_into_the_template` | US-01-01-13 | Govern Deterministic Document Creation via Templates |
| `doc_new_substitutes_the_title_into_the_template` | US-01-01-20 | Create Documents from Template Files |
| `doc_new_substitutes_the_title_into_the_template` | US-02-01-05 | Create Documents Quickly from Templates |
| `doc_new_substitutes_the_title_into_the_template` | US-02-01-07 | Create Conforming Documents Quickly via Templates |
| `doc_new_substitutes_the_title_into_the_template` | US-05-01-03 | Expose Machine-Usable Metadata Contracts |
| `doc_new_substitutes_the_title_into_the_template` | US-06-01-03 | Create Architecture Documents from Templates |
| `doc_new_substitutes_the_title_into_the_template` | US-08-01-05 | Create Documents Deterministically from Templates |
| `doc_new_substitutes_the_title_into_the_template` | US-08-01-10 | Use Metadata Contracts Deterministically |
| `doc_new_substitutes_the_title_into_the_template` | US-08-01-23 | Create Documents without Ambiguity via Templates |
| `doc_new_writes_into_the_configured_kind_location` | US-01-01-13 | Govern Deterministic Document Creation via Templates |
| `doc_new_writes_into_the_configured_kind_location` | US-02-01-07 | Create Conforming Documents Quickly via Templates |
| `doc_new_writes_into_the_configured_kind_location` | US-08-01-23 | Create Documents without Ambiguity via Templates |
| `doc_read_distinguishes_a_document_miss_from_a_selector_miss` | US-05-01-10 | Read Structured Document Sections with Stable Selectors |
| `doc_read_distinguishes_a_document_miss_from_a_selector_miss` | US-08-01-09 | Read Precise Document Sections for Scoped Execution |
| `doc_read_retrieves_a_document_by_id` | US-05-01-10 | Read Structured Document Sections with Stable Selectors |
| `doc_read_retrieves_a_document_by_id` | US-08-01-09 | Read Precise Document Sections for Scoped Execution |
| `doc_search_finds_documents_by_full_text` | US-02-01-06 | Find and Read Documentation During Implementation |
| `empty_link_cases_stay_visible_in_the_matrix` | US-03-01-02 | Export Trace Matrices |
| `failed_outcomes_demote_the_verifying_claim` | US-03-01-10 | Track Planned and Executed Test Evidence |
| `failure_diagnostics_name_the_stop_condition` | US-08-01-01 | Initialize a Doc Package Deterministically and Safely |
| `finalise_fails_clearly_on_unsupported_frontmatter` | US-01-01-06 | Finalise Document Metadata Mechanically |
| `finalise_fails_clearly_on_unsupported_frontmatter` | US-02-01-08 | Finalise Metadata without Touching Content |
| `finalise_fails_clearly_on_unsupported_frontmatter` | US-08-01-06 | Finalise Metadata Safely and Deterministically |
| `finalise_leaves_current_metadata_untouched` | US-01-01-06 | Finalise Document Metadata Mechanically |
| `finalise_leaves_current_metadata_untouched` | US-02-01-08 | Finalise Metadata without Touching Content |
| `finalise_leaves_current_metadata_untouched` | US-08-01-06 | Finalise Metadata Safely and Deterministically |
| `finalise_rejects_a_non_iso_date` | US-01-01-06 | Finalise Document Metadata Mechanically |
| `finalise_rejects_a_non_iso_date` | US-02-01-08 | Finalise Metadata without Touching Content |
| `finalise_rejects_a_non_iso_date` | US-08-01-06 | Finalise Metadata Safely and Deterministically |
| `finalise_sets_updated_to_the_injected_date` | US-01-01-06 | Finalise Document Metadata Mechanically |
| `finalise_sets_updated_to_the_injected_date` | US-02-01-08 | Finalise Metadata without Touching Content |
| `finalise_sets_updated_to_the_injected_date` | US-08-01-06 | Finalise Metadata Safely and Deterministically |
| `finalise_touches_only_the_meta_updated_field` | US-01-01-06 | Finalise Document Metadata Mechanically |
| `finalise_touches_only_the_meta_updated_field` | US-02-01-08 | Finalise Metadata without Touching Content |
| `finalise_touches_only_the_meta_updated_field` | US-08-01-06 | Finalise Metadata Safely and Deterministically |
| `fmt_and_config_show_share_one_contract_source` | US-01-01-19 | Configure Frontmatter Contracts |
| `fmt_is_idempotent` | US-01-01-03 | Format Documents Canonically |
| `fmt_is_idempotent` | US-01-01-04 | Lint Documents Deterministically |
| `fmt_is_idempotent` | US-01-01-08 | Generate Governed Coverage Reports |
| `fmt_is_idempotent` | US-01-01-10 | Define Schema-Backed Metadata Contracts |
| `fmt_is_idempotent` | US-01-01-12 | Govern Glossary Term Metadata and IDs |
| `fmt_is_idempotent` | US-01-01-16 | Validate Repository Configuration and Inspect Effective Config |
| `fmt_is_idempotent` | US-02-01-03 | Format Documents During Implementation |
| `fmt_is_idempotent` | US-02-01-04 | Lint Documents Before Commit |
| `fmt_is_idempotent` | US-02-01-06 | Find and Read Documentation During Implementation |
| `fmt_is_idempotent` | US-02-01-09 | Use Include Directives During Implementation |
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
| `fmt_keeps_diffs_focused_on_content` | US-02-01-03 | Format Documents During Implementation |
| `fmt_never_changes_body_text` | US-01-01-03 | Format Documents Canonically |
| `fmt_never_changes_body_text` | US-02-01-03 | Format Documents During Implementation |
| `fmt_never_changes_body_text` | US-08-01-03 | Format Documents Deterministically within Scope |
| `fmt_orders_keys_from_the_configured_contract` | US-01-01-19 | Configure Frontmatter Contracts |
| `fmt_orders_ontology_frontmatter_by_family` | US-01-01-03 | Format Documents Canonically |
| `fmt_orders_ontology_frontmatter_by_family` | US-02-01-03 | Format Documents During Implementation |
| `fmt_orders_ontology_frontmatter_by_family` | US-08-01-03 | Format Documents Deterministically within Scope |
| `fmt_sorts_frontmatter_keys_canonically` | US-01-01-03 | Format Documents Canonically |
| `fmt_sorts_frontmatter_keys_canonically` | US-02-01-03 | Format Documents During Implementation |
| `fmt_sorts_frontmatter_keys_canonically` | US-08-01-03 | Format Documents Deterministically within Scope |
| `format_option_is_accepted_globally` | US-04-01-10 | Emit Machine-Readable Diagnostics for CI |
| `format_option_is_accepted_globally` | US-05-01-14 | Emit Machine-Readable Diagnostics for Downstream Tooling |
| `format_option_is_accepted_globally` | US-08-01-21 | Emit Machine-Readable Diagnostics for Agent Workflows |
| `freshness_degrades_without_version_control` | US-03-01-11 | Keep Verified Status Current |
| `freshness_excludes_ignored_skeleton_markers` | US-03-01-11 | Keep Verified Status Current |
| `freshness_flags_a_marker_whose_requirement_changed_after_the_test` | US-03-01-11 | Keep Verified Status Current |
| `freshness_flags_a_marker_whose_requirement_is_newer` | US-03-01-11 | Keep Verified Status Current |
| `freshness_ignores_owning_story_churn` | US-03-01-11 | Keep Verified Status Current |
| `freshness_is_clean_when_the_test_is_the_later_commit` | US-03-01-11 | Keep Verified Status Current |
| `freshness_is_silent_when_the_marker_is_newer` | US-03-01-11 | Keep Verified Status Current |
| `freshness_treats_missing_history_as_fresh` | US-03-01-11 | Keep Verified Status Current |
| `ids_and_slugs_derive_deterministically` | US-01-01-01 | Initialise Standardised Doc Package |
| `ids_and_slugs_derive_deterministically` | US-01-01-04 | Lint Documents Deterministically |
| `ids_and_slugs_derive_deterministically` | US-01-01-05 | Create Documents from Configured Templates |
| `ids_and_slugs_derive_deterministically` | US-01-01-12 | Govern Glossary Term Metadata and IDs |
| `ids_and_slugs_derive_deterministically` | US-01-01-18 | Configure the ID Policy |
| `ids_and_slugs_derive_deterministically` | US-02-01-01 | Initialize a Doc Package with One Command |
| `ids_and_slugs_derive_deterministically` | US-02-01-04 | Lint Documents Before Commit |
| `ids_and_slugs_derive_deterministically` | US-02-01-05 | Create Documents Quickly from Templates |
| `ids_and_slugs_derive_deterministically` | US-03-01-01 | Lint Documents for Traceability Gaps |
| `ids_and_slugs_derive_deterministically` | US-05-01-10 | Read Structured Document Sections with Stable Selectors |
| `ids_and_slugs_derive_deterministically` | US-06-01-03 | Create Architecture Documents from Templates |
| `ids_and_slugs_derive_deterministically` | US-06-01-10 | Create Glossary Terms with Stable IDs |
| `ids_and_slugs_derive_deterministically` | US-08-01-01 | Initialize a Doc Package Deterministically and Safely |
| `ids_and_slugs_derive_deterministically` | US-08-01-04 | Lint Documents with Actionable Diagnostics |
| `ids_and_slugs_derive_deterministically` | US-08-01-05 | Create Documents Deterministically from Templates |
| `ids_and_slugs_derive_deterministically` | US-08-01-09 | Read Precise Document Sections for Scoped Execution |
| `include_directives_parse_with_and_without_level_arguments` | US-02-01-09 | Use Include Directives During Implementation |
| `includes_never_resolve_outside_the_configured_roots` | US-02-01-09 | Use Include Directives During Implementation |
| `init_alias_mirrors_doc_init` | US-01-01-01 | Initialise Standardised Doc Package |
| `lint_checks_encoded_groups_against_declared_triples` | US-01-01-18 | Configure the ID Policy |
| `lint_flags_a_lifecycle_status_outside_the_natures_vocabulary` | US-03-01-09 | Machine-Check the Done Claim |
| `lint_reports_each_unverified_requirement_of_a_done_story` | US-03-01-09 | Machine-Check the Done Claim |
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
| `lint_validates_id_shape_against_the_configured_pattern` | US-01-01-18 | Configure the ID Policy |
| `mcp_list_supports_a_lifecycle_filter` | US-05-01-12 | Expose Arqix via MCP over STDIO |
| `mcp_list_supports_a_lifecycle_filter` | US-08-01-12 | Use MCP Tools Deterministically in Agent Workflows |
| `mcp_search_supports_kind_and_path_filters` | US-05-01-12 | Expose Arqix via MCP over STDIO |
| `mcp_search_supports_kind_and_path_filters` | US-08-01-12 | Use MCP Tools Deterministically in Agent Workflows |
| `mcp_serve_exposes_search_read_and_list_tools` | US-05-01-12 | Expose Arqix via MCP over STDIO |
| `mcp_serve_exposes_search_read_and_list_tools` | US-08-01-12 | Use MCP Tools Deterministically in Agent Workflows |
| `mcp_serve_speaks_jsonrpc_over_stdio` | US-05-01-12 | Expose Arqix via MCP over STDIO |
| `mcp_serve_speaks_jsonrpc_over_stdio` | US-08-01-12 | Use MCP Tools Deterministically in Agent Workflows |
| `mcp_trace_answers_coverage_for_a_requirement_and_a_story` | US-05-01-12 | Expose Arqix via MCP over STDIO |
| `mcp_trace_answers_coverage_for_a_requirement_and_a_story` | US-08-01-12 | Use MCP Tools Deterministically in Agent Workflows |
| `mcp_trace_reports_an_unknown_id_as_a_tool_error` | US-05-01-12 | Expose Arqix via MCP over STDIO |
| `mcp_trace_reports_an_unknown_id_as_a_tool_error` | US-08-01-12 | Use MCP Tools Deterministically in Agent Workflows |
| `mutating_commands_leave_files_outside_the_roots_untouched` | US-01-01-07 | Enforce Scope Guardrails for Automation Agents |
| `mutating_commands_leave_files_outside_the_roots_untouched` | US-04-01-02 | Check Scope Guardrails in CI |
| `mutating_commands_leave_files_outside_the_roots_untouched` | US-08-01-08 | Stay within Declared Change Scope |
| `pdf_staging_promotes_the_leading_heading_and_omits_title_yaml` | US-04-01-03 | Generate Publishing Outputs |
| `policy_check_evaluates_changed_files_against_the_declared_scope` | US-01-01-07 | Enforce Scope Guardrails for Automation Agents |
| `policy_check_evaluates_changed_files_against_the_declared_scope` | US-04-01-02 | Check Scope Guardrails in CI |
| `policy_check_evaluates_changed_files_against_the_declared_scope` | US-08-01-08 | Stay within Declared Change Scope |
| `policy_check_passes_when_no_policy_is_declared` | US-01-01-07 | Enforce Scope Guardrails for Automation Agents |
| `policy_check_passes_when_no_policy_is_declared` | US-04-01-02 | Check Scope Guardrails in CI |
| `policy_check_passes_when_no_policy_is_declared` | US-08-01-08 | Stay within Declared Change Scope |
| `policy_check_reads_the_declared_scope_from_the_policy_file` | US-01-01-07 | Enforce Scope Guardrails for Automation Agents |
| `policy_check_reads_the_declared_scope_from_the_policy_file` | US-04-01-02 | Check Scope Guardrails in CI |
| `policy_check_reads_the_declared_scope_from_the_policy_file` | US-08-01-08 | Stay within Declared Change Scope |
| `policy_check_reports_violations_as_structured_diagnostics` | US-01-01-07 | Enforce Scope Guardrails for Automation Agents |
| `policy_check_reports_violations_as_structured_diagnostics` | US-01-01-08 | Generate Governed Coverage Reports |
| `policy_check_reports_violations_as_structured_diagnostics` | US-01-01-14 | Lint Translation Metadata and Drift |
| `policy_check_reports_violations_as_structured_diagnostics` | US-03-01-03 | Generate Coverage Reports |
| `policy_check_reports_violations_as_structured_diagnostics` | US-03-01-05 | Scan Traceability Information |
| `policy_check_reports_violations_as_structured_diagnostics` | US-03-01-06 | Detect Missing Trace Markers for Quality Gaps |
| `policy_check_reports_violations_as_structured_diagnostics` | US-03-01-08 | Make Trace and Coverage Outputs Reproducible |
| `policy_check_reports_violations_as_structured_diagnostics` | US-04-01-01 | Emit a CI-Friendly Assembly Log |
| `policy_check_reports_violations_as_structured_diagnostics` | US-04-01-02 | Check Scope Guardrails in CI |
| `policy_check_reports_violations_as_structured_diagnostics` | US-04-01-04 | Gate Bilingual Documentation Quality in CI |
| `policy_check_reports_violations_as_structured_diagnostics` | US-04-01-05 | Run a One-Command Verification Loop |
| `policy_check_reports_violations_as_structured_diagnostics` | US-04-01-07 | Publish Language-Aware Sites |
| `policy_check_reports_violations_as_structured_diagnostics` | US-04-01-10 | Emit Machine-Readable Diagnostics for CI |
| `policy_check_reports_violations_as_structured_diagnostics` | US-04-01-12 | Publish Stable Report Exports for Automation |
| `policy_check_reports_violations_as_structured_diagnostics` | US-05-01-02 | Emit a Machine-Readable Assembly Log |
| `policy_check_reports_violations_as_structured_diagnostics` | US-05-01-05 | Detect Translation Drift for Automation |
| `policy_check_reports_violations_as_structured_diagnostics` | US-05-01-07 | Build Machine-Readable Trace Graphs |
| `policy_check_reports_violations_as_structured_diagnostics` | US-05-01-08 | Export a Deterministic Document Catalog |
| `policy_check_reports_violations_as_structured_diagnostics` | US-05-01-13 | Expose Language-Aware Site Outputs Deterministically |
| `policy_check_reports_violations_as_structured_diagnostics` | US-05-01-14 | Emit Machine-Readable Diagnostics for Downstream Tooling |
| `policy_check_reports_violations_as_structured_diagnostics` | US-06-01-02 | Trace Document Assembly Structure |
| `policy_check_reports_violations_as_structured_diagnostics` | US-07-01-01 | Review Coverage Evidence |
| `policy_check_reports_violations_as_structured_diagnostics` | US-07-01-04 | Review Trace Graphs as Audit Evidence |
| `policy_check_reports_violations_as_structured_diagnostics` | US-07-01-06 | Export Deterministic Trace and Coverage Evidence |
| `policy_check_reports_violations_as_structured_diagnostics` | US-07-01-07 | Publish Stable Compliance-Ready Report Exports |
| `policy_check_reports_violations_as_structured_diagnostics` | US-08-01-02 | Emit a Deterministic Assembly Log for Verification |
| `policy_check_reports_violations_as_structured_diagnostics` | US-08-01-07 | List Documents Deterministically for Automation |
| `policy_check_reports_violations_as_structured_diagnostics` | US-08-01-08 | Stay within Declared Change Scope |
| `policy_check_reports_violations_as_structured_diagnostics` | US-08-01-11 | Interpret i18n Lint Results Deterministically |
| `policy_check_reports_violations_as_structured_diagnostics` | US-08-01-13 | Run One-Command Verification in Agent Workflows |
| `policy_check_reports_violations_as_structured_diagnostics` | US-08-01-16 | Scan Traceability Deterministically within Verification Loops |
| `policy_check_reports_violations_as_structured_diagnostics` | US-08-01-19 | Detect Missing Trace Markers for a Requirement |
| `policy_check_reports_violations_as_structured_diagnostics` | US-08-01-21 | Emit Machine-Readable Diagnostics for Agent Workflows |
| `policy_check_reports_violations_as_structured_diagnostics` | US-08-01-22 | Make Trace and Coverage Outputs Deterministic |
| `policy_check_supports_warn_only_mode` | US-01-01-07 | Enforce Scope Guardrails for Automation Agents |
| `policy_check_supports_warn_only_mode` | US-04-01-02 | Check Scope Guardrails in CI |
| `policy_check_supports_warn_only_mode` | US-08-01-08 | Stay within Declared Change Scope |
| `processed_content_is_never_executed` | US-01-01-07 | Enforce Scope Guardrails for Automation Agents |
| `processed_content_is_never_executed` | US-04-01-02 | Check Scope Guardrails in CI |
| `processed_content_is_never_executed` | US-08-01-08 | Stay within Declared Change Scope |
| `publish_site_diagnoses_a_failing_toolchain` | US-04-01-07 | Publish Language-Aware Sites |
| `publish_site_diagnoses_a_failing_toolchain` | US-05-01-13 | Expose Language-Aware Site Outputs Deterministically |
| `publish_site_generates_outputs_for_the_configured_target` | US-04-01-03 | Generate Publishing Outputs |
| `publish_site_generates_outputs_for_the_configured_target` | US-06-01-05 | Generate Publishable Documentation Outputs |
| `publish_site_orchestrates_the_configured_toolchain` | US-04-01-03 | Generate Publishing Outputs |
| `publish_site_orchestrates_the_configured_toolchain` | US-06-01-05 | Generate Publishable Documentation Outputs |
| `publish_site_publishes_per_language` | US-04-01-07 | Publish Language-Aware Sites |
| `publish_site_publishes_per_language` | US-05-01-13 | Expose Language-Aware Site Outputs Deterministically |
| `publish_site_stages_artefact_ready_inputs` | US-04-01-03 | Generate Publishing Outputs |
| `publish_site_stages_artefact_ready_inputs` | US-06-01-05 | Generate Publishable Documentation Outputs |
| `publish_site_stages_catalogue_pages_per_workflow_group` | US-04-01-17 | Publish the Specification as Catalogue Pages |
| `release_documents_stay_consistent_with_the_crate_version` | US-01-01-15 | Operationalise the Release Process with SemVer |
| `release_documents_stay_consistent_with_the_crate_version` | US-04-01-09 | Run Governed Release Preparation Workflows |
| `release_documents_stay_consistent_with_the_crate_version` | US-08-01-17 | Prepare Releases within Explicit Automation Boundaries |
| `release_process_documents_semver_and_the_versioned_contracts` | US-01-01-15 | Operationalise the Release Process with SemVer |
| `release_process_documents_semver_and_the_versioned_contracts` | US-04-01-09 | Run Governed Release Preparation Workflows |
| `release_process_documents_semver_and_the_versioned_contracts` | US-08-01-17 | Prepare Releases within Explicit Automation Boundaries |
| `render_forwards_tool_errors_transparently` | US-04-01-03 | Generate Publishing Outputs |
| `render_forwards_tool_errors_transparently` | US-06-01-05 | Generate Publishable Documentation Outputs |
| `render_pdf_accepts_selected_markdown_files` | US-04-01-03 | Generate Publishing Outputs |
| `render_pdf_accepts_selected_markdown_files` | US-06-01-05 | Generate Publishable Documentation Outputs |
| `render_pdf_drops_included_fragments_from_a_document` | US-04-01-03 | Generate Publishing Outputs |
| `render_pdf_passes_each_document_title_as_metadata` | US-04-01-03 | Generate Publishing Outputs |
| `render_pdf_produces_one_artefact_per_document` | US-04-01-03 | Generate Publishing Outputs |
| `render_pdf_renders_via_pandoc` | US-04-01-03 | Generate Publishing Outputs |
| `render_pdf_renders_via_pandoc` | US-06-01-05 | Generate Publishable Documentation Outputs |
| `render_pdf_stores_artefacts_per_configured_mode` | US-04-01-03 | Generate Publishing Outputs |
| `render_pdf_stores_artefacts_per_configured_mode` | US-06-01-05 | Generate Publishable Documentation Outputs |
| `render_pdf_supports_defaults_eisvogel_and_package_overrides` | US-04-01-03 | Generate Publishing Outputs |
| `render_pdf_supports_defaults_eisvogel_and_package_overrides` | US-06-01-05 | Generate Publishable Documentation Outputs |
| `report_bundle_exports_an_evidence_bundle_by_id_scope` | US-03-01-04 | Export Scoped Evidence Bundles for Quality Review |
| `report_bundle_includes_linked_evidence` | US-03-01-04 | Export Scoped Evidence Bundles for Quality Review |
| `report_bundle_output_is_deterministic_and_schema_stable` | US-04-01-12 | Publish Stable Report Exports for Automation |
| `report_bundle_output_is_deterministic_and_schema_stable` | US-07-01-07 | Publish Stable Compliance-Ready Report Exports |
| `report_bundle_records_generation_metadata` | US-04-01-12 | Publish Stable Report Exports for Automation |
| `report_bundle_records_generation_metadata` | US-07-01-07 | Publish Stable Compliance-Ready Report Exports |
| `report_bundle_resolves_a_story_scope_to_its_requirements` | US-03-01-04 | Export Scoped Evidence Bundles for Quality Review |
| `report_bundle_writes_reviewable_markdown_csv_and_json` | US-03-01-04 | Export Scoped Evidence Bundles for Quality Review |
| `report_bundle_writes_reviewable_markdown_csv_and_json` | US-04-01-12 | Publish Stable Report Exports for Automation |
| `report_bundle_writes_reviewable_markdown_csv_and_json` | US-07-01-07 | Publish Stable Compliance-Ready Report Exports |
| `report_knowledge_exports_an_okf_bundle_with_mapped_fields` | US-05-01-15 | Export the Corpus as an OKF Knowledge Bundle |
| `report_knowledge_honours_scope_lifecycle_and_determinism` | US-05-01-15 | Export the Corpus as an OKF Knowledge Bundle |
| `scaffolded_documents_satisfy_the_default_meta_contract` | US-01-01-19 | Configure Frontmatter Contracts |
| `search_answers_within_a_second_on_a_thousand_documents` | US-02-01-06 | Find and Read Documentation During Implementation |
| `search_answers_within_a_second_on_a_thousand_documents` | US-05-01-06 | Search and Read Documentation via CLI |
| `search_answers_within_a_second_on_a_thousand_documents` | US-06-01-09 | Retrieve Architecture Documentation Quickly |
| `staged_pages_do_not_duplicate_the_title_heading` | US-04-01-03 | Generate Publishing Outputs |
| `staged_pages_do_not_duplicate_the_title_heading` | US-06-01-05 | Generate Publishable Documentation Outputs |
| `templates_and_validation_share_the_contract_source` | US-01-01-10 | Define Schema-Backed Metadata Contracts |
| `templates_and_validation_share_the_contract_source` | US-05-01-03 | Expose Machine-Usable Metadata Contracts |
| `templates_and_validation_share_the_contract_source` | US-08-01-10 | Use Metadata Contracts Deterministically |
| `the_assembly_log_is_a_collectable_artefact` | US-04-01-01 | Emit a CI-Friendly Assembly Log |
| `the_verification_loop_completes_within_ten_seconds_on_a_thousand_documents` | US-04-01-05 | Run a One-Command Verification Loop |
| `the_verification_loop_completes_within_ten_seconds_on_a_thousand_documents` | US-08-01-13 | Run One-Command Verification in Agent Workflows |
| `tool_logic_answers_without_any_transport` | US-05-01-12 | Expose Arqix via MCP over STDIO |
| `tool_logic_answers_without_any_transport` | US-08-01-12 | Use MCP Tools Deterministically in Agent Workflows |
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
| `trace_coverage_output_is_deterministic` | US-02-01-09 | Use Include Directives During Implementation |
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
| `trace_ratchet_fails_on_a_coverage_regression` | US-04-01-15 | Gate Coverage as a Ratchet |
| `trace_ratchet_passes_without_regression` | US-04-01-15 | Gate Coverage as a Ratchet |
| `trace_ratchet_reads_the_configured_baseline` | US-04-01-16 | Configure the Snapshot Strategy |
| `trace_records_plans_markers_as_planned` | US-03-01-10 | Track Planned and Executed Test Evidence |
| `trace_resolves_ownership_from_triples_under_a_custom_pattern` | US-01-01-18 | Configure the ID Policy |
| `trace_scan_detects_markers_in_rust_comments` | US-03-01-05 | Scan Traceability Information |
| `trace_scan_does_not_follow_directory_symlinks` | US-01-01-03 | Format Documents Canonically |
| `trace_scan_does_not_follow_directory_symlinks` | US-01-01-04 | Lint Documents Deterministically |
| `trace_scan_does_not_follow_directory_symlinks` | US-01-01-08 | Generate Governed Coverage Reports |
| `trace_scan_does_not_follow_directory_symlinks` | US-01-01-10 | Define Schema-Backed Metadata Contracts |
| `trace_scan_does_not_follow_directory_symlinks` | US-01-01-12 | Govern Glossary Term Metadata and IDs |
| `trace_scan_does_not_follow_directory_symlinks` | US-01-01-16 | Validate Repository Configuration and Inspect Effective Config |
| `trace_scan_does_not_follow_directory_symlinks` | US-02-01-03 | Format Documents During Implementation |
| `trace_scan_does_not_follow_directory_symlinks` | US-02-01-04 | Lint Documents Before Commit |
| `trace_scan_does_not_follow_directory_symlinks` | US-02-01-06 | Find and Read Documentation During Implementation |
| `trace_scan_does_not_follow_directory_symlinks` | US-02-01-09 | Use Include Directives During Implementation |
| `trace_scan_does_not_follow_directory_symlinks` | US-02-01-11 | Assemble Documentation During Implementation |
| `trace_scan_does_not_follow_directory_symlinks` | US-03-01-01 | Lint Documents for Traceability Gaps |
| `trace_scan_does_not_follow_directory_symlinks` | US-03-01-02 | Export Trace Matrices |
| `trace_scan_does_not_follow_directory_symlinks` | US-03-01-03 | Generate Coverage Reports |
| `trace_scan_does_not_follow_directory_symlinks` | US-03-01-04 | Export Scoped Evidence Bundles for Quality Review |
| `trace_scan_does_not_follow_directory_symlinks` | US-03-01-07 | Filter Traceability Reports for Quality Analysis |
| `trace_scan_does_not_follow_directory_symlinks` | US-03-01-08 | Make Trace and Coverage Outputs Reproducible |
| `trace_scan_does_not_follow_directory_symlinks` | US-04-01-06 | Build Deterministic Page Artefacts in CI |
| `trace_scan_does_not_follow_directory_symlinks` | US-04-01-10 | Emit Machine-Readable Diagnostics for CI |
| `trace_scan_does_not_follow_directory_symlinks` | US-04-01-11 | Inspect Effective Config for CI Reproducibility |
| `trace_scan_does_not_follow_directory_symlinks` | US-05-01-03 | Expose Machine-Usable Metadata Contracts |
| `trace_scan_does_not_follow_directory_symlinks` | US-05-01-04 | Parse Document Structure Deterministically for Automation |
| `trace_scan_does_not_follow_directory_symlinks` | US-05-01-06 | Search and Read Documentation via CLI |
| `trace_scan_does_not_follow_directory_symlinks` | US-05-01-08 | Export a Deterministic Document Catalog |
| `trace_scan_does_not_follow_directory_symlinks` | US-05-01-09 | Observe Assembled Outputs for Downstream Tooling |
| `trace_scan_does_not_follow_directory_symlinks` | US-05-01-10 | Read Structured Document Sections with Stable Selectors |
| `trace_scan_does_not_follow_directory_symlinks` | US-05-01-11 | Consume Effective Configuration as Automation Baseline |
| `trace_scan_does_not_follow_directory_symlinks` | US-05-01-14 | Emit Machine-Readable Diagnostics for Downstream Tooling |
| `trace_scan_does_not_follow_directory_symlinks` | US-06-01-04 | Compose Modular Documents with Chapter and Include Directives |
| `trace_scan_does_not_follow_directory_symlinks` | US-06-01-08 | Assemble Modular Document Packages into Pages |
| `trace_scan_does_not_follow_directory_symlinks` | US-06-01-09 | Retrieve Architecture Documentation Quickly |
| `trace_scan_does_not_follow_directory_symlinks` | US-06-01-10 | Create Glossary Terms with Stable IDs |
| `trace_scan_does_not_follow_directory_symlinks` | US-07-01-01 | Review Coverage Evidence |
| `trace_scan_does_not_follow_directory_symlinks` | US-07-01-02 | Review Evidence Chains through Trace Matrices |
| `trace_scan_does_not_follow_directory_symlinks` | US-07-01-03 | Generate Audit Evidence Bundles by Scope |
| `trace_scan_does_not_follow_directory_symlinks` | US-07-01-05 | Filter Traceability Reports for Audit Review |
| `trace_scan_does_not_follow_directory_symlinks` | US-07-01-06 | Export Deterministic Trace and Coverage Evidence |
| `trace_scan_does_not_follow_directory_symlinks` | US-08-01-01 | Initialize a Doc Package Deterministically and Safely |
| `trace_scan_does_not_follow_directory_symlinks` | US-08-01-03 | Format Documents Deterministically within Scope |
| `trace_scan_does_not_follow_directory_symlinks` | US-08-01-04 | Lint Documents with Actionable Diagnostics |
| `trace_scan_does_not_follow_directory_symlinks` | US-08-01-06 | Finalise Metadata Safely and Deterministically |
| `trace_scan_does_not_follow_directory_symlinks` | US-08-01-07 | List Documents Deterministically for Automation |
| `trace_scan_does_not_follow_directory_symlinks` | US-08-01-09 | Read Precise Document Sections for Scoped Execution |
| `trace_scan_does_not_follow_directory_symlinks` | US-08-01-10 | Use Metadata Contracts Deterministically |
| `trace_scan_does_not_follow_directory_symlinks` | US-08-01-20 | Read Effective Config Deterministically Before Execution |
| `trace_scan_does_not_follow_directory_symlinks` | US-08-01-21 | Emit Machine-Readable Diagnostics for Agent Workflows |
| `trace_scan_does_not_follow_directory_symlinks` | US-08-01-22 | Make Trace and Coverage Outputs Deterministic |
| `trace_scan_outputs_the_trace_graph_as_json` | US-03-01-05 | Scan Traceability Information |
| `unit_new_creates_a_unit_from_the_configured_template` | US-01-01-05 | Create Documents from Configured Templates |
| `unit_new_creates_a_unit_from_the_configured_template` | US-01-01-10 | Define Schema-Backed Metadata Contracts |
| `unit_new_creates_a_unit_from_the_configured_template` | US-01-01-13 | Govern Deterministic Document Creation via Templates |
| `unit_new_creates_a_unit_from_the_configured_template` | US-01-01-20 | Create Documents from Template Files |
| `unit_new_creates_a_unit_from_the_configured_template` | US-02-01-05 | Create Documents Quickly from Templates |
| `unit_new_creates_a_unit_from_the_configured_template` | US-02-01-07 | Create Conforming Documents Quickly via Templates |
| `unit_new_creates_a_unit_from_the_configured_template` | US-05-01-03 | Expose Machine-Usable Metadata Contracts |
| `unit_new_creates_a_unit_from_the_configured_template` | US-06-01-03 | Create Architecture Documents from Templates |
| `unit_new_creates_a_unit_from_the_configured_template` | US-08-01-05 | Create Documents Deterministically from Templates |
| `unit_new_creates_a_unit_from_the_configured_template` | US-08-01-10 | Use Metadata Contracts Deterministically |
| `unit_new_creates_a_unit_from_the_configured_template` | US-08-01-23 | Create Documents without Ambiguity via Templates |
| `unit_new_help_explains_location_metadata_and_ids` | US-01-01-02 | Create Governed Units |
| `unit_new_help_explains_location_metadata_and_ids` | US-02-01-02 | Create Units Quickly During Implementation |
| `unit_new_help_explains_location_metadata_and_ids` | US-05-01-01 | Create Units for Retrieval and Automation |
| `unit_new_help_explains_location_metadata_and_ids` | US-06-01-01 | Create Modular Document Units |
| `unresolved_references_stay_visible_in_trace_outputs` | US-03-01-05 | Scan Traceability Information |
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
| `verify_defaults_to_informational_coverage_and_gating_rest` | US-04-01-14 | Configure the Verification Loop |
| `verify_emits_per_step_results_in_json_mode` | US-04-01-05 | Run a One-Command Verification Loop |
| `verify_emits_per_step_results_in_json_mode` | US-08-01-13 | Run One-Command Verification in Agent Workflows |
| `verify_excludes_rendering_from_the_default_loop` | US-04-01-05 | Run a One-Command Verification Loop |
| `verify_excludes_rendering_from_the_default_loop` | US-08-01-13 | Run One-Command Verification in Agent Workflows |
| `verify_reports_informational_findings_without_gating` | US-04-01-14 | Configure the Verification Loop |
| `verify_runs_exactly_the_configured_steps_in_order` | US-04-01-14 | Configure the Verification Loop |
| `verify_runs_freshness_as_an_informational_step` | US-03-01-11 | Keep Verified Status Current |
| `verify_runs_the_configured_sub_steps` | US-04-01-05 | Run a One-Command Verification Loop |
| `verify_runs_the_configured_sub_steps` | US-08-01-13 | Run One-Command Verification in Agent Workflows |
| `verify_supports_fail_fast_and_aggregate_modes` | US-04-01-05 | Run a One-Command Verification Loop |
| `verify_supports_fail_fast_and_aggregate_modes` | US-08-01-13 | Run One-Command Verification in Agent Workflows |
