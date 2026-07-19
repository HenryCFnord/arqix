//! Command contract: `publish site`, `render pdf` — owned by the Publish &
//! Render Orchestrator (arc42 chapter 5). External toolchain errors are
//! forwarded transparently (REQ-04-01-03-07).

mod common;

use common::{run_arqix_in, scratch_copy};

/// The staging tests need a toolchain that succeeds without doing anything:
/// there is deliberately no built-in renderer and no fallback.
const NOOP_TOOLCHAIN: &str = "[policies.publish]\nsite-command = \"true\"\n";

// arqix:verifies REQ-04-01-07-04
#[test]
fn publish_site_stages_doc_assets_at_page_relative_paths() {
    let repo = scratch_copy(
        "minimal",
        "publish_site_stages_doc_assets_at_page_relative_paths",
    );
    std::fs::write(
        repo.join("arqix.toml"),
        "[policies.publish]\nsite-command = \"true\"\n\
         assets = [\"assets/brand.svg\", \"docs/media/diagram.svg\"]\n",
    )
    .unwrap();
    std::fs::create_dir_all(repo.join("assets")).unwrap();
    std::fs::write(repo.join("assets/brand.svg"), "<svg/>\n").unwrap();
    std::fs::create_dir_all(repo.join("docs/media")).unwrap();
    std::fs::write(repo.join("docs/media/diagram.svg"), "<svg/>\n").unwrap();

    common::assert_success(&run_arqix_in(&repo, &["publish", "site"]));
    // The asset under the doc root stages with the `docs/` prefix stripped, so a
    // staged page's `../media/diagram.svg`-style link resolves.
    assert!(
        repo.join("site-src/media/diagram.svg").is_file(),
        "a doc-root asset stages at its document-relative path"
    );
    assert!(
        !repo.join("site-src/docs/media/diagram.svg").exists(),
        "the verbatim docs/ path must not be used for a doc-root asset"
    );
    // An asset outside any doc root keeps its configured path.
    assert!(
        repo.join("site-src/assets/brand.svg").is_file(),
        "a non-doc-root asset stages verbatim"
    );
}

// arqix:verifies REQ-04-01-07-03
#[test]
fn publish_site_omits_included_fragments() {
    let repo = scratch_copy("minimal", "publish_site_omits_included_fragments");
    std::fs::write(repo.join("arqix.toml"), NOOP_TOOLCHAIN).unwrap();
    std::fs::write(
        repo.join("docs/fragment.md"),
        "---\nid: frag\ntitle: Frag\n---\n\n## Frag Body\n\nStitched prose.\n",
    )
    .unwrap();
    std::fs::write(
        repo.join("docs/page.md"),
        "---\nid: pg\ntitle: Page\n---\n\n## Page\n\n<!-- arqix:include fragment.md level=+1 -->\n",
    )
    .unwrap();
    common::assert_success(&run_arqix_in(&repo, &["publish", "site"]));
    assert!(
        !repo.join("site-src/fragment.md").exists(),
        "an included fragment must not be staged as a standalone page"
    );
    let page = std::fs::read_to_string(repo.join("site-src/page.md")).expect("site-src/page.md");
    assert!(
        page.contains("Frag Body"),
        "the fragment still reaches the site through its parent page: {page}"
    );
}

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

/// A recording fake renderer: appends its argument list to `args.txt` and
/// writes the word `rendered` to whatever follows `-o` — enough to observe
/// every invocation arqix builds (one per document, now that PDF splits per
/// document) without requiring Pandoc in the test image.
fn install_fake_renderer(repo: &std::path::Path) {
    let script = "#!/bin/sh\nprintf '%s\\n' \"$@\" >> args.txt\nout=\"\"\nprev=\"\"\nfor a in \"$@\"; do\n  if [ \"$prev\" = \"-o\" ]; then out=\"$a\"; fi\n  prev=\"$a\"\ndone\nif [ -n \"$out\" ]; then echo rendered > \"$out\"; fi\n";
    let path = repo.join("fakepandoc.sh");
    std::fs::write(&path, script).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).unwrap();
    }
}

