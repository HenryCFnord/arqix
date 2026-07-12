//! Command contract: `trace scan`, `trace check`, `trace coverage`,
//! `trace matrix` — owned by the Trace Engine (arc42 chapter 5, ADR-0005:
//! coverage exists exactly once, as `trace coverage`).

mod common;

use common::{assert_success, fixture, run_arqix_in, stdout_json};

// arqix:verifies REQ-03-01-05-01
#[test]
fn trace_scan_detects_markers_in_rust_comments() {
    let repo = common::scratch_copy("minimal", "trace_scan_detects_markers_in_rust_comments");
    std::fs::write(
        repo.join("lib.rs"),
        "// arqix:verifies REQ-99-99-99-01\nfn covered() {}\n",
    )
    .unwrap();

    let out = run_arqix_in(&repo, &["trace", "scan", "--format", "json"]);
    assert_success(&out);
    assert!(stdout_json(&out).to_string().contains("REQ-99-99-99-01"));
}

// arqix:verifies REQ-03-01-05-04
#[test]
fn trace_scan_outputs_the_trace_graph_as_json() {
    let out = run_arqix_in(&fixture("minimal"), &["trace", "scan", "--format", "json"]);
    assert_success(&out);
    let graph = stdout_json(&out);
    assert!(
        graph.is_object(),
        "the trace graph must be a JSON object: {graph}"
    );
}

// arqix:verifies REQ-03-01-06-02
#[test]
fn trace_check_reports_verifies_markers_per_requirement() {
    let out = run_arqix_in(
        &fixture("minimal"),
        &["trace", "check", "REQ-99-99-99-01", "--format", "json"],
    );
    assert_success(&out);
    assert!(stdout_json(&out).to_string().contains("verifies"));
}

// arqix:verifies REQ-01-01-08-01
#[test]
fn trace_coverage_identifies_requirements_without_verifying_tests() {
    let out = run_arqix_in(
        &fixture("minimal"),
        &["trace", "coverage", "--format", "json"],
    );
    // The fixture requirement has no verifying test: coverage reports it.
    common::assert_findings(&out);
    assert!(stdout_json(&out).to_string().contains("REQ-99-99-99-01"));
}

// arqix:verifies REQ-01-01-08-03
#[test]
fn trace_coverage_supports_json_output() {
    let out = run_arqix_in(
        &fixture("minimal"),
        &["trace", "coverage", "--format", "json"],
    );
    let coverage = stdout_json(&out);
    assert!(coverage.is_object());
}

// arqix:verifies REQ-00-00-00-01
#[test]
fn trace_coverage_output_is_deterministic() {
    let first = run_arqix_in(
        &fixture("minimal"),
        &["trace", "coverage", "--format", "json"],
    );
    let second = run_arqix_in(
        &fixture("minimal"),
        &["trace", "coverage", "--format", "json"],
    );
    assert_eq!(
        first.stdout, second.stdout,
        "identical input must yield identical bytes"
    );
}

// arqix:verifies REQ-00-00-00-01
#[cfg(unix)]
#[test]
fn trace_scan_does_not_follow_directory_symlinks() {
    let repo = common::scratch_copy("minimal", "trace_scan_does_not_follow_directory_symlinks");
    std::fs::create_dir_all(repo.join("docs/sub")).unwrap();
    // A parent symlink forms a cycle under the corpus walk; the Python
    // oracle's rglob does not follow directory symlinks, so the engine
    // must not either (unbounded walk, phantom duplicate paths).
    std::os::unix::fs::symlink("..", repo.join("docs/sub/up")).unwrap();

    let out = run_arqix_in(&repo, &["trace", "scan", "--format", "json"]);
    assert_success(&out);
    let graph = stdout_json(&out).to_string();
    assert!(
        !graph.contains("sub/up"),
        "paths reached through a directory symlink must not enter the graph: {graph}"
    );
}

// arqix:verifies REQ-03-01-02-01
#[test]
fn trace_matrix_exports_csv() {
    let out = run_arqix_in(&fixture("minimal"), &["trace", "matrix"]);
    assert_success(&out);
    let csv = String::from_utf8_lossy(&out.stdout);
    assert!(
        csv.lines().next().unwrap_or_default().contains(','),
        "expected a CSV header row"
    );
}

