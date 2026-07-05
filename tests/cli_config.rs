//! Command contract: `config validate`, `config show` — owned by the
//! Config Resolver (arc42 chapter 5).

mod common;

use common::{assert_success, fixture, run_arqix_in, scratch_copy, stdout_json};

// arqix:verifies REQ-01-01-16-01
#[test]
fn config_validate_accepts_a_valid_configuration() {
    let out = run_arqix_in(&fixture("minimal"), &["config", "validate"]);
    assert_success(&out);
}

// arqix:verifies REQ-01-01-16-01
// arqix:verifies REQ-00-00-00-06
#[test]
fn config_validate_accepts_a_missing_file_as_pure_defaults() {
    // The effective configuration is defaults + overrides (REQ-00-00-00-06);
    // no arqix.toml simply means no overrides.
    let repo = scratch_copy("minimal", "config_validate_accepts_a_missing_file");
    std::fs::remove_file(repo.join("arqix.toml")).unwrap();

    let out = run_arqix_in(&repo, &["config", "validate"]);
    assert_success(&out);
}

// arqix:verifies REQ-01-01-16-01
// arqix:verifies REQ-01-01-16-03
#[test]
fn config_validate_identifies_the_failing_key() {
    let out = run_arqix_in(
        &fixture("broken-config"),
        &["config", "validate", "--format", "json"],
    );
    common::assert_findings(&out);
    let diagnostics = stdout_json(&out);
    let text = diagnostics.to_string();
    assert!(
        text.contains("roots"),
        "diagnostics must name the failing key: {diagnostics}"
    );
    assert!(
        text.contains("CFG-001"),
        "diagnostics must carry the stable code: {diagnostics}"
    );
}

// arqix:verifies REQ-01-01-16-02
#[test]
fn config_show_renders_the_effective_configuration_as_json() {
    let out = run_arqix_in(&fixture("minimal"), &["config", "show", "--format", "json"]);
    assert_success(&out);
    let effective = stdout_json(&out);
    assert!(effective.is_object());
    assert!(
        effective.get("roots").is_some(),
        "the effective configuration must include the roots default: {effective}"
    );
}
