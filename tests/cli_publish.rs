//! Command contract: `publish site`, `render pdf` — owned by the Publish &
//! Render Orchestrator (arc42 chapter 5). External toolchain errors are
//! forwarded transparently (REQ-04-01-03-07).

mod common;

use common::{run_arqix_in, scratch_copy};

/// The staging tests need a toolchain that succeeds without doing anything:
/// there is deliberately no built-in renderer and no fallback.
const NOOP_TOOLCHAIN: &str = "[policies.publish]\nsite-command = \"true\"\n";

// arqix:verifies REQ-04-01-07-01
#[test]
fn publish_site_publishes_per_language() {
    let repo = scratch_copy("minimal", "publish_site_publishes_per_language");
    std::fs::write(repo.join("arqix.toml"), NOOP_TOOLCHAIN).unwrap();
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

    // The default language stages to the staging root...
    let out = run_arqix_in(&repo, &["publish", "site", "--lang", "en"]);
    common::assert_success(&out);
    let staged =
        std::fs::read_to_string(repo.join("site-src/guide.md")).expect("site-src/guide.md");
    assert!(
        staged.contains("Hello **world**."),
        "staging keeps the Markdown body verbatim: {staged}"
    );
    assert!(
        !repo.join("site-src/anleitung.md").exists() && !repo.join("site-src/de").exists(),
        "a language build stages only that language's root"
    );

    // ...and every other language to its own subdirectory.
    let out = run_arqix_in(&repo, &["publish", "site", "--lang", "de"]);
    common::assert_success(&out);
    assert!(
        repo.join("site-src/de/anleitung.md").is_file(),
        "the non-default language stages under <staging-dir>/<lang>/"
    );
}

// arqix:verifies REQ-04-01-03-02
#[test]
fn publish_site_stages_artefact_ready_inputs() {
    let repo = scratch_copy("minimal", "publish_site_stages_artefact_ready_inputs");
    std::fs::write(repo.join("arqix.toml"), NOOP_TOOLCHAIN).unwrap();
    std::fs::write(
        repo.join("docs/fragment.md"),
        "included text from the fragment\n",
    )
    .unwrap();
    std::fs::write(
        repo.join("docs/page.md"),
        "---\nid: page-x\ntitle: Stitched Page\n---\n\n## Stitched Page\n\n<!-- arqix:include fragment.md -->\n",
    )
    .unwrap();

    common::assert_success(&run_arqix_in(&repo, &["publish", "site"]));
    let staged = std::fs::read_to_string(repo.join("site-src/page.md")).expect("site-src/page.md");
    assert!(
        staged.contains("included text from the fragment"),
        "includes are expanded before the toolchain sees the page: {staged}"
    );
    assert!(
        !staged.contains("arqix:include"),
        "directives never reach the toolchain: {staged}"
    );
    assert!(
        staged.contains("title: \"Stitched Page\"") && !staged.contains("iri:"),
        "the arqix frontmatter is reduced to the toolchain-consumable part: {staged}"
    );
}

// arqix:verifies REQ-04-01-03-01
#[test]
fn publish_site_generates_outputs_for_the_configured_target() {
    let repo = scratch_copy(
        "minimal",
        "publish_site_generates_outputs_for_the_configured_target",
    );
    std::fs::write(
        repo.join("arqix.toml"),
        "[policies.publish]\nsite-command = \"touch site-built\"\n",
    )
    .unwrap();
    common::assert_success(&run_arqix_in(&repo, &["publish", "site"]));
    assert!(
        repo.join("site-src/REQ-99-99-99-01-fixture-requirement.md")
            .is_file(),
        "every corpus document is staged for the target"
    );
    assert!(
        repo.join("site-built").exists(),
        "one publish run drives the target toolchain to its output"
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
        "the configured site command must run after staging"
    );
}

// arqix:no-requirement
#[test]
fn publish_site_requires_a_configured_toolchain() {
    // No built-in renderer, no fallback (owner decision 2026-07-11): a
    // publish without a configured toolchain is a config error, exactly
    // like `render pdf` without Pandoc.
    let repo = scratch_copy("minimal", "publish_site_requires_a_configured_toolchain");
    let out = run_arqix_in(&repo, &["publish", "site"]);
    assert_eq!(
        out.status.code(),
        Some(2),
        "publishing without a toolchain is a usage/config error"
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("site-command"),
        "the diagnostic must point at the missing configuration: {stderr}"
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

// arqix:no-requirement
#[test]
fn publish_site_honours_the_configured_exclude_scope() {
    // The publish scope keeps internal corpus areas (plan packages,
    // templates, the requirement/story source files) off the public site;
    // the lifecycle-based final-filter (ADR-0010) and the generated
    // requirement catalogue pages are the specified follow-ups.
    let repo = scratch_copy(
        "minimal",
        "publish_site_honours_the_configured_exclude_scope",
    );
    std::fs::write(
        repo.join("arqix.toml"),
        "[policies.publish]\nsite-command = \"true\"\nexclude = [\"internal\"]\n",
    )
    .unwrap();
    std::fs::create_dir_all(repo.join("docs/internal")).unwrap();
    std::fs::write(
        repo.join("docs/internal/notes.md"),
        "---\nid: notes\ntitle: Internal Notes\n---\n\n## Internal Notes\n\nNot public.\n",
    )
    .unwrap();

    common::assert_success(&run_arqix_in(&repo, &["publish", "site"]));
    assert!(
        repo.join("site-src/REQ-99-99-99-01-fixture-requirement.md")
            .is_file(),
        "documents outside the exclude scope stage as before"
    );
    assert!(
        !repo.join("site-src/internal").exists(),
        "an excluded subtree must not reach the staging dir"
    );
}

// arqix:no-requirement
#[test]
fn publish_site_stages_configured_assets() {
    // The site toolchain can only reference what reaches the staging dir:
    // logo and favicon are inputs like any page (site polish 2026-07-11).
    let repo = scratch_copy("minimal", "publish_site_stages_configured_assets");
    std::fs::write(
        repo.join("arqix.toml"),
        "[policies.publish]\nsite-command = \"true\"\nassets = [\"assets\"]\n",
    )
    .unwrap();
    std::fs::create_dir_all(repo.join("assets")).unwrap();
    std::fs::write(repo.join("assets/logo.png"), b"\x89PNG fake").unwrap();
    common::assert_success(&run_arqix_in(&repo, &["publish", "site"]));
    assert!(
        repo.join("site-src/assets/logo.png").is_file(),
        "configured asset trees are copied into the staging dir verbatim"
    );
}
