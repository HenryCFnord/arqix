//! Command contract: `trace markers` — the ported TDD marker gate
//! (REQ-03-01-06-04). Owned by the Trace Engine (arc42 chapter 5); the
//! Python checker `scripts/check_trace_markers.py` remains the conformance
//! oracle for the grace period (arc42 chapter 8, oracle policy).

mod common;

use common::{assert_findings, run_arqix_in, scratch_copy, stdout_json};

// The fixture content is assembled as single-line literals (with `\n`
// escapes) so no physical line of this test file is itself a whole-line
// marker the gate would read out of its own source.

// arqix:verifies REQ-03-01-06-04
#[test]
fn trace_markers_gates_test_functions_without_a_marker() {
    let repo = scratch_copy(
        "minimal",
        "trace_markers_gates_test_functions_without_a_marker",
    );

    // A requirement the gate can know about (the gate reads the req directory
    // directly, exactly like the oracle).
    std::fs::create_dir_all(repo.join("docs/en/architecture/req")).unwrap();
    std::fs::write(
        repo.join("docs/en/architecture/req/REQ-01-01-16-01-sample.md"),
        "---\nid: REQ-01-01-16-01\nrdf:\n  type:\n    - arqix:classes/functional-requirement\n---\n\nbody\n",
    )
    .unwrap();

    // A test function under tests/ with neither a marker nor a no-requirement
    // annotation: the canonical TRC-002 finding.
    std::fs::create_dir_all(repo.join("tests")).unwrap();
    std::fs::write(
        repo.join("tests/sample.rs"),
        "#[test]\nfn needs_a_marker() {}\n",
    )
    .unwrap();

    let out = run_arqix_in(&repo, &["trace", "markers", "--format", "json"]);
    // A finding is present, so the gate exits non-zero.
    assert_findings(&out);

    let report = stdout_json(&out);
    // The frozen JSON shape: sorted keys, the four top-level fields.
    for key in ["findings", "warnings", "tests_files", "coverage_by_kind"] {
        assert!(
            report.get(key).is_some(),
            "missing top-level key {key}: {report}"
        );
    }
    assert_eq!(
        report["tests_files"], 1,
        "exactly one file under tests/ was scanned: {report}"
    );
    assert!(
        report["coverage_by_kind"]["functional"]["total"]
            .as_u64()
            .is_some_and(|t| t >= 1),
        "the sample requirement counts toward functional coverage: {report}"
    );

    // The representative finding: TRC-002 against the marker-less test.
    let findings = report["findings"].as_array().expect("findings array");
    assert!(
        findings.iter().any(|f| f["rule"] == "TRC-002"
            && f["file"] == "tests/sample.rs"
            && f["message"] == "test has neither a verifies/plans marker nor an arqix:no-requirement annotation"),
        "expected a TRC-002 finding for the marker-less test: {report}"
    );
}