// arqix:verifies REQ-04-01-03-04
#[test]
fn render_pdf_renders_via_pandoc() {
    let repo = scratch_copy("minimal", "render_pdf_renders_via_pandoc");
    let out = run_arqix_in(&repo, &["render", "pdf"]);
    // CI images without Pandoc surface a clear diagnostic instead of a
    // silent failure; the success path is asserted with the recording
    // renderer below.
    assert_ne!(out.status.code(), Some(70), "command must be implemented");

    // The staged, artefact-ready pages are what reaches the renderer.
    install_fake_renderer(&repo);
    std::fs::write(
        repo.join("arqix.toml"),
        "[policies.render]\npdf-command = \"./fakepandoc.sh\"\n",
    )
    .unwrap();
    common::assert_success(&run_arqix_in(&repo, &["render", "pdf"]));
    let args = std::fs::read_to_string(repo.join("args.txt")).unwrap();
    assert!(
        args.contains("REQ-99-99-99-01") && args.contains("-o"),
        "the renderer is invoked on the staged pages with an output target: {args}"
    );
}

// arqix:verifies REQ-04-01-03-04
#[test]
fn render_pdf_accepts_selected_markdown_files() {
    let repo = scratch_copy("minimal", "render_pdf_accepts_selected_markdown_files");
    install_fake_renderer(&repo);
    std::fs::write(
        repo.join("arqix.toml"),
        "[policies.render]\npdf-command = \"./fakepandoc.sh\"\n",
    )
    .unwrap();
    common::assert_success(&run_arqix_in(
        &repo,
        &[
            "render",
            "pdf",
            "docs/REQ-99-99-99-01-fixture-requirement.md",
        ],
    ));
    let args = std::fs::read_to_string(repo.join("args.txt")).unwrap();
    assert!(
        args.contains("REQ-99-99-99-01-fixture-requirement"),
        "selected files are what the renderer sees: {args}"
    );
}

// arqix:verifies REQ-04-01-03-05
// arqix:verifies REQ-04-01-03-08
#[test]
fn render_pdf_supports_defaults_eisvogel_and_package_overrides() {
    let repo = scratch_copy(
        "minimal",
        "render_pdf_supports_defaults_eisvogel_and_package_overrides",
    );
    install_fake_renderer(&repo);
    std::fs::write(repo.join("pandoc-defaults.yaml"), "toc: true\n").unwrap();
    // The global policy sets the defaults file; the per-package override
    // adds the eisvogel template for the `docs` package only.
    std::fs::write(
        repo.join("arqix.toml"),
        "[policies.render]\npdf-command = \"./fakepandoc.sh\"\ndefaults = \"pandoc-defaults.yaml\"\n\n[policies.render.package.docs]\ntemplate = \"eisvogel\"\n",
    )
    .unwrap();
    common::assert_success(&run_arqix_in(&repo, &["render", "pdf"]));
    let args = std::fs::read_to_string(repo.join("args.txt")).unwrap();
    assert!(
        args.contains("--defaults") && args.contains("pandoc-defaults.yaml"),
        "the configured Pandoc defaults file is passed through: {args}"
    );
    assert!(
        args.contains("--template") && args.contains("eisvogel"),
        "the per-package override adds the eisvogel template: {args}"
    );
}

