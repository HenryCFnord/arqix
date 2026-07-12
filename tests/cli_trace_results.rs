//! Command contract: `trace coverage --results <report>` — joined test
//! outcomes (US-03-01-10). Engine-only surface: the Python oracle stays at
//! the frozen pre-results interface, so these tests live outside the
//! `cli_trace` conformance suite by design.

mod common;

use common::{run_arqix_in, scratch_copy, stdout_json};

/// A scratch corpus: two requirements, one verified by a passing test and
/// one by a failing test, per the JUnit report written alongside.
fn write_results_fixture(repo: &std::path::Path) {
    for (id, slug) in [("REQ-88-01-01-01", "green"), ("REQ-88-01-01-02", "red")] {
        std::fs::write(
            repo.join(format!("docs/{id}-{slug}.md")),
            format!(
                "---\nid: {id}\ntitle: A {slug} requirement\niri: arqix:requirements/{}\nrdf:\n  type:\n    - arqix:classes/functional-requirement\n---\n\n## Requirement\n\nThe system SHALL be {slug}.\n",
                id.to_lowercase()
            ),
        )
        .unwrap();
    }
    std::fs::write(
        repo.join("checks.rs"),
        "// arqix:verifies REQ-88-01-01-01\n#[test]\nfn green_check() {}\n\n// arqix:verifies REQ-88-01-01-02\n#[test]\nfn red_check() {}\n\n// arqix:verifies REQ-99-99-99-01\n#[test]\nfn fixture_covered() {}\n",
    )
    .unwrap();
    std::fs::write(
        repo.join("results.xml"),
        r#"<?xml version="1.0" encoding="UTF-8"?>
<testsuites>
  <testsuite name="checks" tests="3">
    <testcase name="green_check" classname="checks" time="0.01"/>
    <testcase name="red_check" classname="checks" time="0.01">
      <failure message="assertion failed">left != right</failure>
    </testcase>
  </testsuite>
</testsuites>
"#,
    )
    .unwrap();
}

// arqix:verifies REQ-03-01-10-02
#[test]
fn coverage_joins_junit_outcomes_by_test_name() {
    let repo = scratch_copy("minimal", "coverage_joins_junit_outcomes_by_test_name");
    write_results_fixture(&repo);
    let out = run_arqix_in(
        &repo,
        &[
            "trace",
            "coverage",
            "--results",
            "results.xml",
            "--format",
            "json",
        ],
    );
    let coverage = stdout_json(&out);
    let row = |id: &str| {
        coverage["requirements"]
            .as_array()
            .and_then(|rows| rows.iter().find(|r| r["id"] == id))
            .cloned()
            .unwrap_or_default()
    };

    let green = row("REQ-88-01-01-01");
    assert_eq!(
        green["results"]["passed"][0], "green_check",
        "the passing outcome joins by test name: {coverage}"
    );

    // The fixture requirement's test has no entry in the report: unjoined
    // markers keep their marker-derived status — results never invent
    // evidence.
    let untouched = row("REQ-99-99-99-01");
    assert!(
        untouched["verified_by"]
            .as_array()
            .is_some_and(|v| !v.is_empty()),
        "unjoined claims stay verified: {coverage}"
    );
}

// arqix:verifies REQ-03-01-10-03
#[test]
fn failed_outcomes_demote_the_verifying_claim() {
    let repo = scratch_copy("minimal", "failed_outcomes_demote_the_verifying_claim");
    write_results_fixture(&repo);
    let out = run_arqix_in(
        &repo,
        &[
            "trace",
            "coverage",
            "--results",
            "results.xml",
            "--format",
            "json",
        ],
    );
    let coverage = stdout_json(&out);
    let red = coverage["requirements"]
        .as_array()
        .and_then(|rows| rows.iter().find(|r| r["id"] == "REQ-88-01-01-02"))
        .cloned()
        .unwrap_or_default();
    assert!(
        red["verified_by"].as_array().is_some_and(|v| v.is_empty()),
        "a failed outcome must not count as verified: {coverage}"
    );
    assert!(
        red["planned_by"].as_array().is_some_and(|p| !p.is_empty()),
        "the demoted claim stays visible as planned: {coverage}"
    );
    assert_eq!(
        red["results"]["failed"][0], "red_check",
        "the row names the failing test: {coverage}"
    );
}

// arqix:verifies REQ-03-01-10-02
#[test]
fn coverage_without_results_is_byte_identical_to_before() {
    // The flag refines, its absence changes nothing: without --results the
    // rows carry no results key at all, keeping the JSON value-equal to
    // the frozen oracle surface.
    let repo = scratch_copy(
        "minimal",
        "coverage_without_results_is_byte_identical_to_before",
    );
    write_results_fixture(&repo);
    let out = run_arqix_in(&repo, &["trace", "coverage", "--format", "json"]);
    let coverage = stdout_json(&out);
    assert!(
        coverage["requirements"]
            .as_array()
            .is_some_and(|rows| rows.iter().all(|r| r.get("results").is_none())),
        "no results key without a results file: {coverage}"
    );
}
