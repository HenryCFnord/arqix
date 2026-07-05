//! Command contract: `config validate`, `config show` — owned by the
//! Config Resolver (arc42 chapter 5).

mod common;

use common::{assert_success, fixture, run_arqix_in, stdout_json};

// arqix:verifies REQ-01-01-16-01
#[test]
#[ignore = "US-01-01-16: not implemented"]
fn config_validate_accepts_a_valid_configuration() {
    let out = run_arqix_in(&fixture("minimal"), &["config", "validate"]);
    assert_success(&out);
}

// arqix:verifies REQ-01-01-16-01
// arqix:verifies REQ-01-01-16-03
#[test]
#[ignore = "US-01-01-16: not implemented"]
fn config_validate_identifies_the_failing_key() {
    let out = run_arqix_in(
        &fixture("broken-config"),
        &["config", "validate", "--format", "json"],
    );
    common::assert_findings(&out);
    let diagnostics = stdout_json(&out);
    assert!(
        diagnostics.to_string().contains("roots"),
        "diagnostics must name the failing key: {diagnostics}"
    );
}

// arqix:verifies REQ-01-01-16-02
#[test]
#[ignore = "US-01-01-16: not implemented"]
fn config_show_renders_the_effective_configuration_as_json() {
    let out = run_arqix_in(&fixture("minimal"), &["config", "show", "--format", "json"]);
    assert_success(&out);
    let effective = stdout_json(&out);
    assert!(effective.is_object());
}
