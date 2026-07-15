//! Command contract: `lint frontmatter` — the ported frontmatter, formatting,
//! and ontology-vocabulary checker (REQ-01-01-11-07). Owned by the Linter noun
//! family (arc42 chapter 5, ADR-0005); ported from the retired Python checker
//! `scripts/check_frontmatter.py` (arc42 chapter 8, oracle policy; the oracle's
//! selftest fixtures are mirrored inline in src/checkers/frontmatter.rs).

mod common;

use common::{assert_findings, run_arqix_in, scratch_copy};

// A structurally clean story whose only defect is a non-`en` language: it
// triggers exactly one FMT-006 finding and nothing else. The rdf.type is the
// external `rdfs:Class` so the fixture needs no ontology documents to satisfy
// the ONT-* vocabulary checks.
const STORY: &str = r#"---
id: US-09-09-09
title: Sample Story
slug: sample-story
iri: arqix:user-stories/us-09-09-09

rdf:
  type:
    - rdfs:Class

triples: []

properties: {}

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-13
  updated: 2026-07-13
  lang: de
  generated: false
---

## Sample Story
"#;

// arqix:verifies REQ-01-01-11-07
#[test]
fn lint_frontmatter_reports_contract_violations_as_json() {
    let repo = scratch_copy(
        "minimal",
        "lint_frontmatter_reports_contract_violations_as_json",
    );

    // The checker refuses to run without an ontology tree (exit 2); an empty
    // directory is enough to clear that guard for this architecture-only case.
    std::fs::create_dir_all(repo.join("docs/ontology")).unwrap();
    let story_dir = repo.join("docs/en/architecture/stories");
    std::fs::create_dir_all(&story_dir).unwrap();
    std::fs::write(story_dir.join("US-09-09-09-sample-story.md"), STORY).unwrap();

    let out = run_arqix_in(&repo, &["lint", "frontmatter", "--format", "json"]);
    // Any finding — the language mismatch is one — makes the check exit 1.
    assert_findings(&out);

    let report: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap_or_else(|e| {
        panic!(
            "stdout is not valid JSON ({e}): {}",
            String::from_utf8_lossy(&out.stdout)
        )
    });

    // The oracle's JSON shape: a `findings` array and a `summary` object.
    for key in ["findings", "summary"] {
        assert!(
            report.get(key).is_some(),
            "missing top-level key {key}: {report}"
        );
    }

    let findings = report["findings"].as_array().expect("findings array");
    // The representative finding: FMT-006 against the non-`en` language, with
    // the oracle's exact message string and the document's path.
    assert!(
        findings.iter().any(|f| f["rule"] == "FMT-006"
            && f["level"] == "error"
            && f["path"] == "docs/en/architecture/stories/US-09-09-09-sample-story.md"
            && f["message"] == "meta.lang 'de', expected 'en'"),
        "expected the language-mismatch finding: {report}"
    );
    // The clean-but-for-language fixture yields exactly one error, no warnings.
    assert_eq!(report["summary"]["errors"], 1, "{report}");
    assert_eq!(report["summary"]["warnings"], 0, "{report}");
}
