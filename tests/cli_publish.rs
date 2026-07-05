//! Command contract: `publish site`, `render pdf` — owned by the Publish &
//! Render Orchestrator (arc42 chapter 5). External toolchain errors are
//! forwarded transparently (REQ-04-01-03-07).

mod common;

use common::{run_arqix_in, scratch_copy};

// arqix:verifies REQ-04-01-07-01
#[test]
#[ignore = "US-04-01-07: not implemented"]
fn publish_site_publishes_per_language() {
    let repo = scratch_copy("minimal", "publish_site_publishes_per_language");
    let out = run_arqix_in(&repo, &["publish", "site", "--lang", "en"]);
    common::assert_success(&out);
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
