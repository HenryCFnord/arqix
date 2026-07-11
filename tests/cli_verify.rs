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

// arqix:verifies REQ-04-01-14-01
#[test]
fn verify_runs_exactly_the_configured_steps_in_order() {
    let repo = common::scratch_copy(
        "minimal",
        "verify_runs_exactly_the_configured_steps_in_order",
    );
    std::fs::write(
        repo.join("arqix.toml"),
        "[policies.verify]\nsteps = [\"lint\", \"format\"]\n",
    )
    .unwrap();

    let out = run_arqix_in(&repo, &["verify", "--format", "json"]);
    let report = stdout_json(&out);
    let steps: Vec<&str> = report["steps"]
        .as_array()
        .expect("steps array")
        .iter()
        .map(|s| s["step"].as_str().unwrap_or("?"))
        .collect();
    assert_eq!(
        steps,
        ["lint", "format"],
        "verify must run exactly the configured sub-steps in their configured order"
    );
}

// arqix:verifies REQ-04-01-14-02
#[test]
fn verify_reports_informational_findings_without_gating() {
    let repo = common::scratch_copy(
        "minimal",
        "verify_reports_informational_findings_without_gating",
    );
    // The fixture requirement has no verifies marker, so coverage exits 1.
    std::fs::write(
        repo.join("arqix.toml"),
        "[policies.verify]\nsteps = [\"coverage\"]\ninformational = [\"coverage\"]\n",
    )
    .unwrap();

    let out = run_arqix_in(&repo, &["verify", "--format", "json"]);
    let report = stdout_json(&out);
    let step = &report["steps"][0];
    assert_eq!(
        step["exit_code"], 1,
        "the findings are still reported: {report}"
    );
    assert_eq!(
        step["informational"], true,
        "the step declares its channel: {report}"
    );
    assert_eq!(
        out.status.code(),
        Some(0),
        "an informational step must not affect the exit code"
    );

    // The same step configured as gating fails the loop.
    std::fs::write(
        repo.join("arqix.toml"),
        "[policies.verify]\nsteps = [\"coverage\"]\ninformational = []\n",
    )
    .unwrap();
    let gated = run_arqix_in(&repo, &["verify"]);
    assert_eq!(
        gated.status.code(),
        Some(1),
        "gating is the configured default channel"
    );
}

// arqix:verifies REQ-04-01-14-03
#[test]
fn verify_defaults_to_informational_coverage_and_gating_rest() {
    // No [policies.verify] table: the fixture's uncovered requirement makes
    // coverage exit 1, but the default treats coverage as informational.
    let repo = common::scratch_copy(
        "minimal",
        "verify_defaults_to_informational_coverage_and_gating_rest",
    );
    let out = run_arqix_in(&repo, &["verify", "--format", "json"]);
    let report = stdout_json(&out);
    assert_eq!(
        out.status.code(),
        Some(0),
        "coverage findings alone must not gate by default: {report}"
    );
    let coverage = report["steps"]
        .as_array()
        .expect("steps array")
        .iter()
        .find(|s| s["step"] == "coverage")
        .expect("coverage step present by default")
        .clone();
    assert_eq!(
        coverage["exit_code"], 1,
        "the fixture requirement is uncovered: {report}"
    );
    assert_eq!(
        coverage["informational"], true,
        "coverage is informational by default"
    );
}
