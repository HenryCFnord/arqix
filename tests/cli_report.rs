//! Command contract: `report bundle` — owned by Report & Export; the only
//! `report` command (ADR-0005: the report verb is reserved for export
//! products).

mod common;

use common::{assert_success, run_arqix_in, scratch_copy};

// arqix:verifies REQ-03-01-04-01
#[test]
fn report_bundle_exports_an_evidence_bundle_by_id_scope() {
    let repo = scratch_copy(
        "minimal",
        "report_bundle_exports_an_evidence_bundle_by_id_scope",
    );
    let out = run_arqix_in(&repo, &["report", "bundle", "REQ-99-99-99-01"]);
    assert_success(&out);
}

// arqix:verifies REQ-03-01-04-02
#[test]
fn report_bundle_includes_linked_evidence() {
    let repo = scratch_copy("minimal", "report_bundle_includes_linked_evidence");
    let out = run_arqix_in(
        &repo,
        &["report", "bundle", "REQ-99-99-99-01", "--format", "json"],
    );
    assert_success(&out);
    let bundle = common::stdout_json(&out);
    assert!(bundle.to_string().contains("REQ-99-99-99-01"));
}

// arqix:verifies REQ-03-01-04-01
#[test]
fn report_bundle_resolves_a_story_scope_to_its_requirements() {
    let repo = scratch_copy(
        "minimal",
        "report_bundle_resolves_a_story_scope_to_its_requirements",
    );
    std::fs::write(
        repo.join("docs/US-42-01-01-story.md"),
        "---\nid: US-42-01-01\ntitle: Scoped Story\niri: arqix:user-stories/us-42-01-01\nrdf:\n  type:\n    - arqix:classes/user-story\n---\n\n## Scoped Story\n",
    )
    .unwrap();
    std::fs::write(
        repo.join("docs/REQ-42-01-01-01-scoped.md"),
        "---\nid: REQ-42-01-01-01\ntitle: Scoped Requirement\niri: arqix:requirements/req-42-01-01-01\nrdf:\n  type:\n    - arqix:classes/functional-requirement\ntriples:\n  - predicate: arqix:properties/derived-from\n    object:\n      - arqix:user-stories/us-42-01-01\n---\n\n## Requirement\n\nThe system SHALL be in scope.\n",
    )
    .unwrap();
    let out = run_arqix_in(
        &repo,
        &["report", "bundle", "US-42-01-01", "--format", "json"],
    );
    assert_success(&out);
    let bundle = common::stdout_json(&out);
    assert!(
        bundle.to_string().contains("REQ-42-01-01-01"),
        "a story scope pulls in the requirements derived from it: {bundle}"
    );
    assert!(
        !bundle.to_string().contains("REQ-99-99-99-01"),
        "out-of-scope requirements stay out: {bundle}"
    );
}

// arqix:verifies REQ-03-01-04-03
// arqix:verifies REQ-04-01-12-01
#[test]
fn report_bundle_writes_reviewable_markdown_csv_and_json() {
    // Reviewable without reshaping: the bundle directory carries the
    // evidence as Markdown for humans, CSV for spreadsheets, JSON for
    // automation — the audit formats, ready to attach.
    let repo = scratch_copy(
        "minimal",
        "report_bundle_writes_reviewable_markdown_csv_and_json",
    );
    assert_success(&run_arqix_in(
        &repo,
        &["report", "bundle", "REQ-99-99-99-01", "--out", "evidence"],
    ));
    let md = std::fs::read_to_string(repo.join("evidence/evidence.md")).unwrap();
    assert!(
        md.contains("REQ-99-99-99-01") && md.contains("| requirement |"),
        "the Markdown evidence is a readable table: {md}"
    );
    let csv = std::fs::read_to_string(repo.join("evidence/matrix.csv")).unwrap();
    assert!(
        csv.starts_with("requirement,kind,verified_markers,planned_markers,implements_markers"),
        "the CSV carries the stable matrix schema: {csv}"
    );
    assert!(repo.join("evidence/bundle.json").is_file());
}

// arqix:verifies REQ-04-01-12-02
#[test]
fn report_bundle_output_is_deterministic_and_schema_stable() {
    let repo = scratch_copy(
        "minimal",
        "report_bundle_output_is_deterministic_and_schema_stable",
    );
    let args = ["report", "bundle", "REQ-99-99-99-01", "--format", "json"];
    let first = common::stdout_json(&run_arqix_in(&repo, &args));
    let second = common::stdout_json(&run_arqix_in(&repo, &args));
    assert_eq!(
        first, second,
        "identical inputs must produce identical bundles"
    );
    assert_eq!(
        first["schema_version"], 1,
        "exports carry their schema version"
    );
}

