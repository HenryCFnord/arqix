//! Command contract: `trace freshness` — owned by the Trace Engine (arc42
//! chapter 5, US-03-01-11, ADR-0015). A `verifies`/`implements` marker is
//! possibly stale when its target requirement or owning story was committed
//! to version control after the marker's own file.
//!
//! Deliberately outside the `cli_trace` conformance suite: freshness is a new
//! engine-only analysis with no counterpart in the frozen Python oracle, so
//! it cannot perturb the ported `trace scan/coverage/matrix` surface (the
//! `--results` and `ratchet` precedents). Freshness reads git history, so
//! these tests build a repository with pinned commit dates and stay
//! deterministic.

mod common;

use common::{run_arqix_in, stdout_json};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A `verifies` marker line, assembled so no source line here starts with
/// `//` — the marker gate and trace engine must never read this literal as a
/// marker of this test file.
fn marker_rs() -> String {
    format!("// arqix:{} REQ-99-99-99-01\nfn t() {{}}\n", "verifies")
}

const REQ_MD: &str = "---\nid: REQ-99-99-99-01\n---\nbody\n";

/// A fresh, empty scratch directory under the test target dir.
fn fresh_dir(name: &str) -> PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    if dir.exists() {
        fs::remove_dir_all(&dir).expect("clear scratch dir");
    }
    fs::create_dir_all(&dir).expect("create scratch dir");
    dir
}

fn write(dir: &Path, rel: &str, text: &str) {
    let path = dir.join(rel);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create parent dir");
    }
    fs::write(path, text).expect("write file");
}

/// Run a git command in `dir` with both author and committer date pinned, so
/// commit timestamps — and therefore freshness output — are byte-stable.
fn git(dir: &Path, date: &str, args: &[&str]) {
    let out = Command::new("git")
        .current_dir(dir)
        .args(args)
        .env("GIT_AUTHOR_DATE", date)
        .env("GIT_COMMITTER_DATE", date)
        .env("GIT_AUTHOR_NAME", "t")
        .env("GIT_AUTHOR_EMAIL", "t@example.com")
        .env("GIT_COMMITTER_NAME", "t")
        .env("GIT_COMMITTER_EMAIL", "t@example.com")
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

const T1: &str = "2026-01-01T00:00:00";
const T2: &str = "2026-06-01T00:00:00";

// arqix:verifies REQ-03-01-11-01
#[test]
fn freshness_flags_a_marker_whose_requirement_changed_after_the_test() {
    let dir = fresh_dir("freshness_requirement_newer");
    git(&dir, T1, &["init", "-q"]);
    write(&dir, "t.rs", &marker_rs());
    write(&dir, "docs/req.md", REQ_MD);
    git(&dir, T1, &["add", "-A"]);
    git(&dir, T1, &["commit", "-q", "-m", "initial"]);
    // The requirement is revised and committed later than the test file.
    write(
        &dir,
        "docs/req.md",
        "---\nid: REQ-99-99-99-01\n---\nbody revised\n",
    );
    git(&dir, T2, &["add", "docs/req.md"]);
    git(&dir, T2, &["commit", "-q", "-m", "revise requirement"]);

    let out = run_arqix_in(&dir, &["trace", "freshness", "--format", "json"]);
    let report = stdout_json(&out);
    let stale = report["stale"].as_array().expect("stale array");
    assert!(
        stale.iter().any(|s| s["marker"] == "t.rs:1"),
        "the marker is stale — its requirement is the later commit: {report}"
    );
    assert_eq!(
        out.status.code(),
        Some(1),
        "stale markers are a finding: {report}"
    );
}

// arqix:verifies REQ-03-01-11-01
#[test]
fn freshness_is_clean_when_the_test_is_the_later_commit() {
    let dir = fresh_dir("freshness_test_newer");
    git(&dir, T1, &["init", "-q"]);
    write(&dir, "docs/req.md", REQ_MD);
    git(&dir, T1, &["add", "-A"]);
    git(&dir, T1, &["commit", "-q", "-m", "requirement"]);
    // The verifying test is added after the requirement — current, not stale.
    write(&dir, "t.rs", &marker_rs());
    git(&dir, T2, &["add", "t.rs"]);
    git(&dir, T2, &["commit", "-q", "-m", "add test"]);

    let out = run_arqix_in(&dir, &["trace", "freshness", "--format", "json"]);
    let report = stdout_json(&out);
    assert_eq!(
        report["summary"]["evaluated"], 1,
        "the active marker was evaluated: {report}"
    );
    assert_eq!(
        report["stale"].as_array().expect("stale array").len(),
        0,
        "the test is the later commit, so nothing is stale: {report}"
    );
    assert_eq!(out.status.code(), Some(0), "no findings: {report}");
}

// arqix:verifies REQ-03-01-11-02
#[test]
fn freshness_degrades_without_version_control() {
    let dir = fresh_dir("freshness_no_git");
    // A corpus with a marker, but no `.git` at all.
    write(&dir, "t.rs", &marker_rs());
    write(&dir, "docs/req.md", REQ_MD);

    let out = run_arqix_in(&dir, &["trace", "freshness", "--format", "json"]);
    let report = stdout_json(&out);
    assert_eq!(
        report["summary"]["evaluated"], 1,
        "the marker is considered even without history: {report}"
    );
    assert_eq!(
        report["stale"].as_array().expect("stale array").len(),
        0,
        "no history means nothing is reported stale: {report}"
    );
    assert_eq!(out.status.code(), Some(0), "degradation is not a failure");
}

// arqix:verifies REQ-03-01-11-03
#[test]
fn verify_runs_freshness_as_an_informational_step() {
    let dir = fresh_dir("freshness_verify_informational");
    git(&dir, T1, &["init", "-q"]);
    write(&dir, "t.rs", &marker_rs());
    write(&dir, "docs/req.md", REQ_MD);
    git(&dir, T1, &["add", "-A"]);
    git(&dir, T1, &["commit", "-q", "-m", "initial"]);
    write(
        &dir,
        "docs/req.md",
        "---\nid: REQ-99-99-99-01\n---\nbody revised\n",
    );
    git(&dir, T2, &["add", "docs/req.md"]);
    git(&dir, T2, &["commit", "-q", "-m", "revise requirement"]);
    // Run only freshness, declared informational.
    write(
        &dir,
        "arqix.toml",
        "[policies.verify]\nsteps = [\"freshness\"]\ninformational = [\"freshness\"]\n",
    );

    let out = run_arqix_in(&dir, &["verify", "--format", "json"]);
    let report = stdout_json(&out);
    let step = report["steps"]
        .as_array()
        .expect("steps array")
        .iter()
        .find(|s| s["step"] == "freshness")
        .expect("freshness step present")
        .clone();
    assert_eq!(
        step["exit_code"], 1,
        "freshness found a stale marker: {report}"
    );
    assert_eq!(
        step["informational"], true,
        "declared informational: {report}"
    );
    assert_eq!(
        out.status.code(),
        Some(0),
        "an informational freshness finding must not fail the loop: {report}"
    );
}
