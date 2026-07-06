//! Live contract tests for the CLI surface itself: argument parsing,
//! the usage-error leg of the exit-code contract, and stub behaviour.

mod common;

use common::run_arqix;

// arqix:no-requirement
#[test]
fn prints_help_for_help_flag() {
    let output = run_arqix(&["--help"]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("Usage:"));
    assert!(stdout.contains("verify"));
    assert!(stdout.contains("--format"));
}

// arqix:no-requirement
#[test]
fn prints_version_for_version_flag() {
    let output = run_arqix(&["--version"]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("arqix"));
}

// arqix:verifies REQ-00-00-00-02
#[test]
fn usage_error_exits_with_code_2() {
    let unknown_flag = run_arqix(&["--nope"]);
    assert_eq!(unknown_flag.status.code(), Some(2));
    assert!(!unknown_flag.stderr.is_empty());

    let unknown_command = run_arqix(&["frobnicate"]);
    assert_eq!(unknown_command.status.code(), Some(2));

    let missing_subcommand = run_arqix(&["doc"]);
    assert_eq!(missing_subcommand.status.code(), Some(2));
}

// arqix:verifies REQ-04-01-10-01
#[test]
fn format_option_is_accepted_globally() {
    // The option must parse on any command (exit code 2 would mean a usage
    // error); the stub exit code 70 proves parsing succeeded. `mcp serve` is
    // a phase-5 command still stubbed at this point.
    let output = run_arqix(&["--format", "json", "mcp", "serve"]);
    assert_eq!(output.status.code(), Some(70));

    let after_subcommand = run_arqix(&["mcp", "serve", "--format", "json"]);
    assert_eq!(after_subcommand.status.code(), Some(70));

    let invalid_value = run_arqix(&["--format", "yaml", "mcp", "serve"]);
    assert_eq!(invalid_value.status.code(), Some(2));
}

// arqix:no-requirement
#[test]
fn unimplemented_commands_exit_outside_the_stable_contract() {
    // Stubs must never be mistaken for a real result: exit code 70 is
    // deliberately outside the stable 0/1/2 contract. `mcp serve` is a
    // phase-5 command still stubbed at this point.
    let output = run_arqix(&["mcp", "serve"]);
    assert_eq!(output.status.code(), Some(70));
    assert!(String::from_utf8_lossy(&output.stderr).contains("not implemented"));
}
