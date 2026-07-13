//! Command contract: `verify` — owned by the Verification Orchestrator
//! (arc42 chapter 5, ADR-0003): sequences the configured sub-steps through
//! the stable command interface, never implements a check itself.

mod common;

use common::{fixture, run_arqix_in, run_arqix_in_env, stdout_json};

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

// A structurally clean story/requirement pair the requirements checker accepts.
const CLEAN_STORY: &str = r#"---
id: US-09-09-09
title: Sample Story
slug: sample-story
iri: arqix:user-stories/us-09-09-09

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-requirement
    object: arqix:requirements/req-09-09-09-01

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-13
  updated: 2026-07-13
  lang: en
  generated: false
---

## Sample Story
"#;

const CLEAN_REQ: &str = r#"---
id: REQ-09-09-09-01
title: Sample
slug: sample
iri: arqix:requirements/req-09-09-09-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object: arqix:user-stories/us-09-09-09

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-13
  updated: 2026-07-13
  lang: en
  generated: false
---

## Requirement

The arqix CLI SHALL reject unknown flags.
"#;

// The same requirement with a forbidden RFC 2119 keyword — a `requirements`
// authoring violation (EARS-002/003/004).
const BAD_REQ: &str = r#"---
id: REQ-09-09-09-01
title: Sample
slug: sample
iri: arqix:requirements/req-09-09-09-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object: arqix:user-stories/us-09-09-09

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-13
  updated: 2026-07-13
  lang: en
  generated: false
---

## Requirement

The tool MUST reject unknown flags.
"#;

fn seed_clean_requirements(repo: &std::path::Path) {
    let story_dir = repo.join("docs/en/architecture/stories");
    let req_dir = repo.join("docs/en/architecture/req");
    std::fs::create_dir_all(&story_dir).unwrap();
    std::fs::create_dir_all(&req_dir).unwrap();
    std::fs::write(story_dir.join("US-09-09-09-sample.md"), CLEAN_STORY).unwrap();
    std::fs::write(req_dir.join("REQ-09-09-09-01-sample.md"), CLEAN_REQ).unwrap();
}

