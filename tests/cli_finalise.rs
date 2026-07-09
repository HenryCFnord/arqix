//! Command contract: `finalise` — owned by the Formatter & Finaliser with
//! an injected clock, never an ambient system call (ADR-0004).

mod common;

use common::{assert_success, run_arqix_in, scratch_copy};
use std::fs;

// arqix:verifies REQ-01-01-06-01
#[test]
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

// arqix:verifies REQ-01-01-06-01
#[test]
fn finalise_rejects_a_non_iso_date() {
    let repo = scratch_copy("minimal", "finalise_rejects_a_non_iso_date");
    let doc = repo.join("docs/REQ-99-99-99-01-fixture-requirement.md");
    let before = fs::read_to_string(&doc).unwrap();

    // Wrong shape and impossible calendar dates are both usage errors
    // (REQ-01-01-06-01 mandates an ISO-8601 YYYY-MM-DD date).
    for bad in ["31.01.2027", "2027-1-31", "2026-13-40", "2026-02-30"] {
        let out = run_arqix_in(&repo, &["finalise", "--date", bad]);
        assert_eq!(
            out.status.code(),
            Some(2),
            "--date {bad} must be rejected as a usage error"
        );
    }
    assert_eq!(
        before,
        fs::read_to_string(&doc).unwrap(),
        "a rejected date must not touch any document"
    );
}

// arqix:verifies REQ-01-01-06-01
#[test]
fn finalise_touches_only_the_meta_updated_field() {
    let repo = scratch_copy("minimal", "finalise_touches_only_the_meta_updated_field");
    // An `updated:` key outside `meta` (here under `properties`) is data,
    // not lifecycle metadata, and must survive finalise untouched.
    fs::write(
        repo.join("docs/tricky.md"),
        "---\nid: DOC-99\ntitle: Tricky\nproperties:\n  updated: never\nmeta:\n  updated: 2026-01-01\n---\nbody\n",
    )
    .unwrap();

    assert_success(&run_arqix_in(&repo, &["finalise", "--date", "2027-01-31"]));

    let after = fs::read_to_string(repo.join("docs/tricky.md")).unwrap();
    assert!(
        after.contains("  updated: never"),
        "properties.updated must not be rewritten: {after}"
    );
    assert!(
        after.contains("  updated: 2027-01-31"),
        "meta.updated must carry the injected date: {after}"
    );
}

// arqix:verifies REQ-01-01-06-03
#[test]
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
