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

// arqix:verifies REQ-04-01-15-01
#[test]
fn trace_ratchet_fails_on_a_coverage_regression() {
    let repo = common::scratch_copy("minimal", "trace_ratchet_fails_on_a_coverage_regression");
    // The committed baseline claims REQ-99-99-99-01 is verified; the corpus
    // has no active verifying test — someone lost a verification.
    std::fs::create_dir_all(repo.join("docs/en/reports/trace")).unwrap();
    std::fs::write(
        repo.join("docs/en/reports/trace/matrix.csv"),
        "requirement,kind,verified_markers,planned_markers,implements_markers\n\
         REQ-99-99-99-01,functional,tests/cli_gone.rs:12,,\n",
    )
    .unwrap();

    let out = run_arqix_in(&repo, &["trace", "ratchet"]);
    common::assert_findings(&out);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("REQ-99-99-99-01"),
        "the finding must name the regressed requirement: {stdout}"
    );
}

// arqix:verifies REQ-04-01-15-02
#[test]
fn trace_ratchet_passes_without_regression() {
    let repo = common::scratch_copy("minimal", "trace_ratchet_passes_without_regression");

    // No baseline: nothing to compare, never a failure.
    assert_success(&run_arqix_in(&repo, &["trace", "ratchet"]));

    // Pure specification growth: the baseline predates the requirement, the
    // requirement is uncovered — the ratchet stays green.
    std::fs::create_dir_all(repo.join("docs/en/reports/trace")).unwrap();
    let baseline = repo.join("docs/en/reports/trace/matrix.csv");
    std::fs::write(
        &baseline,
        "requirement,kind,verified_markers,planned_markers,implements_markers\n",
    )
    .unwrap();
    assert_success(&run_arqix_in(&repo, &["trace", "ratchet"]));

    // Retirement: the baseline claims a verification, but the requirement is
    // retired now — a declared intent change, never a regression.
    std::fs::write(
        &baseline,
        "requirement,kind,verified_markers,planned_markers,implements_markers\n\
         REQ-99-99-99-01,functional,tests/cli_gone.rs:12,,\n",
    )
    .unwrap();
    let req = repo.join("docs/REQ-99-99-99-01-fixture-requirement.md");
    let text = std::fs::read_to_string(&req).unwrap();
    std::fs::write(
        &req,
        text.replace("lifecycle-status: draft", "lifecycle-status: retired"),
    )
    .unwrap();
    assert_success(&run_arqix_in(&repo, &["trace", "ratchet"]));
}
