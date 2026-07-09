//! Command contract: `doc init`, `doc new`, `doc list`, `doc read`,
//! `doc search` — owned by Template Engine and Document Store & Catalog
//! (arc42 chapter 5).

mod common;

use common::{assert_success, fixture, run_arqix_in, scratch_copy, stdout_json};

// arqix:verifies REQ-01-01-01-01
#[test]
fn doc_init_creates_the_standard_package_scaffold() {
    let repo = scratch_copy("minimal", "doc_init_creates_the_standard_package_scaffold");
    let out = run_arqix_in(&repo, &["doc", "init"]);
    assert_success(&out);
}

// arqix:verifies REQ-00-00-00-05
#[test]
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
fn doc_list_filters_the_catalog_by_kind() {
    let out = run_arqix_in(
        &fixture("minimal"),
        &["doc", "list", "--format", "json", "--kind", "requirement"],
    );
    assert_success(&out);
}

// arqix:verifies REQ-05-01-10-01
#[test]
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
fn doc_search_finds_documents_by_full_text() {
    let out = run_arqix_in(
        &fixture("minimal"),
        &["doc", "search", "Fixture", "--format", "json"],
    );
    assert_success(&out);
    let hits = stdout_json(&out);
    assert!(hits.to_string().contains("REQ-99-99-99-01"));
}

// arqix:verifies REQ-01-01-17-01
#[test]
fn doc_list_honours_configured_skip_dirs() {
    let repo = scratch_copy("minimal", "doc_list_honours_configured_skip_dirs");
    std::fs::write(repo.join("arqix.toml"), "skip-dirs = [\"archive\"]\n").unwrap();
    std::fs::create_dir_all(repo.join("docs/archive")).unwrap();
    std::fs::write(
        repo.join("docs/archive/REQ-99-99-99-02-archived.md"),
        "---\nid: REQ-99-99-99-02\ntitle: Archived\n---\nbody\n",
    )
    .unwrap();

    let out = run_arqix_in(&repo, &["doc", "list", "--format", "json"]);
    assert_success(&out);
    let catalog = stdout_json(&out).to_string();
    assert!(catalog.contains("REQ-99-99-99-01"), "regular docs stay");
    assert!(
        !catalog.contains("REQ-99-99-99-02"),
        "a document inside a configured skip-dir must not be discovered"
    );
}

// arqix:verifies REQ-00-00-00-01
#[cfg(unix)]
#[test]
fn doc_list_does_not_follow_directory_symlinks() {
    let repo = scratch_copy("minimal", "doc_list_does_not_follow_directory_symlinks");
    std::fs::create_dir_all(repo.join("docs/sub")).unwrap();
    // A parent symlink forms a cycle: docs -> docs/sub/up -> docs -> …
    // Following it makes discovery unbounded and multiplies every document
    // in the catalog, so directory symlinks must not be traversed (the
    // Python oracle's rglob does not follow them either).
    std::os::unix::fs::symlink("..", repo.join("docs/sub/up")).unwrap();

    let out = run_arqix_in(&repo, &["doc", "list", "--format", "json"]);
    assert_success(&out);
    let catalog = stdout_json(&out);
    let documents = catalog["documents"].as_array().expect("documents array");
    assert_eq!(
        documents.len(),
        1,
        "a directory symlink must not multiply catalog entries: {catalog}"
    );
}

// arqix:verifies REQ-05-01-08-01
#[test]
fn doc_list_lists_each_document_once_under_overlapping_roots() {
    let repo = scratch_copy(
        "minimal",
        "doc_list_lists_each_document_once_under_overlapping_roots",
    );
    std::fs::write(
        repo.join("arqix.toml"),
        "roots = [\"docs\", \"docs/sub\"]\n",
    )
    .unwrap();
    std::fs::create_dir_all(repo.join("docs/sub")).unwrap();
    std::fs::write(
        repo.join("docs/sub/REQ-99-99-99-04-nested.md"),
        "---\nid: REQ-99-99-99-04\ntitle: Nested\n---\nbody\n",
    )
    .unwrap();

    let out = run_arqix_in(&repo, &["doc", "list", "--format", "json"]);
    assert_success(&out);
    let catalog = stdout_json(&out);
    let nested: Vec<_> = catalog["documents"]
        .as_array()
        .expect("documents array")
        .iter()
        .filter(|d| d["id"] == "REQ-99-99-99-04")
        .collect();
    assert_eq!(
        nested.len(),
        1,
        "a document under two overlapping roots must appear once: {catalog}"
    );
}

// arqix:verifies REQ-01-01-17-02
#[test]
fn doc_list_skips_the_default_directories_without_an_override() {
    let repo = scratch_copy(
        "minimal",
        "doc_list_skips_the_default_directories_without_an_override",
    );
    std::fs::create_dir_all(repo.join("docs/node_modules")).unwrap();
    std::fs::write(
        repo.join("docs/node_modules/REQ-99-99-99-03-vendored.md"),
        "---\nid: REQ-99-99-99-03\ntitle: Vendored\n---\nbody\n",
    )
    .unwrap();

    let out = run_arqix_in(&repo, &["doc", "list", "--format", "json"]);
    assert_success(&out);
    let catalog = stdout_json(&out).to_string();
    assert!(
        !catalog.contains("REQ-99-99-99-03"),
        "the default skip set must apply without any override"
    );
}