// arqix:verifies REQ-04-01-14-04
#[test]
fn verify_reports_the_corpus_checks_as_sub_steps() {
    // A profile that lists the ported corpus checks (as arqix's own arqix.toml
    // does) wires them into the loop so `arqix verify` alone covers the
    // reference sequencer's corpus steps. They are not in the hard-coded default
    // because they need a full corpus (a fresh package must still pass verify —
    // REQ-08-01-01-02).
    let repo = common::scratch_copy("minimal", "verify_reports_the_corpus_checks_as_sub_steps");
    std::fs::write(
        repo.join("arqix.toml"),
        "[policies.verify]\nsteps = [\"requirements\", \"frontmatter\", \"markers\", \"report-freshness\"]\n",
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
    for name in ["requirements", "frontmatter", "markers", "report-freshness"] {
        assert!(
            steps.contains(&name),
            "the configured profile must run the {name} corpus check: {report}"
        );
    }
}

// arqix:verifies REQ-04-01-14-04
#[test]
fn verify_gates_on_a_seeded_requirements_violation() {
    let repo = common::scratch_copy("minimal", "verify_gates_on_a_seeded_requirements_violation");
    seed_clean_requirements(&repo);
    std::fs::write(
        repo.join("arqix.toml"),
        "[policies.verify]\nsteps = [\"requirements\"]\n",
    )
    .unwrap();

    // A clean corpus passes the requirements sub-step — the gate is not vacuous.
    let clean = run_arqix_in(&repo, &["verify", "--format", "json"]);
    let report = stdout_json(&clean);
    assert_eq!(report["steps"][0]["step"], "requirements", "{report}");
    assert_eq!(
        report["steps"][0]["exit_code"], 0,
        "a clean corpus passes: {report}"
    );
    assert_eq!(clean.status.code(), Some(0), "{report}");

    // Seeding an authoring violation fails `verify` on the requirements step.
    std::fs::write(
        repo.join("docs/en/architecture/req/REQ-09-09-09-01-sample.md"),
        BAD_REQ,
    )
    .unwrap();
    let gated = run_arqix_in(&repo, &["verify", "--format", "json"]);
    let report = stdout_json(&gated);
    assert_eq!(
        report["steps"][0]["exit_code"], 1,
        "the seeded violation is a finding: {report}"
    );
    assert_eq!(
        gated.status.code(),
        Some(1),
        "verify gates on the requirements violation: {report}"
    );
}

// arqix:verifies REQ-04-01-14-04
#[test]
fn verify_gates_on_a_seeded_marker_violation() {
    let repo = common::scratch_copy("minimal", "verify_gates_on_a_seeded_marker_violation");
    std::fs::write(
        repo.join("arqix.toml"),
        "[policies.verify]\nsteps = [\"markers\"]\n",
    )
    .unwrap();

    // No test files yet: the marker gate has nothing to flag and passes.
    let clean = run_arqix_in(&repo, &["verify", "--format", "json"]);
    let report = stdout_json(&clean);
    assert_eq!(report["steps"][0]["step"], "markers", "{report}");
    assert_eq!(
        clean.status.code(),
        Some(0),
        "a clean tree passes: {report}"
    );

    // A test function with neither a marker nor a no-requirement annotation is
    // the canonical TDD-gate violation (TRC-002). Assembled as a single-line
    // literal so no physical line of this file is itself read as a marker.
    std::fs::create_dir_all(repo.join("tests")).unwrap();
    std::fs::write(
        repo.join("tests/sample.rs"),
        "#[test]\nfn needs_a_marker() {}\n",
    )
    .unwrap();
    let gated = run_arqix_in(&repo, &["verify", "--format", "json"]);
    let report = stdout_json(&gated);
    assert_eq!(
        report["steps"][0]["exit_code"], 1,
        "the marker-less test is a finding: {report}"
    );
    assert_eq!(
        gated.status.code(),
        Some(1),
        "verify gates on the marker violation: {report}"
    );
}

// arqix:verifies REQ-04-01-14-05
#[test]
fn verify_skips_report_freshness_off_the_default_branch_under_main_only() {
    // Under `main-only`, the report-freshness sub-step gates only on the
    // default branch; a parallel branch must skip it so a legitimately stale
    // snapshot never fails the loop. The stub corpus has no committed
    // snapshots, so the step would fail if it were run — proving the skip is
    // load-bearing, not cosmetic.
    let repo = common::scratch_copy(
        "minimal",
        "verify_skips_report_freshness_off_the_default_branch_under_main_only",
    );
    std::fs::write(
        repo.join("arqix.toml"),
        "[policies.reports]\nsnapshot-strategy = \"main-only\"\n\n\
         [policies.verify]\nsteps = [\"report-freshness\"]\n",
    )
    .unwrap();

    // Off the default branch: skipped, and the loop stays green.
    let skipped = run_arqix_in_env(
        &repo,
        &["verify", "--format", "json"],
        &[("GITHUB_REF_NAME", "feature/parallel")],
    );
    let report = stdout_json(&skipped);
    let step = &report["steps"][0];
    assert_eq!(step["step"], "report-freshness", "{report}");
    assert_eq!(
        step["skipped"], true,
        "report-freshness is skipped off the default branch under main-only: {report}"
    );
    assert_eq!(
        skipped.status.code(),
        Some(0),
        "a skipped step must not fail the loop: {report}"
    );

    // On the default branch, the same step runs and gates on the (missing)
    // snapshots — the strategy resolves to committed there.
    let gated = run_arqix_in_env(
        &repo,
        &["verify", "--format", "json"],
        &[("GITHUB_REF_NAME", "main")],
    );
    let report = stdout_json(&gated);
    let step = &report["steps"][0];
    assert_eq!(
        step["skipped"], false,
        "report-freshness runs on the default branch: {report}"
    );
    assert_eq!(
        step["exit_code"], 1,
        "the stub corpus has no committed snapshots: {report}"
    );
    assert_eq!(
        gated.status.code(),
        Some(1),
        "report-freshness gates on the default branch: {report}"
    );
}
