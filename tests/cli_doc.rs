//! Command contract: `doc init`, `doc new`, `doc list`, `doc read`,
//! `doc search` — owned by Template Engine and Document Store & Catalog
//! (arc42 chapter 5).

mod common;

use common::{assert_success, fixture, run_arqix_in, scratch_copy, stdout_json};

// arqix:verifies REQ-01-01-01-01
#[test]
#[ignore = "US-01-01-01: not implemented"]
fn doc_init_creates_the_standard_package_scaffold() {
    let repo = scratch_copy("minimal", "doc_init_creates_the_standard_package_scaffold");
    let out = run_arqix_in(&repo, &["doc", "init"]);
    assert_success(&out);
}

// arqix:verifies REQ-00-00-00-05
#[test]
#[ignore = "US-01-01-13: not implemented"]
fn doc_new_creates_a_document_from_the_configured_template() {
    let repo = scratch_copy(
        "minimal",
        "doc_new_creates_a_document_from_the_configured_template",
    );
    let out = run_arqix_in(&repo, &["doc", "new", "adr"]);
    assert_success(&out);
}

// arqix:verifies REQ-01-01-13-01
#[test]
#[ignore = "US-01-01-13: not implemented"]
fn doc_new_generates_a_unique_id_from_the_configured_policy() {
    let repo = scratch_copy(
        "minimal",
        "doc_new_generates_a_unique_id_from_the_configured_policy",
    );
    assert_success(&run_arqix_in(&repo, &["doc", "new", "adr"]));
    assert_success(&run_arqix_in(&repo, &["doc", "new", "adr"]));
    // Two creations must yield two distinct IDs; the duplicate check is
    // asserted through the catalog once `doc list` exists.
}

// arqix:verifies REQ-01-01-13-02
#[test]
#[ignore = "US-01-01-13: not implemented"]
fn doc_new_writes_into_the_configured_kind_location() {
    let repo = scratch_copy(
        "minimal",
        "doc_new_writes_into_the_configured_kind_location",
    );
    let out = run_arqix_in(&repo, &["doc", "new", "adr", "--format", "json"]);
    assert_success(&out);
    let result = stdout_json(&out);
    assert!(
        result.to_string().contains("path"),
        "creation result must report the target path: {result}"
    );
}

// arqix:verifies REQ-05-01-08-01
#[test]
#[ignore = "US-05-01-08: not implemented"]
fn doc_list_emits_a_json_document_catalog() {
    let out = run_arqix_in(&fixture("minimal"), &["doc", "list", "--format", "json"]);
    assert_success(&out);
    let catalog = stdout_json(&out);
    assert!(
        catalog.to_string().contains("REQ-99-99-99-01"),
        "catalog must contain the fixture document: {catalog}"
    );
}

// arqix:verifies REQ-05-01-08-03
#[test]
#[ignore = "US-05-01-08: not implemented"]
fn doc_list_filters_the_catalog_by_kind() {
    let out = run_arqix_in(
        &fixture("minimal"),
        &["doc", "list", "--format", "json", "--kind", "requirement"],
    );
    assert_success(&out);
}

// arqix:verifies REQ-05-01-10-01
#[test]
#[ignore = "US-05-01-10: not implemented"]
fn doc_read_retrieves_a_document_by_id() {
    let out = run_arqix_in(
        &fixture("minimal"),
        &["doc", "read", "REQ-99-99-99-01", "--format", "json"],
    );
    assert_success(&out);
    let doc = stdout_json(&out);
    assert!(doc.to_string().contains("Fixture Requirement"));
}

// arqix:verifies REQ-05-01-10-03
#[test]
#[ignore = "US-05-01-10: not implemented"]
fn doc_read_distinguishes_a_document_miss_from_a_selector_miss() {
    let missing_doc = run_arqix_in(
        &fixture("minimal"),
        &["doc", "read", "REQ-99-99-99-99", "--format", "json"],
    );
    common::assert_findings(&missing_doc);
    assert!(
        String::from_utf8_lossy(&missing_doc.stdout).contains("document"),
        "a missing document must be diagnosed as such"
    );
}

// arqix:verifies REQ-02-01-06-01
#[test]
#[ignore = "US-02-01-06: not implemented"]
fn doc_search_finds_documents_by_full_text() {
    let out = run_arqix_in(
        &fixture("minimal"),
        &["doc", "search", "Fixture", "--format", "json"],
    );
    assert_success(&out);
    let hits = stdout_json(&out);
    assert!(hits.to_string().contains("REQ-99-99-99-01"));
}