// arqix:verifies REQ-04-01-03-06
#[test]
fn render_pdf_stores_artefacts_per_configured_mode() {
    let repo = scratch_copy("minimal", "render_pdf_stores_artefacts_per_configured_mode");
    install_fake_renderer(&repo);
    // A single top-level document so the artefact name is predictable: the
    // configured mode governs where its PDF lands (per document now, not per
    // package — REQ-04-01-03-06 granularity moved package->document).
    std::fs::create_dir_all(repo.join("docs/handbook")).unwrap();
    std::fs::write(
        repo.join("docs/handbook/index.md"),
        "---\nid: handbook\ntitle: Handbook\n---\n\n## Handbook\n\nBody.\n",
    )
    .unwrap();

    // The default mode stores into the package's artefacts/ directory —
    // the location the doc init scaffold reserves for render products.
    std::fs::write(
        repo.join("arqix.toml"),
        "[policies.render]\npdf-command = \"./fakepandoc.sh\"\ndocuments = [ { name = \"handbook\", path = \"handbook\" } ]\n",
    )
    .unwrap();
    common::assert_success(&run_arqix_in(&repo, &["render", "pdf"]));
    assert!(
        repo.join("docs/artefacts/handbook.pdf").is_file(),
        "package mode stores each document's artefact inside the package"
    );

    // The detached mode stores into the configured artefact directory.
    std::fs::write(
        repo.join("arqix.toml"),
        "[policies.render]\npdf-command = \"./fakepandoc.sh\"\nartefact-mode = \"detached\"\nartefact-dir = \"render-out\"\ndocuments = [ { name = \"handbook\", path = \"handbook\" } ]\n",
    )
    .unwrap();
    common::assert_success(&run_arqix_in(&repo, &["render", "pdf"]));
    assert!(
        repo.join("render-out/handbook.pdf").is_file(),
        "detached mode stores the artefact outside the package"
    );
}

// arqix:verifies REQ-04-01-03-09
#[test]
fn render_pdf_produces_one_artefact_per_document() {
    // PDF splits per top-level document: two document families produce two
    // named artefacts, not one concatenated docs.pdf (REQ-04-01-03-09).
    let repo = scratch_copy("minimal", "render_pdf_produces_one_artefact_per_document");
    install_fake_renderer(&repo);
    std::fs::write(
        repo.join("arqix.toml"),
        "[policies.render]\npdf-command = \"./fakepandoc.sh\"\ndocuments = [\n  { name = \"alpha\", path = \"alpha\", title = \"Alpha\" },\n  { name = \"beta\", path = \"beta\", title = \"Beta\" },\n]\n",
    )
    .unwrap();
    std::fs::create_dir_all(repo.join("docs/alpha")).unwrap();
    std::fs::write(
        repo.join("docs/alpha/index.md"),
        "---\nid: alpha\ntitle: Alpha\n---\n\n## Alpha\n\nAlpha body.\n",
    )
    .unwrap();
    std::fs::create_dir_all(repo.join("docs/beta")).unwrap();
    std::fs::write(
        repo.join("docs/beta/index.md"),
        "---\nid: beta\ntitle: Beta\n---\n\n## Beta\n\nBeta body.\n",
    )
    .unwrap();
    common::assert_success(&run_arqix_in(&repo, &["render", "pdf"]));
    assert!(
        repo.join("docs/artefacts/alpha.pdf").is_file(),
        "the alpha document renders its own artefact"
    );
    assert!(
        repo.join("docs/artefacts/beta.pdf").is_file(),
        "the beta document renders its own artefact"
    );
    assert!(
        !repo.join("docs/artefacts/docs.pdf").exists(),
        "the package is no longer concatenated into a single PDF"
    );
}

