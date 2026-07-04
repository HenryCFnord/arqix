//! Command contract: `trace scan`, `trace check`, `trace coverage`,
//! `trace matrix` — owned by the Trace Engine (arc42 chapter 5, ADR-0005:
//! coverage exists exactly once, as `trace coverage`).

mod common;

use common::{assert_success, fixture, run_arqix_in, stdout_json};

// arqix:verifies REQ-03-01-05-01
#[test]
#[ignore = "US-03-01-05: not implemented"]
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
#[ignore = "US-03-01-05: not implemented"]
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
#[ignore = "US-03-01-06: not implemented"]
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
#[ignore = "US-01-01-08: not implemented"]
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
#[ignore = "US-01-01-08: not implemented"]
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
#[ignore = "US-03-01-08: not implemented"]
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

// arqix:verifies REQ-03-01-02-01
#[test]
#[ignore = "US-03-01-02: not implemented"]
fn trace_matrix_exports_csv() {
    let out = run_arqix_in(&fixture("minimal"), &["trace", "matrix"]);
    assert_success(&out);
    let csv = String::from_utf8_lossy(&out.stdout);
    assert!(
        csv.lines().next().unwrap_or_default().contains(','),
        "expected a CSV header row"
    );
}
