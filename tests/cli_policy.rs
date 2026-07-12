//! Command contract: `policy check` — owned by the Policy Checker
//! (arc42 chapter 5).
//!
//! The changed-file list arrives as positional arguments (the external
//! list, e.g. from `git diff --name-only`); the policy comes from the
//! `[policies.change]` table in arqix.toml.

mod common;

use common::{run_arqix_in, scratch_copy, stdout_json};

/// A gate-mode policy: `docs/` allows the subtree, `README.md` exactly
/// that file.
const GATE_POLICY: &str = "[policies.change]\nallow = [\"docs/\", \"README.md\"]\n";

/// The same scope in warn-only mode (REQ-01-01-07-03).
const WARN_POLICY: &str = "[policies.change]\nallow = [\"docs/\"]\nmode = \"warn\"\n";

// arqix:verifies REQ-01-01-07-01
#[test]
fn policy_check_reads_the_declared_scope_from_the_policy_file() {
    let repo = scratch_copy(
        "minimal",
        "policy_check_reads_the_declared_scope_from_the_policy_file",
    );
    std::fs::write(repo.join("arqix.toml"), GATE_POLICY).unwrap();

    // A trailing slash declares a subtree, an exact entry that one file.
    let inside = run_arqix_in(&repo, &["policy", "check", "docs/note.md", "README.md"]);
    common::assert_success(&inside);

    // An exact entry is not a prefix: a sibling sharing the prefix is
    // outside the scope.
    let sibling = run_arqix_in(&repo, &["policy", "check", "README.md.bak"]);
    common::assert_findings(&sibling);
}

// arqix:verifies REQ-01-01-07-02
#[test]
fn policy_check_evaluates_changed_files_against_the_declared_scope() {
    let repo = scratch_copy(
        "minimal",
        "policy_check_evaluates_changed_files_against_the_declared_scope",
    );
    std::fs::write(repo.join("arqix.toml"), GATE_POLICY).unwrap();

    let out = run_arqix_in(
        &repo,
        &["policy", "check", "docs/inside.md", "src/outside.rs"],
    );
    common::assert_findings(&out);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("src/outside.rs"),
        "the out-of-scope file must be named: {stdout}"
    );
    assert!(
        !stdout.contains("docs/inside.md"),
        "the in-scope file must not be flagged: {stdout}"
    );
}

// arqix:verifies REQ-01-01-07-02
#[test]
fn policy_check_passes_when_no_policy_is_declared() {
    let repo = scratch_copy("minimal", "policy_check_passes_when_no_policy_is_declared");
    // The minimal fixture declares no [policies.change]: the mechanism is
    // optional — nothing to enforce, exit 0 with a note.
    let out = run_arqix_in(&repo, &["policy", "check", "src/anything.rs"]);
    common::assert_success(&out);
    assert!(
        String::from_utf8_lossy(&out.stdout).contains("no change policy"),
        "expected a note that no policy is declared"
    );
}

// arqix:verifies REQ-01-01-07-03
#[test]
fn policy_check_supports_warn_only_mode() {
    let repo = scratch_copy("minimal", "policy_check_supports_warn_only_mode");
    std::fs::write(repo.join("arqix.toml"), WARN_POLICY).unwrap();

    // Violations are reported, but warn-only mode never fails the check.
    let out = run_arqix_in(&repo, &["policy", "check", "src/outside.rs"]);
    common::assert_success(&out);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("src/outside.rs") && stdout.contains("warning"),
        "the violation must still be reported as a warning: {stdout}"
    );
}

// arqix:verifies REQ-00-00-00-03
// arqix:verifies REQ-01-01-07-02
#[test]
fn policy_check_reports_violations_as_structured_diagnostics() {
    let repo = scratch_copy(
        "minimal",
        "policy_check_reports_violations_as_structured_diagnostics",
    );
    std::fs::write(repo.join("arqix.toml"), GATE_POLICY).unwrap();

    let out = run_arqix_in(
        &repo,
        &["--format", "json", "policy", "check", "src/outside.rs"],
    );
    common::assert_findings(&out);
    let report = stdout_json(&out);
    assert_eq!(report["schema_version"], 1);
    let diags = report["diagnostics"].as_array().expect("diagnostics array");
    assert_eq!(diags.len(), 1);
    assert_eq!(diags[0]["severity"], "error");
    assert_eq!(diags[0]["code"], "POL-001");
    assert_eq!(diags[0]["file"], "src/outside.rs");
}

// arqix:no-requirement — usage-contract pin: the changed-file list is the
// command's input, so an empty invocation is a usage error, not a pass.
#[test]
fn policy_check_requires_at_least_one_file() {
    let repo = scratch_copy("minimal", "policy_check_requires_at_least_one_file");
    let out = run_arqix_in(&repo, &["policy", "check"]);
    assert_eq!(out.status.code(), Some(2));
}