// arqix:verifies REQ-04-01-03-09
#[test]
fn render_pdf_drops_included_fragments_from_a_document() {
    // A fragment that a page includes is inlined by the assembler, so it
    // must not also appear as a standalone staged input (REQ-04-01-03-09).
    let repo = scratch_copy(
        "minimal",
        "render_pdf_drops_included_fragments_from_a_document",
    );
    install_fake_renderer(&repo);
    std::fs::write(
        repo.join("arqix.toml"),
        "[policies.render]\npdf-command = \"./fakepandoc.sh\"\ndocuments = [ { name = \"guide\", path = \"guide\", title = \"Guide\" } ]\n",
    )
    .unwrap();
    std::fs::create_dir_all(repo.join("docs/guide/units")).unwrap();
    std::fs::write(
        repo.join("docs/guide/index.md"),
        "---\nid: guide\ntitle: Guide\n---\n\n## Guide\n\nIntro.\n",
    )
    .unwrap();
    std::fs::write(
        repo.join("docs/guide/page-01.md"),
        "---\nid: page-01\ntitle: Page One\n---\n\n## Page One\n\n<!-- arqix:include units/unit-01.md -->\n",
    )
    .unwrap();
    std::fs::write(
        repo.join("docs/guide/units/unit-01.md"),
        "### Unit One\n\nUnit body from the fragment.\n",
    )
    .unwrap();
    common::assert_success(&run_arqix_in(&repo, &["render", "pdf"]));
    assert!(
        !repo.join("render-staging/guide/units/unit-01.md").exists(),
        "the included fragment is not staged as its own page"
    );
    let args = std::fs::read_to_string(repo.join("args.txt")).unwrap();
    assert!(
        !args.contains("units/unit-01"),
        "the included fragment is not a separate renderer input: {args}"
    );
    let page = std::fs::read_to_string(repo.join("render-staging/guide/page-01.md")).unwrap();
    assert!(
        page.contains("Unit body from the fragment"),
        "the fragment is inlined into the page that includes it: {page}"
    );
}

// arqix:verifies REQ-04-01-03-09
#[test]
fn render_pdf_drops_the_index_landing_and_lifts_units_to_top_level() {
    // A family's index.md landing is a site-navigation stub, not a PDF
    // chapter: it is dropped when the family carries other content, and the
    // content page's own leading title is dropped so its units lift to H1
    // instead of nesting under a wrapper title (REQ-04-01-03-09).
    let repo = scratch_copy(
        "minimal",
        "render_pdf_drops_the_index_landing_and_lifts_units_to_top_level",
    );
    install_fake_renderer(&repo);
    std::fs::write(
        repo.join("arqix.toml"),
        "[policies.render]\npdf-command = \"./fakepandoc.sh\"\ndocuments = [ { name = \"guide\", path = \"guide\", title = \"Guide\" } ]\n",
    )
    .unwrap();
    std::fs::create_dir_all(repo.join("docs/guide/units")).unwrap();
    std::fs::write(
        repo.join("docs/guide/index.md"),
        "---\nid: guide\ntitle: Guide\n---\n\n## Guide\n\nStart with the assembled document.\n",
    )
    .unwrap();
    std::fs::write(
        repo.join("docs/guide/page-01.md"),
        "---\nid: page-01\ntitle: Page One\n---\n\n## Page One\n\n<!-- arqix:include units/unit-01.md -->\n",
    )
    .unwrap();
    std::fs::write(
        repo.join("docs/guide/units/unit-01.md"),
        "### Unit One\n\nUnit body.\n",
    )
    .unwrap();
    common::assert_success(&run_arqix_in(&repo, &["render", "pdf"]));
    assert!(
        !repo.join("render-staging/guide/index.md").exists(),
        "the index.md landing is not staged as a chapter"
    );
    let page = std::fs::read_to_string(repo.join("render-staging/guide/page-01.md")).unwrap();
    assert!(
        !page.contains("Page One"),
        "the page's own leading title is dropped: {page}"
    );
    assert!(
        page.lines().any(|l| l == "# Unit One"),
        "the unit lifts to a top-level H1 chapter: {page}"
    );
}

