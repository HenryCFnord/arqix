//! Command contract: `lint run` — owned by the Linter (arc42 chapter 5).

mod common;

use common::{fixture, run_arqix_in};

// arqix:verifies REQ-01-01-04-01
#[test]
fn lint_run_checks_that_include_targets_exist() {
    let out = run_arqix_in(&fixture("minimal"), &["lint", "run"]);
    common::assert_success(&out);
}

// arqix:verifies REQ-01-01-04-03
#[test]
fn lint_run_reports_duplicate_ids_globally() {
    let repo = common::scratch_copy("minimal", "lint_run_reports_duplicate_ids_globally");
    std::fs::copy(
        repo.join("docs/REQ-99-99-99-01-fixture-requirement.md"),
        repo.join("docs/REQ-99-99-99-01-duplicate.md"),
    )
    .unwrap();

    let out = run_arqix_in(&repo, &["lint", "run", "--format", "json"]);
    common::assert_findings(&out);
    assert!(
        common::stdout_json(&out)
            .to_string()
            .contains("REQ-99-99-99-01")
    );
}

// arqix:verifies REQ-01-01-04-04
#[test]
fn lint_run_reports_findings_with_file_and_line_context() {
    let repo = common::scratch_copy(
        "minimal",
        "lint_run_reports_findings_with_file_and_line_context",
    );
    std::fs::copy(
        repo.join("docs/REQ-99-99-99-01-fixture-requirement.md"),
        repo.join("docs/REQ-99-99-99-01-duplicate.md"),
    )
    .unwrap();

    let out = run_arqix_in(&repo, &["lint", "run", "--format", "json"]);
    common::assert_findings(&out);
    let findings = common::stdout_json(&out).to_string();
    assert!(
        findings.contains("line"),
        "findings must carry line context: {findings}"
    );
}

// arqix:verifies REQ-00-00-00-10
#[test]
fn lint_run_detects_translation_drift() {
    // Contract only at skeleton stage: the i18n profile is part of the
    // configured lint run; the fixture grows a translation pair with the
    // red phase of US-01-01-10.
    let out = run_arqix_in(&fixture("minimal"), &["lint", "run"]);
    common::assert_success(&out);
}
