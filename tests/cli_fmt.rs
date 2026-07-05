//! Command contract: `fmt` — owned by the Formatter & Finaliser, the only
//! mutator of existing source documents (arc42 chapter 5, ADR-0004).

mod common;

use common::{assert_success, run_arqix_in, scratch_copy};
use std::fs;

// arqix:verifies REQ-01-01-03-01
#[test]
#[ignore = "US-01-01-03: not implemented"]
fn fmt_sorts_frontmatter_keys_canonically() {
    let repo = scratch_copy("minimal", "fmt_sorts_frontmatter_keys_canonically");
    let out = run_arqix_in(&repo, &["fmt"]);
    assert_success(&out);
}

// arqix:verifies REQ-01-01-03-02
// arqix:verifies REQ-01-01-03-03
#[test]
#[ignore = "US-01-01-03: not implemented"]
fn fmt_never_changes_body_text() {
    let repo = scratch_copy("minimal", "fmt_never_changes_body_text");
    let doc = repo.join("docs/REQ-99-99-99-01-fixture-requirement.md");
    let before = fs::read_to_string(&doc).unwrap();
    let body_before = before.split("---").nth(2).unwrap().to_string();

    assert_success(&run_arqix_in(&repo, &["fmt"]));

    let after = fs::read_to_string(&doc).unwrap();
    let body_after = after.split("---").nth(2).unwrap();
    assert_eq!(body_before.trim(), body_after.trim());
}

// arqix:verifies REQ-00-00-00-01
#[test]
#[ignore = "US-01-01-03: not implemented"]
fn fmt_is_idempotent() {
    let repo = scratch_copy("minimal", "fmt_is_idempotent");
    let doc = repo.join("docs/REQ-99-99-99-01-fixture-requirement.md");

    assert_success(&run_arqix_in(&repo, &["fmt"]));
    let first = fs::read_to_string(&doc).unwrap();
    assert_success(&run_arqix_in(&repo, &["fmt"]));
    let second = fs::read_to_string(&doc).unwrap();

    assert_eq!(first, second, "a second fmt run must be a no-op");
}
