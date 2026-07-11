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

    // The standard scaffold (REQ-01-01-01-01) under the package root.
    assert!(repo.join("docs/index.md").is_file(), "index.md missing");
    for dir in [
        "docs/units",
        "docs/pages",
        "docs/artefacts",
        "docs/logs",
        "docs/.arqix",
    ] {
        assert!(repo.join(dir).is_dir(), "{dir} missing from the scaffold");
    }

    // Initialising is idempotent and never overwrites (REQ-00-00-00-08).
    std::fs::write(repo.join("docs/index.md"), "user content\n").unwrap();
    assert_success(&run_arqix_in(&repo, &["doc", "init"]));
    assert_eq!(
        std::fs::read_to_string(repo.join("docs/index.md")).unwrap(),
        "user content\n",
        "a second init must not overwrite an existing index.md"
    );
}

// arqix:verifies REQ-01-01-01-01
#[test]
fn doc_init_scaffolds_an_explicit_path() {
    let repo = scratch_copy("minimal", "doc_init_scaffolds_an_explicit_path");
    let out = run_arqix_in(&repo, &["doc", "init", "pkg"]);
    assert_success(&out);
    assert!(repo.join("pkg/index.md").is_file());
    assert!(repo.join("pkg/units").is_dir());
    assert!(repo.join("pkg/logs").is_dir());
}

// arqix:verifies REQ-01-01-01-02
#[test]
fn doc_init_writes_doc_index_frontmatter() {
    let repo = scratch_copy("minimal", "doc_init_writes_doc_index_frontmatter");
    assert_success(&run_arqix_in(&repo, &["doc", "init"]));
    let index = std::fs::read_to_string(repo.join("docs/index.md")).unwrap();
    let frontmatter = index
        .split("---")
        .nth(1)
        .expect("index.md must start with frontmatter");
    assert!(frontmatter.contains("id:"), "id missing: {index}");
    assert!(
        frontmatter.contains("kind: doc_index"),
        "kind=doc_index missing: {index}"
    );
    assert!(frontmatter.contains("title:"), "title missing: {index}");
}