// arqix:verifies REQ-01-01-18-02
#[test]
fn trace_resolves_ownership_from_triples_under_a_custom_pattern() {
    // ADR-0012: the ID is an opaque label; a group-free pattern still
    // yields a complete graph because ownership comes from the declared
    // derived-from triple. Runs against the oracle too (ARQIX_BIN) — the
    // configured policy is one source for both implementations.
    let repo = common::scratch_copy(
        "minimal",
        "trace_resolves_ownership_from_triples_under_a_custom_pattern",
    );
    std::fs::write(
        repo.join("arqix.toml"),
        "[kinds.requirement]\ndir = \"docs\"\nid-pattern = '^R-(?P<seq>\\d{4})$'\n",
    )
    .unwrap();
    std::fs::write(
        repo.join("docs/story.md"),
        "---\nid: US-77-01-01\ntitle: Owning Story\niri: arqix:user-stories/us-77-01-01\nrdf:\n  type:\n    - arqix:classes/user-story\n---\n\n## Owning Story\n",
    )
    .unwrap();
    std::fs::write(
        repo.join("docs/R-0007.md"),
        "---\nid: R-0007\ntitle: Opaque Label\niri: arqix:requirements/r-0007\nrdf:\n  type:\n    - arqix:classes/functional-requirement\ntriples:\n  - predicate: arqix:properties/derived-from\n    object:\n      - arqix:user-stories/us-77-01-01\n---\n\n## Requirement\n\nThe system SHALL work with opaque labels.\n",
    )
    .unwrap();
    std::fs::write(
        repo.join("tests.rs"),
        "// arqix:verifies R-0007\n#[test]\nfn opaque_covered() {}\n\n// arqix:verifies REQ-99-99-99-01\n#[test]\nfn fixture_covered() {}\n",
    )
    .unwrap();

    let out = run_arqix_in(&repo, &["trace", "coverage", "--format", "json"]);
    assert_success(&out);
    let coverage = stdout_json(&out);
    let row = coverage["requirements"]
        .as_array()
        .and_then(|rows| rows.iter().find(|r| r["id"] == "R-0007"))
        .cloned()
        .unwrap_or_default();
    assert_eq!(
        row["story"], "US-77-01-01",
        "ownership comes from the declared triple, not the ID: {coverage}"
    );
    assert!(
        row["verified_by"].as_array().is_some_and(|v| !v.is_empty()),
        "the marker with a non-default payload counts: {coverage}"
    );
}

// arqix:verifies REQ-03-01-10-01
#[test]
fn trace_records_plans_markers_as_planned() {
    // The language-neutral planned claim: no #[ignore], no framework skip
    // syntax — the marker alone declares the intent, in Rust comments and
    // Markdown HTML comments alike.
    let repo = common::scratch_copy("minimal", "trace_records_plans_markers_as_planned");
    std::fs::write(
        repo.join("docs/REQ-88-01-01-01-planned.md"),
        "---\nid: REQ-88-01-01-01\ntitle: A Planned Requirement\niri: arqix:requirements/req-88-01-01-01\nrdf:\n  type:\n    - arqix:classes/functional-requirement\n---\n\n## Requirement\n\nThe system SHALL be planned first.\n",
    )
    .unwrap();
    std::fs::write(
        repo.join("skeleton.rs"),
        "// arqix:plans REQ-88-01-01-01\nfn a_future_test() {}\n\n// arqix:verifies REQ-99-99-99-01\n#[test]\nfn fixture_covered() {}\n",
    )
    .unwrap();

    let out = run_arqix_in(&repo, &["trace", "coverage", "--format", "json"]);
    let coverage = stdout_json(&out);
    let row = coverage["requirements"]
        .as_array()
        .and_then(|rows| rows.iter().find(|r| r["id"] == "REQ-88-01-01-01"))
        .cloned()
        .unwrap_or_default();
    assert!(
        row["planned_by"].as_array().is_some_and(|p| !p.is_empty()),
        "a plans marker is a planned claim: {coverage}"
    );
    assert!(
        row["verified_by"].as_array().is_some_and(|v| v.is_empty()),
        "a plans marker never counts as verified: {coverage}"
    );

    // The scan graph carries the claim as its own edge kind.
    let out = run_arqix_in(&repo, &["trace", "scan", "--format", "json"]);
    let graph = stdout_json(&out);
    assert!(
        graph["edges"]
            .as_array()
            .is_some_and(|edges| edges.iter().any(|e| e["kind"] == "plans")),
        "the graph records the plans edge: {graph}"
    );
}
