//! Command contract: `policy check` — owned by the Policy Checker
//! (arc42 chapter 5).

mod common;

use common::{run_arqix_in, scratch_copy};

// arqix:verifies REQ-01-01-07-02
#[test]
#[ignore = "US-01-01-07: not implemented"]
fn policy_check_evaluates_changed_files_against_the_declared_scope() {
    let repo = scratch_copy(
        "minimal",
        "policy_check_evaluates_changed_files_against_the_declared_scope",
    );
    let out = run_arqix_in(&repo, &["policy", "check"]);
    // Without a declared policy the check passes; scoped fixtures land
    // with the red phase of US-01-01-07.
    common::assert_success(&out);
}

// arqix:verifies REQ-01-01-07-03
#[test]
#[ignore = "US-01-01-07: not implemented"]
fn policy_check_supports_warn_only_mode() {
    let repo = scratch_copy("minimal", "policy_check_supports_warn_only_mode");
    let out = run_arqix_in(&repo, &["policy", "check", "--warn-only"]);
    common::assert_success(&out);
}
