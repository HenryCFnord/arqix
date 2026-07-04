//! Command contract: `report bundle` — owned by Report & Export; the only
//! `report` command (ADR-0005: the report verb is reserved for export
//! products).

mod common;

use common::{assert_success, run_arqix_in, scratch_copy};

// arqix:verifies REQ-03-01-04-01
#[test]
#[ignore = "US-03-01-04: not implemented"]
fn report_bundle_exports_an_evidence_bundle_by_id_scope() {
    let repo = scratch_copy(
        "minimal",
        "report_bundle_exports_an_evidence_bundle_by_id_scope",
    );
    let out = run_arqix_in(&repo, &["report", "bundle", "REQ-99-99-99-01"]);
    assert_success(&out);
}

// arqix:verifies REQ-03-01-04-02
#[test]
#[ignore = "US-03-01-04: not implemented"]
fn report_bundle_includes_linked_evidence() {
    let repo = scratch_copy("minimal", "report_bundle_includes_linked_evidence");
    let out = run_arqix_in(
        &repo,
        &["report", "bundle", "REQ-99-99-99-01", "--format", "json"],
    );
    assert_success(&out);
    let bundle = common::stdout_json(&out);
    assert!(bundle.to_string().contains("REQ-99-99-99-01"));
}
