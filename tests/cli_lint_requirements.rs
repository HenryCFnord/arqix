//! Command contract: `lint requirements` — the ported requirements checker
//! (REQ-01-01-11-06). Owned by the Linter noun family (arc42 chapter 5,
//! ADR-0005); ported from the retired Python checker
//! `scripts/check_requirements.py` (arc42 chapter 8, oracle policy; the
//! oracle's selftest fixtures are mirrored inline in src/checkers/requirements.rs).

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

    // The shared diagnostics payload (REQ-04-01-10-03) — the oracle's
    // findings/summary shape retired with its oracle.
    assert_eq!(report["schema_version"], 1, "{report}");
    let diagnostics = report["diagnostics"].as_array().expect("diagnostics array");
    // The representative finding: EARS-003 against the forbidden keyword, with
    // the exact message string and the requirement's path.
    assert!(
        diagnostics.iter().any(|d| d["code"] == "EARS-003"
            && d["severity"] == "error"
            && d["file"] == "docs/en/architecture/req/REQ-09-09-09-01-sample.md"
            && d["message"] == "forbidden keyword 'MUST'; use the SHALL/SHOULD/MAY subset"),
        "expected the forbidden-keyword finding: {report}"
    );
    // The forbidden sentence also fails to classify (EARS-002) and carries no
    // allowed keyword (EARS-004): three error findings, no warnings.
    // The fixture yields exactly three errors, no warnings.
    assert_eq!(
        diagnostics
            .iter()
            .filter(|d| d["severity"] == "error")
            .count(),
        3,
        "{report}"
    );
    assert_eq!(diagnostics.len(), 3, "{report}");
}

// arqix:verifies REQ-01-01-19-03
#[test]
fn lint_requirements_resolves_the_configured_required_meta() {
    // One source (ADR-0011): REQ-META-001 checks the effective
    // [kinds.req].required-meta contract, not a hardcoded key set.
    let repo = scratch_copy(
        "minimal",
        "lint_requirements_resolves_the_configured_required_meta",
    );
    std::fs::write(
        repo.join("arqix.toml"),
        "[kinds.req]\ndir = \"docs/en/architecture/req\"\nrequired-meta = [\"owner\", \"lang\"]\n",
    )
    .unwrap();
    let story_dir = repo.join("docs/en/architecture/stories");
    let req_dir = repo.join("docs/en/architecture/req");
    std::fs::create_dir_all(&story_dir).unwrap();
    std::fs::create_dir_all(&req_dir).unwrap();
    std::fs::write(story_dir.join("US-09-09-09-sample-story.md"), STORY).unwrap();
    std::fs::write(
        req_dir.join("REQ-09-09-09-01-sample.md"),
        "---\nid: REQ-09-09-09-01\ntitle: Sample\nslug: sample\niri: arqix:requirements/req-09-09-09-01\n\nrdf:\n  type:\n    - arqix:classes/functional-requirement\n\ntriples:\n  - predicate: arqix:properties/derived-from\n    object: arqix:user-stories/us-09-09-09\n\nmeta:\n  owner: hcf\n  lang: en\n---\n\n## Requirement\n\nWhen `arqix lint requirements` runs, arqix SHALL honour the configured contract.\n",
    )
    .unwrap();

    // The shrunk contract is satisfied: no REQ-META-001 findings.
    let out = run_arqix_in(&repo, &["lint", "requirements"]);
    common::assert_success(&out);

    // A grown contract bites: a missing configured key is a finding.
    std::fs::write(
        repo.join("arqix.toml"),
        "[kinds.req]\ndir = \"docs/en/architecture/req\"\nrequired-meta = [\"owner\", \"lang\", \"reviewed\"]\n",
    )
    .unwrap();
    let out = run_arqix_in(&repo, &["lint", "requirements"]);
    assert_findings(&out);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("meta.reviewed"),
        "the configured key is named: {stdout}"
    );
}

// arqix:verifies REQ-08-01-31-01
#[test]
fn lint_requirements_binds_coupling_rules_to_the_story_module() {
    // ADR-0017 process profiles: the story-workflow coupling rules run
    // exactly when the story-driven module is effective.
    let repo = scratch_copy(
        "minimal",
        "lint_requirements_binds_coupling_rules_to_the_story_module",
    );
    let story_dir = repo.join("docs/en/architecture/stories");
    std::fs::create_dir_all(&story_dir).unwrap();
    std::fs::create_dir_all(repo.join("docs/en/architecture/workflows")).unwrap();
    // The story's id encodes workflow 09-09 but it declares wf-01-01: US-WF-001.
    let story = STORY.replace(
        "triples: []",
        "triples:\n  - predicate: arqix:properties/is-part-of-workflow\n    object: arqix:workflows/wf-01-01",
    );
    std::fs::write(story_dir.join("US-09-09-09-sample-story.md"), &story).unwrap();
    let req_dir = repo.join("docs/en/architecture/req");
    std::fs::create_dir_all(&req_dir).unwrap();
    std::fs::write(
        req_dir.join("REQ-09-09-09-01-sample.md"),
        "---\nid: REQ-09-09-09-01\ntitle: Sample\nslug: sample\niri: arqix:requirements/req-09-09-09-01\n\nrdf:\n  type:\n    - arqix:classes/functional-requirement\n\ntriples:\n  - predicate: arqix:properties/derived-from\n    object: arqix:user-stories/us-09-09-09\n\nmeta:\n  owner: hcf\n  created: 2026-07-13\n  updated: 2026-07-13\n  lang: en\n  lifecycle-status: active\n  generated: false\n---\n\n## Requirement\n\nWhen `arqix lint requirements` runs, arqix SHALL honour the configured contract.\n",
    )
    .unwrap();

    // Unconfigured: every module is effective, the contradiction is a finding.
    let out = run_arqix_in(&repo, &["lint", "requirements"]);
    assert_findings(&out);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("US-WF-001"),
        "expected the coupling finding without configuration: {stdout}"
    );

    // Configured without story-driven: the coupling rules do not run.
    std::fs::write(
        repo.join("arqix.toml"),
        "[process]\nmodules = [\"knowledge-base\"]\n",
    )
    .unwrap();
    let out = run_arqix_in(&repo, &["lint", "requirements"]);
    common::assert_success(&out);

    // Configured with story-driven: the coupling rules run unchanged.
    std::fs::write(
        repo.join("arqix.toml"),
        "[process]\nmodules = [\"story-driven\"]\n",
    )
    .unwrap();
    let out = run_arqix_in(&repo, &["lint", "requirements"]);
    assert_findings(&out);
}
