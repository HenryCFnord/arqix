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

// arqix:verifies REQ-03-01-09-02
#[test]
fn lint_flags_a_lifecycle_status_outside_the_natures_vocabulary() {
    let repo = common::scratch_copy(
        "minimal",
        "lint_flags_a_lifecycle_status_outside_the_natures_vocabulary",
    );
    // Requirements declare only active/retired (ADR-0010): the gate refutes
    // draft, so a requirement declaring it is a vocabulary error.
    let req = repo.join("docs/REQ-99-99-99-01-fixture-requirement.md");
    let text = std::fs::read_to_string(&req).unwrap();
    std::fs::write(
        &req,
        text.replace("lifecycle-status: active", "lifecycle-status: draft"),
    )
    .unwrap();

    let out = run_arqix_in(&repo, &["lint", "run", "--format", "json"]);
    common::assert_findings(&out);
    let findings = common::stdout_json(&out).to_string();
    assert!(
        findings.contains("LNT-004") && findings.contains("draft"),
        "a value outside the nature's vocabulary is a finding: {findings}"
    );
}

// arqix:verifies REQ-03-01-09-01
#[test]
fn lint_reports_each_unverified_requirement_of_a_done_story() {
    let repo = common::scratch_copy(
        "minimal",
        "lint_reports_each_unverified_requirement_of_a_done_story",
    );
    let story = |status: &str| {
        format!(
            "---\nid: US-99-99-99\ntitle: Fixture Story\nslug: fixture-story\n\
             iri: arqix:user-stories/us-99-99-99\nrdf:\n  type:\n    - arqix:classes/user-story\n\
             triples:\n  - predicate: arqix:properties/has-requirement\n    object: arqix:requirements/req-99-99-99-01\n\
             meta:\n  lifecycle-status: {status}\n  owner: hcf\n  created: 2026-07-04\n  updated: 2026-07-04\n  lang: en\n  generated: false\n\
             ---\n\n## Fixture Story\n\nBody.\n"
        )
    };

    // A done claim over an unverified requirement is a finding naming both.
    std::fs::write(repo.join("docs/US-99-99-99-fixture-story.md"), story("done")).unwrap();
    let out = run_arqix_in(&repo, &["lint", "run", "--format", "json"]);
    common::assert_findings(&out);
    let findings = common::stdout_json(&out).to_string();
    assert!(
        findings.contains("LNT-005")
            && findings.contains("REQ-99-99-99-01")
            && findings.contains("US-99-99-99"),
        "the done claim must name the story and each unverified requirement: {findings}"
    );

    // The same story honestly declared in-implementation passes.
    std::fs::write(
        repo.join("docs/US-99-99-99-fixture-story.md"),
        story("in-implementation"),
    )
    .unwrap();
    common::assert_success(&run_arqix_in(&repo, &["lint", "run"]));
}