// arqix:verifies REQ-04-01-03-09
#[test]
fn render_pdf_keeps_member_titles_in_a_collection_document() {
    // A collection document (several sibling files, like adr/ or blog/) keeps
    // each member's title as its own H1 chapter, unlike a single-document page
    // that drops the wrapper title (REQ-04-01-03-09).
    let repo = scratch_copy(
        "minimal",
        "render_pdf_keeps_member_titles_in_a_collection_document",
    );
    install_fake_renderer(&repo);
    std::fs::write(
        repo.join("arqix.toml"),
        "[policies.render]\npdf-command = \"./fakepandoc.sh\"\ndocuments = [ { name = \"records\", path = \"records\", title = \"Records\" } ]\n",
    )
    .unwrap();
    std::fs::create_dir_all(repo.join("docs/records")).unwrap();
    std::fs::write(
        repo.join("docs/records/index.md"),
        "---\nid: records\ntitle: Records\n---\n\n## Records\n\nLanding.\n",
    )
    .unwrap();
    std::fs::write(
        repo.join("docs/records/rec-01.md"),
        "---\nid: rec-01\ntitle: First Record\n---\n\n## First Record\n\n### Context\n\nBody.\n",
    )
    .unwrap();
    std::fs::write(
        repo.join("docs/records/rec-02.md"),
        "---\nid: rec-02\ntitle: Second Record\n---\n\n## Second Record\n\n### Context\n\nBody.\n",
    )
    .unwrap();
    common::assert_success(&run_arqix_in(&repo, &["render", "pdf"]));
    let rec1 = std::fs::read_to_string(repo.join("render-staging/records/rec-01.md")).unwrap();
    assert!(
        rec1.lines().any(|l| l == "# First Record") && rec1.lines().any(|l| l == "## Context"),
        "a collection member keeps its title as an H1 chapter with sections below: {rec1}"
    );
    assert!(
        !repo.join("render-staging/records/index.md").exists(),
        "the landing index is still dropped"
    );
}

// arqix:verifies REQ-04-01-03-09
#[test]
fn render_pdf_passes_each_document_title_as_metadata() {
    // Each document's title reaches the renderer as explicit metadata, one
    // running header / title-page source per PDF (REQ-04-01-03-09).
    let repo = scratch_copy(
        "minimal",
        "render_pdf_passes_each_document_title_as_metadata",
    );
    install_fake_renderer(&repo);
    std::fs::write(
        repo.join("arqix.toml"),
        "[policies.render]\npdf-command = \"./fakepandoc.sh\"\ndocuments = [\n  { name = \"alpha\", path = \"alpha\", title = \"Alpha\" },\n  { name = \"beta\", path = \"beta\", title = \"Beta\" },\n]\n",
    )
    .unwrap();
    std::fs::create_dir_all(repo.join("docs/alpha")).unwrap();
    std::fs::write(
        repo.join("docs/alpha/index.md"),
        "---\nid: alpha\ntitle: Alpha\n---\n\n## Alpha\n\nAlpha body.\n",
    )
    .unwrap();
    std::fs::create_dir_all(repo.join("docs/beta")).unwrap();
    std::fs::write(
        repo.join("docs/beta/index.md"),
        "---\nid: beta\ntitle: Beta\n---\n\n## Beta\n\nBeta body.\n",
    )
    .unwrap();
    common::assert_success(&run_arqix_in(&repo, &["render", "pdf"]));
    let args = std::fs::read_to_string(repo.join("args.txt")).unwrap();
    assert!(
        args.contains("--metadata"),
        "the renderer is passed document metadata: {args}"
    );
    assert!(
        args.contains("title=Alpha") && args.contains("title=Beta"),
        "each document's own title is passed as metadata: {args}"
    );
}

