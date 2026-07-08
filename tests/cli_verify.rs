//! Command contract: `verify` — owned by the Verification Orchestrator
//! (arc42 chapter 5, ADR-0003): sequences the configured sub-steps through
//! the stable command interface, never implements a check itself.

mod common;

use common::{fixture, run_arqix_in, stdout_json};

// arqix:verifies REQ-04-01-05-01
#[test]
fn verify_runs_the_configured_sub_steps() {
    let out = run_arqix_in(&fixture("minimal"), &["verify"]);
    // Exit 0 (clean) or 1 (findings) — never a usage error or stub code.
    assert!(matches!(out.status.code(), Some(0) | Some(1)));
}

// arqix:verifies REQ-04-01-05-02
#[test]
fn verify_supports_fail_fast_and_aggregate_modes() {
    let fail_fast = run_arqix_in(&fixture("minimal"), &["verify", "--fail-fast"]);
    assert!(matches!(fail_fast.status.code(), Some(0) | Some(1)));

    let aggregate = run_arqix_in(&fixture("minimal"), &["verify", "--aggregate"]);
    assert!(matches!(aggregate.status.code(), Some(0) | Some(1)));
}

// arqix:verifies REQ-04-01-05-03
#[test]
fn verify_emits_per_step_results_in_json_mode() {
    let out = run_arqix_in(&fixture("minimal"), &["verify", "--format", "json"]);
    let results = stdout_json(&out);
    assert!(
        results.to_string().contains("step"),
        "JSON mode must carry per-step results: {results}"
    );
}

// arqix:verifies REQ-04-01-05-04
#[test]
fn verify_excludes_rendering_from_the_default_loop() {
    let out = run_arqix_in(&fixture("minimal"), &["verify", "--format", "json"]);
    let results = stdout_json(&out).to_string();
    assert!(
        !results.contains("render") && !results.contains("publish"),
        "rendering must never be part of the default loop: {results}"
    );
}
