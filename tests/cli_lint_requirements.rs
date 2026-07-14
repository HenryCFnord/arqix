//! Command contract: `lint requirements` — the ported requirements checker
//! (REQ-01-01-11-06). Owned by the Linter noun family (arc42 chapter 5,
//! ADR-0005); the Python checker `scripts/check_requirements.py` remains the
//! conformance oracle for the grace period (arc42 chapter 8, oracle policy).

mod common;

use common::{assert_findings, run_arqix_in, scratch_copy};

const STORY: &str = r#"---
id: US-09-09-09
title: Sample Story
slug: sample-story
iri: arqix:user-stories/us-09-09-09

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-requirement
    object: arqix:requirements/req-09-09-09-01

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-13
  updated: 2026-07-13
  lang: en
  generated: false
---

## Sample Story
"#;

// A requirement that is structurally clean but whose normative sentence uses
// a forbidden RFC 2119 keyword: it triggers EARS-002/003/004 and nothing else.
const REQ: &str = r#"---
id: REQ-09-09-09-01
title: Sample
slug: sample
iri: arqix:requirements/req-09-09-09-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object: arqix:user-stories/us-09-09-09

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-13
  updated: 2026-07-13
  lang: en
  generated: false
---

## Requirement

The tool MUST reject unknown flags.
"#;

// arqix:verifies REQ-01-01-11-06
#[test]
fn lint_requirements_reports_authoring_violations_as_json() {
    let repo = scratch_copy(
        "minimal",
        "lint_requirements_reports_authoring_violations_as_json",
    );

    let story_dir = repo.join("docs/en/architecture/stories");
    let req_dir = repo.join("docs/en/architecture/req");
    std::fs::create_dir_all(&story_dir).unwrap();
    std::fs::create_dir_all(&req_dir).unwrap();
    std::fs::write(story_dir.join("US-09-09-09-sample-story.md"), STORY).unwrap();
    std::fs::write(req_dir.join("REQ-09-09-09-01-sample.md"), REQ).unwrap();

    let out = run_arqix_in(&repo, &["lint", "requirements", "--format", "json"]);
    // Any finding — the forbidden keyword is one — makes the check exit 1.
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
    // The representative finding: EARS-003 against the forbidden keyword, with
    // the oracle's exact message string and the requirement's path.
    assert!(
        findings.iter().any(|f| f["rule"] == "EARS-003"
            && f["level"] == "error"
            && f["path"] == "docs/en/architecture/req/REQ-09-09-09-01-sample.md"
            && f["message"] == "forbidden keyword 'MUST'; use the SHALL/SHOULD/MAY subset"),
        "expected the forbidden-keyword finding: {report}"
    );
    // The forbidden sentence also fails to classify (EARS-002) and carries no
    // allowed keyword (EARS-004): three error findings, no warnings.
    assert_eq!(report["summary"]["errors"], 3, "{report}");
    assert_eq!(report["summary"]["warnings"], 0, "{report}");
}