// arqix:verifies REQ-04-01-03-07
#[test]
fn render_forwards_tool_errors_transparently() {
    let repo = scratch_copy("minimal", "render_forwards_tool_errors_transparently");
    let out = run_arqix_in(&repo, &["render", "pdf"]);
    if !out.status.success() {
        assert!(
            !out.stderr.is_empty(),
            "a failing render must forward the tool error, never swallow it"
        );
    }

    // A failing renderer is a system error naming the invocation, exactly
    // like a failing site toolchain (REQ-04-01-07-02's discipline).
    std::fs::write(
        repo.join("arqix.toml"),
        "[policies.render]\npdf-command = \"false\"\n",
    )
    .unwrap();
    let out = run_arqix_in(&repo, &["render", "pdf"]);
    assert_eq!(
        out.status.code(),
        Some(2),
        "a renderer failure is a system error, not a finding"
    );
    assert!(
        String::from_utf8_lossy(&out.stderr).contains("false"),
        "the diagnostic names the failing invocation"
    );
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

/// A scratch corpus with one workflow group: a story, a requirement
/// derived from it, and a test file whose marker verifies the requirement.
fn write_catalogue_fixture(repo: &std::path::Path) {
    std::fs::write(
        repo.join("arqix.toml"),
        "[policies.publish]\nsite-command = \"true\"\nspecification-catalogue = true\nexclude = [\"req\", \"stories\"]\n",
    )
    .unwrap();
    std::fs::create_dir_all(repo.join("docs/stories")).unwrap();
    std::fs::write(
        repo.join("docs/stories/US-42-01-01-catalogued.md"),
        "---\nid: US-42-01-01\ntitle: A Catalogued Story\niri: arqix:user-stories/us-42-01-01\nrdf:\n  type:\n    - arqix:classes/user-story\ntriples:\n  - predicate: arqix:properties/is-part-of-workflow\n    object: arqix:workflows/wf-42-01\n---\n\n## A Catalogued Story\n\nAs a reader, I want a catalogue.\n",
    )
    .unwrap();
    std::fs::create_dir_all(repo.join("docs/req")).unwrap();
    std::fs::write(
        repo.join("docs/req/REQ-42-01-01-01-catalogued.md"),
        "---\nid: REQ-42-01-01-01\ntitle: A Catalogued Requirement\niri: arqix:requirements/req-42-01-01-01\nrdf:\n  type:\n    - arqix:classes/functional-requirement\ntriples:\n  - predicate: arqix:properties/derived-from\n    object:\n      - arqix:user-stories/us-42-01-01\n---\n\n## Requirement\n\nThe system SHALL be catalogued.\n",
    )
    .unwrap();
    std::fs::create_dir_all(repo.join("tests")).unwrap();
    std::fs::write(
        repo.join("tests/catalogued.rs"),
        "// arqix:verifies REQ-42-01-01-01\n// arqix:verifies REQ-99-99-99-01\n#[test]\nfn catalogued() {}\n",
    )
    .unwrap();
}

// arqix:verifies REQ-04-01-17-01
#[test]
fn publish_site_stages_catalogue_pages_per_workflow_group() {
    let repo = scratch_copy(
        "minimal",
        "publish_site_stages_catalogue_pages_per_workflow_group",
    );
    write_catalogue_fixture(&repo);
    common::assert_success(&run_arqix_in(&repo, &["publish", "site"]));
    let page = std::fs::read_to_string(repo.join("site-src/specification/wf-42-01.md"))
        .expect("one catalogue page per workflow group");
    assert!(
        page.contains("US-42-01-01") && page.contains("REQ-42-01-01-01"),
        "the group page bundles the group's stories and requirements: {page}"
    );
    assert!(
        page.contains("The system SHALL be catalogued."),
        "requirement entries carry the obligation text: {page}"
    );
    assert!(
        !repo.join("site-src/stories").exists() && !repo.join("site-src/req").exists(),
        "the source files stay off the site — the catalogue replaces them"
    );
}

// arqix:verifies REQ-04-01-17-02
#[test]
fn catalogue_entries_carry_anchors_and_coverage_status() {
    let repo = scratch_copy(
        "minimal",
        "catalogue_entries_carry_anchors_and_coverage_status",
    );
    write_catalogue_fixture(&repo);
    common::assert_success(&run_arqix_in(&repo, &["publish", "site"]));
    let page =
        std::fs::read_to_string(repo.join("site-src/specification/wf-42-01.md")).expect("page");
    assert!(
        page.contains("<a id=\"US-42-01-01\"></a>")
            && page.contains("<a id=\"REQ-42-01-01-01\"></a>"),
        "every id gets a deep-linkable anchor: {page}"
    );
    assert!(
        page.contains("verified"),
        "the requirement's coverage status comes from the trace graph: {page}"
    );
}

// arqix:verifies REQ-04-01-17-03
#[test]
fn catalogue_pages_are_deterministic() {
    let repo = scratch_copy("minimal", "catalogue_pages_are_deterministic");
    write_catalogue_fixture(&repo);
    common::assert_success(&run_arqix_in(&repo, &["publish", "site"]));
    let first =
        std::fs::read_to_string(repo.join("site-src/specification/wf-42-01.md")).expect("page");
    common::assert_success(&run_arqix_in(&repo, &["publish", "site"]));
    let second =
        std::fs::read_to_string(repo.join("site-src/specification/wf-42-01.md")).expect("page");
    assert_eq!(first, second, "identical corpus, identical catalogue");
}

// arqix:verifies REQ-04-01-03-02
#[test]
fn staged_pages_do_not_duplicate_the_title_heading() {
    // Site toolchains render the frontmatter title as the page H1; the
    // corpus convention starts every body with `## <Title>`, so staging
    // must drop that leading heading or every page shows its title twice
    // (found on arqix.dev, 2026-07-12).
    let repo = scratch_copy("minimal", "staged_pages_do_not_duplicate_the_title_heading");
    std::fs::write(repo.join("arqix.toml"), NOOP_TOOLCHAIN).unwrap();
    std::fs::write(
        repo.join("docs/dup.md"),
        "---\nid: dup\ntitle: A Doubled Title\n---\n\n## A Doubled Title\n\nBody text.\n\n### A Doubled Title\n\nA deeper section may repeat the words.\n",
    )
    .unwrap();
    common::assert_success(&run_arqix_in(&repo, &["publish", "site"]));
    let staged = std::fs::read_to_string(repo.join("site-src/dup.md")).expect("staged page");
    assert!(
        staged.contains("title: \"A Doubled Title\""),
        "the title stays in the frontmatter: {staged}"
    );
    assert!(
        !staged.contains("## A Doubled Title\n") || staged.contains("### A Doubled Title"),
        "sanity: deeper headings survive"
    );
    assert!(
        !staged.lines().any(|l| l.trim() == "## A Doubled Title"),
        "the leading title heading is dropped — the toolchain renders the title: {staged}"
    );
    assert!(
        staged.contains("### A Doubled Title"),
        "only the leading duplicate goes; deeper headings stay: {staged}"
    );
}

// arqix:verifies REQ-04-01-19-01
#[test]
fn publish_site_stages_csv_artefacts_as_tables() {
    let repo = scratch_copy("minimal", "publish_site_stages_csv_artefacts_as_tables");
    std::fs::write(repo.join("arqix.toml"), NOOP_TOOLCHAIN).unwrap();
    std::fs::create_dir_all(repo.join("docs/reports")).unwrap();
    std::fs::write(
        repo.join("docs/reports/matrix.csv"),
        "requirement,kind\nREQ-1,functional\n\"a,b\",pipe|cell\n",
    )
    .unwrap();

    common::assert_success(&run_arqix_in(&repo, &["publish", "site"]));
    let staged =
        std::fs::read_to_string(repo.join("site-src/reports/matrix.md")).expect("staged table");
    assert!(
        staged.contains("| requirement | kind |"),
        "the CSV header becomes the table header: {staged}"
    );
    assert!(
        staged.contains("| REQ-1 | functional |"),
        "rows become table rows: {staged}"
    );
    assert!(
        staged.contains("| a,b | pipe\\|cell |"),
        "quoted fields and pipes survive escaped: {staged}"
    );
    assert!(
        !repo.join("site-src/reports/matrix.csv").exists(),
        "the raw CSV never reaches the staging tree"
    );
}

// arqix:verifies REQ-04-01-19-02
#[test]
fn publish_site_rewrites_csv_links_to_the_table_pages() {
    let repo = scratch_copy(
        "minimal",
        "publish_site_rewrites_csv_links_to_the_table_pages",
    );
    std::fs::write(repo.join("arqix.toml"), NOOP_TOOLCHAIN).unwrap();
    std::fs::create_dir_all(repo.join("docs/reports")).unwrap();
    std::fs::write(repo.join("docs/reports/matrix.csv"), "a,b\n1,2\n").unwrap();
    std::fs::write(
        repo.join("docs/reports/README.md"),
        "---\nid: reports-readme\ntitle: Reports\n---\n\n## Reports\n\nSee [matrix.csv](matrix.csv) for the data.\n",
    )
    .unwrap();

    common::assert_success(&run_arqix_in(&repo, &["publish", "site"]));
    let staged = std::fs::read_to_string(repo.join("site-src/reports/README.md")).expect("staged");
    assert!(
        staged.contains("[matrix.csv](matrix.md)"),
        "the CSV link points at the staged table page: {staged}"
    );
}

// arqix:verifies REQ-04-01-07-03
#[test]
fn publish_site_stages_a_linked_include_target_as_a_page() {
    // The scoreboard case: the landing page includes the unit AND the
    // catalog links it — it must exist at its own URL.
    let repo = scratch_copy(
        "minimal",
        "publish_site_stages_a_linked_include_target_as_a_page",
    );
    std::fs::write(repo.join("arqix.toml"), NOOP_TOOLCHAIN).unwrap();
    std::fs::write(repo.join("docs/unit.md"), "# Unit\n\ncontent\n").unwrap();
    std::fs::write(
        repo.join("docs/landing.md"),
        "---\nid: landing\ntitle: Landing\n---\n\n## Landing\n\n<!-- arqix:include unit.md -->\n",
    )
    .unwrap();
    std::fs::write(
        repo.join("docs/catalog.md"),
        "---\nid: catalog\ntitle: Catalog\n---\n\n## Catalog\n\nSee [the unit](unit.md).\n",
    )
    .unwrap();

    common::assert_success(&run_arqix_in(&repo, &["publish", "site"]));
    assert!(
        repo.join("site-src/unit.md").exists(),
        "a linked include target stays a standalone page"
    );
}

// arqix:verifies REQ-04-01-07-03
#[test]
fn publish_site_still_omits_unlinked_fragments() {
    let repo = scratch_copy("minimal", "publish_site_still_omits_unlinked_fragments");
    std::fs::write(repo.join("arqix.toml"), NOOP_TOOLCHAIN).unwrap();
    std::fs::write(repo.join("docs/fragment.md"), "just stitched text\n").unwrap();
    std::fs::write(
        repo.join("docs/page.md"),
        "---\nid: page-y\ntitle: Page\n---\n\n## Page\n\n<!-- arqix:include fragment.md -->\n",
    )
    .unwrap();

    common::assert_success(&run_arqix_in(&repo, &["publish", "site"]));
    assert!(
        !repo.join("site-src/fragment.md").exists(),
        "an unlinked fragment reaches the site only through its parent"
    );
}

// arqix:verifies REQ-04-01-07-05
#[test]
fn publish_site_removes_stale_staged_pages() {
    let repo = scratch_copy("minimal", "publish_site_removes_stale_staged_pages");
    std::fs::write(repo.join("arqix.toml"), NOOP_TOOLCHAIN).unwrap();
    std::fs::create_dir_all(repo.join("site-src")).unwrap();
    std::fs::write(repo.join("site-src/ghost.md"), "a stale staged page\n").unwrap();

    common::assert_success(&run_arqix_in(&repo, &["publish", "site"]));
    assert!(
        !repo.join("site-src/ghost.md").exists(),
        "staging starts from a clean tree"
    );
}

// arqix:verifies REQ-08-01-42-03
#[test]
fn publish_site_stages_the_graph_explorer() {
    // ADR-0020: the explorer reaches readers through the site without a
    // committed copy — staging regenerates it from the current corpus.
    let repo = scratch_copy("minimal", "publish_site_stages_the_graph_explorer");
    std::fs::write(repo.join("arqix.toml"), NOOP_TOOLCHAIN).unwrap();

    common::assert_success(&run_arqix_in(&repo, &["publish", "site"]));
    let staged = repo.join("site-src/graph.html");
    assert!(staged.is_file(), "the explorer page stages with the site");
    let page = std::fs::read_to_string(&staged).unwrap();
    assert!(
        page.contains("forceSimulation"),
        "the staged page embeds the layout engine"
    );
}