// arqix:verifies REQ-00-00-00-13
#[test]
fn doc_new_rejects_a_kind_that_escapes_the_root() {
    let repo = scratch_copy("minimal", "doc_new_rejects_a_kind_that_escapes_the_root");
    let out = run_arqix_in(&repo, &["doc", "new", "../escape"]);
    assert_eq!(
        out.status.code(),
        Some(2),
        "a path-escaping kind is a usage error, not a write outside the root"
    );
    assert!(
        !repo.join("escape").exists(),
        "nothing may be created outside the configured root"
    );
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

// arqix:no-requirement
#[test]
fn doc_new_requirement_scaffold_passes_the_default_lint() {
    // Found dogfooding the Quick Start: the generic scaffold declared
    // `lifecycle-status: draft`, but the requirement vocabulary is
    // active/retired only (LNT-004) — a fresh scaffold must never fail
    // the default gates it will be checked against.
    let repo = scratch_copy(
        "minimal",
        "doc_new_requirement_scaffold_passes_the_default_lint",
    );
    assert_success(&run_arqix_in(&repo, &["doc", "new", "requirement"]));
    let scaffold =
        std::fs::read_to_string(repo.join("docs/requirement/REQUIREMENT-0001.md")).unwrap();
    assert!(
        scaffold.contains("lifecycle-status: active"),
        "a requirement is active or retired; there is no draft requirement: {scaffold}"
    );
    let out = run_arqix_in(&repo, &["lint", "run"]);
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

// arqix:verifies REQ-00-00-00-09
#[test]
fn doc_new_dry_run_reports_the_plan_without_writing() {
    let repo = scratch_copy(
        "minimal",
        "doc_new_dry_run_reports_the_plan_without_writing",
    );
    let out = run_arqix_in(
        &repo,
        &["doc", "new", "adr", "--dry-run", "--format", "json"],
    );
    assert_success(&out);
    let result = stdout_json(&out);
    assert_eq!(result["id"], "ADR-0001", "the planned ID: {result}");
    assert!(
        result["path"]
            .as_str()
            .unwrap_or("")
            .ends_with("ADR-0001.md"),
        "the planned target path: {result}"
    );
    assert_eq!(
        result["dry_run"], true,
        "the run must declare itself: {result}"
    );
    assert!(
        !repo.join("docs/adr/ADR-0001.md").exists(),
        "a dry run must not write any file"
    );
}

// arqix:verifies REQ-01-01-13-01
#[test]
fn doc_new_accepts_an_explicit_id_and_rejects_a_duplicate() {
    let repo = scratch_copy(
        "minimal",
        "doc_new_accepts_an_explicit_id_and_rejects_a_duplicate",
    );
    let out = run_arqix_in(&repo, &["doc", "new", "adr", "--id", "ADR-0042"]);
    assert_success(&out);
    let created = std::fs::read_to_string(repo.join("docs/adr/ADR-0042.md")).unwrap();
    assert!(
        created.contains("id: ADR-0042"),
        "the explicit ID must be used verbatim: {created}"
    );

    // Uniqueness is verified for explicit IDs too (REQ-01-01-13-01).
    let duplicate = run_arqix_in(&repo, &["doc", "new", "adr", "--id", "ADR-0042"]);
    common::assert_findings(&duplicate);
}

// arqix:verifies REQ-00-00-00-05
#[test]
fn doc_new_substitutes_the_title_into_the_template() {
    let repo = scratch_copy("minimal", "doc_new_substitutes_the_title_into_the_template");
    let out = run_arqix_in(
        &repo,
        &["doc", "new", "adr", "--title", "Choose a Database"],
    );
    assert_success(&out);
    let created = std::fs::read_to_string(repo.join("docs/adr/ADR-0001.md")).unwrap();
    assert!(
        created.contains("title: Choose a Database"),
        "the {{title}} placeholder must carry the given title: {created}"
    );
    assert!(
        created.contains("slug: choose-a-database"),
        "the {{slug}} placeholder must derive from the title: {created}"
    );
    assert!(
        created.contains("## Choose a Database"),
        "the body heading must carry the title: {created}"
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

// arqix:verifies REQ-01-01-20-01
#[test]
fn doc_new_instantiates_the_configured_template_file() {
    let repo = scratch_copy(
        "minimal",
        "doc_new_instantiates_the_configured_template_file",
    );
    std::fs::write(repo.join("arqix.toml"), "[templates]\ndir = \"tpl\"\n").unwrap();
    std::fs::create_dir_all(repo.join("tpl")).unwrap();
    std::fs::write(
        repo.join("tpl/requirement.tpl.md"),
        "---\nid: {id}\ntitle: {title}\nslug: {slug}\n---\n\n## {title}\n\nHouse style, not a string literal.\n",
    )
    .unwrap();
    let out = run_arqix_in(
        &repo,
        &["doc", "new", "requirement", "--title", "Shaped by a File"],
    );
    assert_success(&out);
    let created =
        std::fs::read_to_string(repo.join("docs/requirement/REQUIREMENT-0001.md")).unwrap();
    assert!(
        created.contains("House style, not a string literal."),
        "the configured template file shapes the document: {created}"
    );
    assert!(
        created.contains("id: REQUIREMENT-0001")
            && created.contains("title: Shaped by a File")
            && created.contains("slug: shaped-by-a-file"),
        "the placeholders substitute exactly as before: {created}"
    );
}

// arqix:verifies REQ-01-01-20-02
#[test]
fn doc_init_scaffolds_the_default_template_files() {
    let repo = scratch_copy("minimal", "doc_init_scaffolds_the_default_template_files");
    assert_success(&run_arqix_in(&repo, &["doc", "init"]));
    let requirement = repo.join("docs/templates/requirement.tpl.md");
    assert!(
        requirement.is_file(),
        "doc init scaffolds the default template files"
    );
    let text = std::fs::read_to_string(&requirement).unwrap();
    assert!(
        text.contains("{title}") && text.contains("{id}"),
        "scaffolded templates keep their placeholders: {text}"
    );
    assert!(
        text.contains("lifecycle-status: active"),
        "the requirement template declares what the default gates accept: {text}"
    );

    // Init never overwrites: a shaped template survives a second init.
    std::fs::write(&requirement, "house template\n").unwrap();
    assert_success(&run_arqix_in(&repo, &["doc", "init"]));
    assert_eq!(
        std::fs::read_to_string(&requirement).unwrap(),
        "house template\n"
    );
}

// arqix:verifies REQ-01-01-20-03
#[test]
fn doc_new_fails_clearly_on_a_missing_template_file() {
    let repo = scratch_copy(
        "minimal",
        "doc_new_fails_clearly_on_a_missing_template_file",
    );
    std::fs::write(repo.join("arqix.toml"), "[templates]\ndir = \"tpl\"\n").unwrap();
    let out = run_arqix_in(&repo, &["doc", "new", "adr"]);
    assert_eq!(
        out.status.code(),
        Some(2),
        "a configured but missing template is a config error"
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("tpl/adr.tpl.md"),
        "the diagnostic names the expected path: {stderr}"
    );
}
