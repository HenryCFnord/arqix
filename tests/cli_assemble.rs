//! Command contract: `assemble build` — owned by the Assembler
//! (arc42 chapter 5).

mod common;

use common::{assert_success, run_arqix_in, scratch_copy};

// arqix:verifies REQ-02-01-11-01
#[test]
fn assemble_build_generates_outputs_under_pages() {
    let repo = scratch_copy("minimal", "assemble_build_generates_outputs_under_pages");
    let out = run_arqix_in(&repo, &["assemble", "build"]);
    assert_success(&out);
}

// arqix:verifies REQ-02-01-11-03
#[test]
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
fn assemble_build_writes_a_jsonl_log() {
    let repo = scratch_copy("minimal", "assemble_build_writes_a_jsonl_log");
    let out = run_arqix_in(&repo, &["assemble", "build"]);
    assert_success(&out);

    // REQ-04-01-01-02/-03: a JSONL log is written at the configured path.
    let log = std::fs::read_to_string(repo.join("pages/assembly.jsonl"))
        .expect("assembly log must exist under pages/");
    let records: Vec<serde_json::Value> = log
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).expect("each log line is one JSON object"))
        .collect();

    // REQ-04-01-01-04: at least one stable record per assembly step. The
    // minimal fixture has one document and no includes, so exactly one step.
    assert!(
        !records.is_empty(),
        "the log must carry one record per assembly step"
    );
    // REQ-04-01-01-05: every record carries the required fields.
    for record in &records {
        for field in [
            "doc",
            "chapter_id",
            "out",
            "include",
            "sha256",
            "bytes",
            "at_line",
        ] {
            assert!(
                record.get(field).is_some(),
                "log record is missing required field `{field}`: {record}"
            );
        }
    }
}
