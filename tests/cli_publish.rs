//! Command contract: `publish site`, `render pdf` — owned by the Publish &
//! Render Orchestrator (arc42 chapter 5). External toolchain errors are
//! forwarded transparently (REQ-04-01-03-07).

mod common;

use common::{run_arqix_in, scratch_copy};

// arqix:verifies REQ-04-01-07-01
#[test]
fn publish_site_publishes_per_language() {
    let repo = scratch_copy("minimal", "publish_site_publishes_per_language");
    std::fs::create_dir_all(repo.join("docs/en")).unwrap();
    std::fs::write(
        repo.join("docs/en/guide.md"),
        "---\nid: guide\ntitle: Guide\n---\n\n## Guide\n\nHello **world**.\n",
    )
    .unwrap();
    std::fs::create_dir_all(repo.join("docs/de")).unwrap();
    std::fs::write(
        repo.join("docs/de/anleitung.md"),
        "---\nid: anleitung\ntitle: Anleitung\n---\n\n## Anleitung\n\nHallo Welt.\n",
    )
    .unwrap();

    // The default language publishes to the site root...
    let out = run_arqix_in(&repo, &["publish", "site", "--lang", "en"]);
    common::assert_success(&out);
    let page = std::fs::read_to_string(repo.join("site/guide.html")).expect("site/guide.html");
    assert!(
        page.contains("<strong>world</strong>"),
        "the body must be rendered to HTML: {page}"
    );
    assert!(
        !repo.join("site/anleitung.html").exists() && !repo.join("site/de/guide.html").exists(),
        "a language build takes only that language's root"
    );

    // ...and every other language to its own subdirectory.
    let out = run_arqix_in(&repo, &["publish", "site", "--lang", "de"]);
    common::assert_success(&out);
    assert!(
        repo.join("site/de/anleitung.html").is_file(),
        "the non-default language publishes under site/<lang>/"
    );
}

// arqix:verifies REQ-04-01-03-01
#[test]
fn publish_site_generates_outputs_with_an_index() {
    let repo = scratch_copy("minimal", "publish_site_generates_outputs_with_an_index");
    let out = run_arqix_in(&repo, &["publish", "site"]);
    common::assert_success(&out);
    assert!(
        repo.join("site/REQ-99-99-99-01-fixture-requirement.html")
            .is_file(),
        "every corpus document becomes a page"
    );
    let index =
        std::fs::read_to_string(repo.join("site/index.html")).expect("site/index.html exists");
    assert!(
        index.contains("REQ-99-99-99-01-fixture-requirement.html"),
        "the index links the published pages: {index}"
    );
}

// arqix:verifies REQ-04-01-03-03
#[test]
fn publish_site_orchestrates_the_configured_toolchain() {
    let repo = scratch_copy(
        "minimal",
        "publish_site_orchestrates_the_configured_toolchain",
    );
    std::fs::write(
        repo.join("arqix.toml"),
        "[policies.publish]\nsite-command = \"touch toolchain-ran\"\n",
    )
    .unwrap();
    let out = run_arqix_in(&repo, &["publish", "site"]);
    common::assert_success(&out);
    assert!(
        repo.join("toolchain-ran").exists(),
        "the configured site command must run after generation"
    );
}

// arqix:verifies REQ-04-01-07-02
#[test]
fn publish_site_diagnoses_a_failing_toolchain() {
    let repo = scratch_copy("minimal", "publish_site_diagnoses_a_failing_toolchain");
    std::fs::write(
        repo.join("arqix.toml"),
        "[policies.publish]\nsite-command = \"false\"\n",
    )
    .unwrap();
    let out = run_arqix_in(&repo, &["publish", "site"]);
    assert_eq!(
        out.status.code(),
        Some(2),
        "a toolchain failure is a system error, not a finding"
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("false"),
        "the diagnostic must name the failing tool invocation: {stderr}"
    );
}

// arqix:verifies REQ-04-01-03-04
#[test]
#[ignore = "US-04-01-03: not implemented"]
fn render_pdf_renders_via_pandoc() {
    let repo = scratch_copy("minimal", "render_pdf_renders_via_pandoc");
    let out = run_arqix_in(&repo, &["render", "pdf"]);
    // CI images without Pandoc surface a clear diagnostic instead of a
    // silent failure; the success path is asserted where Pandoc exists.
    assert_ne!(out.status.code(), Some(70), "command must be implemented");
}

// arqix:verifies REQ-04-01-03-07
#[test]
#[ignore = "US-04-01-03: not implemented"]
fn render_forwards_tool_errors_transparently() {
    let repo = scratch_copy("minimal", "render_forwards_tool_errors_transparently");
    let out = run_arqix_in(&repo, &["render", "pdf"]);
    if !out.status.success() {
        assert!(
            !out.stderr.is_empty(),
            "a failing render must forward the tool error, never swallow it"
        );
    }
}
