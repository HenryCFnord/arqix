//! Command contract: `finalise` — owned by the Formatter & Finaliser with
//! an injected clock, never an ambient system call (ADR-0004).

mod common;

use common::{assert_success, run_arqix_in, scratch_copy};
use std::fs;

// arqix:verifies REQ-01-01-06-01
#[test]
#[ignore = "US-01-01-06: not implemented"]
fn finalise_sets_updated_to_the_injected_date() {
    let repo = scratch_copy("minimal", "finalise_sets_updated_to_the_injected_date");
    let doc = repo.join("docs/REQ-99-99-99-01-fixture-requirement.md");

    // The date is an injected dependency (ADR-0004): tests supply it and
    // must never depend on the wall clock.
    let out = run_arqix_in(&repo, &["finalise", "--date", "2027-01-31"]);
    assert_success(&out);

    let content = fs::read_to_string(&doc).unwrap();
    assert!(content.contains("updated: 2027-01-31"));
}

// arqix:verifies REQ-01-01-06-02
#[test]
#[ignore = "US-01-01-06: not implemented"]
fn finalise_leaves_current_metadata_untouched() {
    let repo = scratch_copy("minimal", "finalise_leaves_current_metadata_untouched");
    let doc = repo.join("docs/REQ-99-99-99-01-fixture-requirement.md");
    let before = fs::read_to_string(&doc).unwrap();

    // The fixture document already carries `updated: 2026-07-04`.
    assert_success(&run_arqix_in(&repo, &["finalise", "--date", "2026-07-04"]));

    let after = fs::read_to_string(&doc).unwrap();
    assert_eq!(
        before, after,
        "already-current metadata must not be rewritten"
    );
}

// arqix:verifies REQ-01-01-06-03
#[test]
#[ignore = "US-01-01-06: not implemented"]
fn finalise_fails_clearly_on_unsupported_frontmatter() {
    let repo = scratch_copy(
        "minimal",
        "finalise_fails_clearly_on_unsupported_frontmatter",
    );
    fs::write(repo.join("docs/broken.md"), "no frontmatter at all\n").unwrap();

    let out = run_arqix_in(&repo, &["finalise", "--date", "2027-01-31"]);
    common::assert_findings(&out);
    assert!(
        String::from_utf8_lossy(&out.stderr).contains("broken.md")
            || String::from_utf8_lossy(&out.stdout).contains("broken.md"),
        "the diagnostic must name the offending file"
    );
}
