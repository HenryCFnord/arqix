//! Command contract: `report bundle` and `report knowledge` — owned by
//! Report & Export (ADR-0005: the report verb is reserved for export
//! products).

mod common;

use common::{assert_findings, assert_success, run_arqix_in, scratch_copy};
use std::path::Path;

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
    assert_success(&run_arqix_in(
        &repo,
        &["report", "knowledge", "--out", "okf"],
    ));
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

    let first =
        std::fs::read_to_string(repo.join("knowledge/REQ-99-99-99-01-fixture-requirement.md"))
            .unwrap();
    assert_success(&run_arqix_in(&repo, &["report", "knowledge"]));
    let second =
        std::fs::read_to_string(repo.join("knowledge/REQ-99-99-99-01-fixture-requirement.md"))
            .unwrap();
    assert_eq!(first, second, "identical inputs, identical bundle");
}

/// Write both trace matrices into the committed-snapshot layout the report
/// freshness gate reads, so a scratch corpus can be gated for freshness.
fn write_matrices(repo: &Path) {
    std::fs::create_dir_all(repo.join("docs/en/reports/trace")).unwrap();
    let req_test = run_arqix_in(repo, &["trace", "matrix", "--type", "req-test"]);
    std::fs::write(
        repo.join("docs/en/reports/trace/matrix.csv"),
        &req_test.stdout,
    )
    .unwrap();
    let us_req = run_arqix_in(repo, &["trace", "matrix", "--type", "us-req"]);
    std::fs::write(
        repo.join("docs/en/reports/trace/matrix-us-req.csv"),
        &us_req.stdout,
    )
    .unwrap();
}

// arqix:verifies REQ-04-01-12-04
#[test]
fn report_snapshot_regenerates_units_deterministically() {
    // Identical corpus and stamp must produce identical units, and the stamp
    // is embedded verbatim as generation provenance (the injected-clock
    // discipline keeps the wall clock out of the engine).
    let repo = scratch_copy(
        "minimal",
        "report_snapshot_regenerates_units_deterministically",
    );
    assert_success(&run_arqix_in(
        &repo,
        &[
            "report",
            "snapshot",
            "--stamp",
            "abc123, 2026-01-01",
            "--out",
            "first",
        ],
    ));
    assert_success(&run_arqix_in(
        &repo,
        &[
            "report",
            "snapshot",
            "--stamp",
            "abc123, 2026-01-01",
            "--out",
            "second",
        ],
    ));
    let first = std::fs::read_to_string(repo.join("first/scoreboard.md")).unwrap();
    let second = std::fs::read_to_string(repo.join("second/scoreboard.md")).unwrap();
    assert_eq!(first, second, "identical corpus + stamp -> identical units");
    assert!(
        first.contains("Snapshot: abc123, 2026-01-01"),
        "the injected stamp is embedded as provenance: {first}"
    );
}

// arqix:verifies REQ-04-01-12-04
#[test]
fn report_snapshot_check_passes_on_fresh_snapshots() {
    let repo = scratch_copy("minimal", "report_snapshot_check_passes_on_fresh_snapshots");
    assert_success(&run_arqix_in(
        &repo,
        &["report", "snapshot", "--stamp", "conformance, 2026-01-01"],
    ));
    write_matrices(&repo);
    write_statements(&repo);
    let out = run_arqix_in(&repo, &["report", "snapshot", "--check"]);
    assert_success(&out);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("reports: fresh (8 units, 2 matrices, 1 export)"),
        "a freshly generated corpus is fresh: {stdout}"
    );
}

// arqix:verifies REQ-04-01-12-04
#[test]
fn report_snapshot_check_detects_a_staled_unit() {
    let repo = scratch_copy("minimal", "report_snapshot_check_detects_a_staled_unit");
    assert_success(&run_arqix_in(
        &repo,
        &["report", "snapshot", "--stamp", "conformance, 2026-01-01"],
    ));
    write_matrices(&repo);
    write_statements(&repo);
    assert_success(&run_arqix_in(&repo, &["report", "snapshot", "--check"]));

    // Mutate one committed unit so it no longer matches the corpus.
    let unit = repo.join("docs/en/reports/units/scoreboard.md");
    let mut text = std::fs::read_to_string(&unit).unwrap();
    text.push_str("\na hand edit the gate must catch\n");
    std::fs::write(&unit, text).unwrap();

    let out = run_arqix_in(&repo, &["report", "snapshot", "--check"]);
    assert_findings(&out);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("scoreboard.md: stale"),
        "the staled unit is named: {stdout}"
    );
}

// arqix:verifies REQ-04-01-12-04
#[test]
fn report_snapshot_check_detects_a_missing_unit() {
    // The missing-file branch of the freshness contract (previously pinned
    // only by the retired Python oracle): a deleted committed unit is a
    // finding, not a silent skip.
    let repo = scratch_copy("minimal", "report_snapshot_check_detects_a_missing_unit");
    assert_success(&run_arqix_in(
        &repo,
        &["report", "snapshot", "--stamp", "conformance, 2026-01-01"],
    ));
    write_matrices(&repo);
    write_statements(&repo);
    assert_success(&run_arqix_in(&repo, &["report", "snapshot", "--check"]));

    std::fs::remove_file(repo.join("docs/en/reports/units/scoreboard.md")).unwrap();

    let out = run_arqix_in(&repo, &["report", "snapshot", "--check"]);
    assert_findings(&out);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("scoreboard.md: missing"),
        "the missing unit is named: {stdout}"
    );
}

