use std::process::Command;

fn run_arqix(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_arqix"))
        .args(args)
        .output()
        .expect("failed to run arqix")
}

#[test]
fn prints_help_for_help_flag() {
    let output = run_arqix(&["--help"]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("Usage:"));
    assert!(stdout.contains("--help"));
}

#[test]
fn prints_version_for_version_flag() {
    let output = run_arqix(&["--version"]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("arqix v"));
}

#[test]
fn unknown_argument_exits_with_error() {
    let output = run_arqix(&["--nope"]);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(!output.status.success());
    assert_eq!(output.status.code(), Some(2));
    assert!(stderr.contains("unrecognized argument `--nope`"));
}
