//! Command contract: `assemble build` — owned by the Assembler
//! (arc42 chapter 5).

mod common;

use common::{assert_success, run_arqix_in, scratch_copy};

// arqix:verifies REQ-02-01-11-01
#[test]
#[ignore = "US-02-01-11: not implemented"]
fn assemble_build_generates_outputs_under_pages() {
    let repo = scratch_copy("minimal", "assemble_build_generates_outputs_under_pages");
    let out = run_arqix_in(&repo, &["assemble", "build"]);
    assert_success(&out);
}

// arqix:verifies REQ-02-01-11-03
#[test]
#[ignore = "US-02-01-11: not implemented"]
fn assemble_build_fails_clearly_on_include_cycles() {
    let repo = scratch_copy("minimal", "assemble_build_fails_clearly_on_include_cycles");
    std::fs::write(
        repo.join("docs/cycle-a.md"),
        "<!-- arqix:include cycle-b.md -->\n",
    )
    .unwrap();
    std::fs::write(
        repo.join("docs/cycle-b.md"),
        "<!-- arqix:include cycle-a.md -->\n",
    )
    .unwrap();

    let out = run_arqix_in(&repo, &["assemble", "build"]);
    common::assert_findings(&out);
    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        combined.contains("cycle"),
        "the diagnostic must name the cycle"
    );
}

// arqix:verifies REQ-04-01-01-02
#[test]
#[ignore = "US-04-01-01: not implemented"]
fn assemble_build_writes_a_jsonl_log() {
    let repo = scratch_copy("minimal", "assemble_build_writes_a_jsonl_log");
    let out = run_arqix_in(&repo, &["assemble", "build"]);
    assert_success(&out);
    // Log path and per-record fields are pinned by REQ-04-01-01-03/-05
    // and asserted in detail with the red phase of US-04-01-01.
}