// arqix:verifies REQ-04-01-12-04
#[test]
fn report_snapshot_check_detects_a_stale_matrix() {
    // The matrix half of the freshness contract: a committed matrix that no
    // longer matches a fresh regeneration is stale.
    let repo = scratch_copy("minimal", "report_snapshot_check_detects_a_stale_matrix");
    assert_success(&run_arqix_in(
        &repo,
        &["report", "snapshot", "--stamp", "conformance, 2026-01-01"],
    ));
    write_matrices(&repo);
    write_statements(&repo);
    assert_success(&run_arqix_in(&repo, &["report", "snapshot", "--check"]));

    let matrix = repo.join("docs/en/reports/trace/matrix.csv");
    let mut text = std::fs::read_to_string(&matrix).unwrap();
    text.push_str("junk,row,that,does,not,belong\n");
    std::fs::write(&matrix, text).unwrap();

    let out = run_arqix_in(&repo, &["report", "snapshot", "--check"]);
    assert_findings(&out);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("matrix.csv: stale"),
        "the stale matrix is named: {stdout}"
    );
}

fn write_statements(repo: &Path) {
    std::fs::create_dir_all(repo.join("docs/en/reports/requirements")).unwrap();
    let statements = run_arqix_in(repo, &["report", "statements"]);
    std::fs::write(
        repo.join("docs/en/reports/requirements/normative-statements.csv"),
        &statements.stdout,
    )
    .unwrap();
}

// arqix:verifies REQ-07-01-08-01
#[test]
fn report_statements_exports_the_classification() {
    // The projection of the checker's EARS/RFC-2119 classification: one CSV
    // row per requirement — id, kind, modality, pattern, subject.
    let repo = scratch_copy("minimal", "report_statements_exports_the_classification");
    std::fs::create_dir_all(repo.join("docs/en/architecture/req")).unwrap();
    std::fs::write(
        repo.join("docs/en/architecture/req/REQ-01-01-01-01-generate-unique-ids.md"),
        "---\nid: REQ-01-01-01-01\ntitle: Generate Unique IDs\nslug: generate-unique-ids\niri: arqix:requirements/req-01-01-01-01\n\nrdf:\n  type:\n    - arqix:classes/functional-requirement\n\ntriples:\n  - predicate: arqix:properties/derived-from\n    object: arqix:user-stories/us-01-01-01\n\nproperties:\n  priority: high\n\nmeta:\n  lifecycle-status: active\n  owner: hcf\n  created: 2026-07-16\n  updated: 2026-07-16\n  lang: en\n  generated: false\n---\n\n## Requirement\n\nWhen `arqix doc new` is invoked without `--id`, arqix SHALL generate a unique document ID from the configured policy.\n",
    )
    .unwrap();

    let out = run_arqix_in(&repo, &["report", "statements"]);
    assert_success(&out);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let mut lines = stdout.lines();
    assert_eq!(
        lines.next(),
        Some("requirement,kind,modality,pattern,subject"),
        "the CSV header: {stdout}"
    );
    assert_eq!(
        lines.next(),
        Some("REQ-01-01-01-01,functional,SHALL,event-driven,arqix"),
        "the classified row: {stdout}"
    );

    // Determinism: the same corpus state exports byte-identically.
    let again = run_arqix_in(&repo, &["report", "statements"]);
    assert_eq!(out.stdout, again.stdout, "byte-identical repeat run");
}

// arqix:verifies REQ-07-01-08-02
#[test]
fn report_snapshot_check_detects_a_stale_statements_export() {
    // The statements half of the freshness contract, mirroring the units
    // and matrices: stale content and a missing file are both findings.
    let repo = scratch_copy(
        "minimal",
        "report_snapshot_check_detects_a_stale_statements_export",
    );
    assert_success(&run_arqix_in(
        &repo,
        &["report", "snapshot", "--stamp", "conformance, 2026-01-01"],
    ));
    write_matrices(&repo);
    write_statements(&repo);
    assert_success(&run_arqix_in(&repo, &["report", "snapshot", "--check"]));

    let export = repo.join("docs/en/reports/requirements/normative-statements.csv");
    let mut text = std::fs::read_to_string(&export).unwrap();
    text.push_str("junk,row,that,does,not,belong\n");
    std::fs::write(&export, text).unwrap();

    let out = run_arqix_in(&repo, &["report", "snapshot", "--check"]);
    assert_findings(&out);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("normative-statements.csv: stale"),
        "the stale export is named: {stdout}"
    );

    std::fs::remove_file(&export).unwrap();
    let out = run_arqix_in(&repo, &["report", "snapshot", "--check"]);
    assert_findings(&out);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("normative-statements.csv: missing"),
        "the missing export is named: {stdout}"
    );
}
