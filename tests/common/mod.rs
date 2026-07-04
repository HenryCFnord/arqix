//! Shared helpers for the command-contract integration tests.
//!
//! Every test in `tests/cli_*.rs` maps to a row of the command-ownership
//! table in arc42 chapter 5 and carries an `arqix:verifies REQ-…` marker
//! (validated by `scripts/check_trace_markers.py`). Tests for
//! not-yet-implemented stories are `#[ignore]`d; implementing a story
//! test-first means removing the ignore, showing the red run, then coding
//! until green (see AGENTS.md, "Test-driven implementation").
#![allow(dead_code)]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// Path to a fixture repository under `tests/fixtures/`.
pub fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

/// Copy a fixture repository into a per-test scratch directory so tests of
/// mutating commands never touch the shared fixture. The directory is
/// recreated on every run.
pub fn scratch_copy(fixture_name: &str, test_name: &str) -> PathBuf {
    let dest = Path::new(env!("CARGO_TARGET_TMPDIR")).join(test_name);
    if dest.exists() {
        fs::remove_dir_all(&dest).expect("failed to clear scratch dir");
    }
    copy_dir(&fixture(fixture_name), &dest);
    dest
}

fn copy_dir(src: &Path, dest: &Path) {
    fs::create_dir_all(dest).expect("failed to create scratch dir");
    for entry in fs::read_dir(src).expect("failed to read fixture dir") {
        let entry = entry.expect("failed to read fixture entry");
        let target = dest.join(entry.file_name());
        if entry
            .file_type()
            .expect("failed to stat fixture entry")
            .is_dir()
        {
            copy_dir(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), &target).expect("failed to copy fixture file");
        }
    }
}

/// Run the arqix binary with `args` in the current directory.
pub fn run_arqix(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_arqix"))
        .args(args)
        .output()
        .expect("failed to run arqix")
}

/// Run the arqix binary with `args` inside `dir` (usually a fixture repo).
pub fn run_arqix_in(dir: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_arqix"))
        .current_dir(dir)
        .args(args)
        .output()
        .expect("failed to run arqix")
}

/// Assert exit code 0, printing stderr on failure.
pub fn assert_success(output: &Output) {
    assert_eq!(
        output.status.code(),
        Some(0),
        "expected success, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

/// Assert exit code 1 — findings / quality-gate failure (REQ-00-00-00-02).
pub fn assert_findings(output: &Output) {
    assert_eq!(
        output.status.code(),
        Some(1),
        "expected findings exit code, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

/// Parse stdout as JSON, failing the test with the raw output on error.
pub fn stdout_json(output: &Output) -> serde_json::Value {
    serde_json::from_slice(&output.stdout).unwrap_or_else(|e| {
        panic!(
            "stdout is not valid JSON ({e}): {}",
            String::from_utf8_lossy(&output.stdout)
        )
    })
}
