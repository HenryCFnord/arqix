//! Command contract: `trace ratchet` — owned by the Verification
//! Orchestrator strand (US-04-01-15). Deliberately outside the
//! `cli_trace` conformance suite: the ratchet consumes the
//! conformance-pinned graph plus a committed baseline, and the Python
//! oracle does not grow new verbs on its retirement path (arc42 ch. 8).

mod common;

use common::{assert_success, run_arqix_in};
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
        text.replace("lifecycle-status: active", "lifecycle-status: retired"),
    )
    .unwrap();
    assert_success(&run_arqix_in(&repo, &["trace", "ratchet"]));
}