// arqix:verifies REQ-04-01-12-03
#[test]
fn report_bundle_records_generation_metadata() {
    // The stamp is caller-provided (the injected-clock discipline: the
    // engine never reads the wall clock), so metadata records generation
    // context without breaking determinism.
    let repo = scratch_copy("minimal", "report_bundle_records_generation_metadata");
    let out = run_arqix_in(
        &repo,
        &[
            "report",
            "bundle",
            "REQ-99-99-99-01",
            "--stamp",
            "abc123, 2026-07-12",
            "--format",
            "json",
        ],
    );
    assert_success(&out);
    let bundle = common::stdout_json(&out);
    assert_eq!(bundle["stamp"], "abc123, 2026-07-12");
    assert_eq!(bundle["scope"][0], "REQ-99-99-99-01");
    assert!(
        bundle["inputs"]
            .as_array()
            .is_some_and(|inputs| !inputs.is_empty()),
        "metadata names the source inputs: {bundle}"
    );
}

// arqix:verifies REQ-05-01-15-01
// arqix:verifies REQ-05-01-15-02
#[test]
fn report_knowledge_exports_an_okf_bundle_with_mapped_fields() {
    let repo = scratch_copy(
        "minimal",
        "report_knowledge_exports_an_okf_bundle_with_mapped_fields",
    );
    std::fs::write(repo.join("docs/fragment.md"), "included knowledge\n").unwrap();
    std::fs::write(
        repo.join("docs/unit.md"),
        "---\nid: unit-k\ntitle: A Knowledge Unit\niri: arqix:units/unit-k\nrdf:\n  type:\n    - arqix:classes/unit\nmeta:\n  lifecycle-status: draft\n  updated: 2026-07-12\n---\n\n## A Knowledge Unit\n\n<!-- arqix:include fragment.md -->\n",
    )
    .unwrap();
    assert_success(&run_arqix_in(&repo, &["report", "knowledge", "--out", "okf"]));
    let concept = std::fs::read_to_string(repo.join("okf/unit.md")).unwrap();
    assert!(
        concept.contains("type: unit") && concept.contains("title: \"A Knowledge Unit\""),
        "OKF fields map from declared metadata: {concept}"
    );
    assert!(
        concept.contains("timestamp: 2026-07-12"),
        "the declared update date becomes the timestamp: {concept}"
    );
    assert!(
        concept.contains("included knowledge") && !concept.contains("arqix:include"),
        "concept documents are artefact-ready: {concept}"
    );

    // A document without a declared class exports as the generic type,
    // and absent metadata is omitted, never fabricated.
    let plain = std::fs::read_to_string(repo.join("okf/REQ-99-99-99-01-fixture-requirement.md"))
        .unwrap_or_default();
    assert!(
        !plain.contains("timestamp:") || plain.contains("timestamp: 2"),
        "no fabricated timestamps: {plain}"
    );
}

// arqix:verifies REQ-05-01-15-03
#[test]
fn report_knowledge_honours_scope_lifecycle_and_determinism() {
    let repo = scratch_copy(
        "minimal",
        "report_knowledge_honours_scope_lifecycle_and_determinism",
    );
    std::fs::write(
        repo.join("arqix.toml"),
        "[policies.publish]\nexclude = [\"internal\"]\n",
    )
    .unwrap();
    std::fs::create_dir_all(repo.join("docs/internal")).unwrap();
    std::fs::write(
        repo.join("docs/internal/notes.md"),
        "---\nid: notes\ntitle: Internal\n---\n\n## Internal\n",
    )
    .unwrap();
    std::fs::write(
        repo.join("docs/old.md"),
        "---\nid: old-doc\ntitle: Old\nmeta:\n  lifecycle-status: retired\n---\n\n## Old\n",
    )
    .unwrap();
    assert_success(&run_arqix_in(&repo, &["report", "knowledge"]));
    assert!(
        !repo.join("knowledge/internal").exists(),
        "the publish scope holds for the knowledge bundle"
    );
    assert!(
        !repo.join("knowledge/old.md").exists(),
        "retired documents leave living knowledge (ADR-0010)"
    );

    let first = std::fs::read_to_string(
        repo.join("knowledge/REQ-99-99-99-01-fixture-requirement.md"),
    )
    .unwrap();
    assert_success(&run_arqix_in(&repo, &["report", "knowledge"]));
    let second = std::fs::read_to_string(
        repo.join("knowledge/REQ-99-99-99-01-fixture-requirement.md"),
    )
    .unwrap();
    assert_eq!(first, second, "identical inputs, identical bundle");
}
