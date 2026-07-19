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
    // The option must parse before and after any subcommand (exit code 2
    // would mean a usage error). The command surface has no stubs left, so
    // the probe is `config show` — cheap, read-only, deterministic.
    let output = run_arqix(&["--format", "json", "config", "show"]);
    assert_eq!(output.status.code(), Some(0));

    let after_subcommand = run_arqix(&["config", "show", "--format", "json"]);
    assert_eq!(after_subcommand.status.code(), Some(0));

    let invalid_value = run_arqix(&["--format", "yaml", "config", "show"]);
    assert_eq!(invalid_value.status.code(), Some(2));
}

// arqix:verifies REQ-01-01-15-01
#[test]
fn release_documents_stay_consistent_with_the_crate_version() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let cargo = std::fs::read_to_string(root.join("Cargo.toml")).unwrap();
    let version = cargo
        .lines()
        .find_map(|l| l.strip_prefix("version = \""))
        .and_then(|rest| rest.strip_suffix('"'))
        .expect("Cargo.toml carries a version");

    let changelog = std::fs::read_to_string(root.join("CHANGELOG.md"))
        .expect("CHANGELOG.md exists (REQ-01-01-15-01)");
    assert!(
        changelog.contains(&format!("## [{version}]")),
        "the changelog's top release section must match the crate version {version}"
    );

    let releasing = std::fs::read_to_string(root.join("RELEASING.md"))
        .expect("RELEASING.md exists (REQ-01-01-15-01)");
    assert!(
        releasing.contains("CHANGELOG.md"),
        "the release process must reference the changelog it maintains"
    );
}

// arqix:verifies REQ-04-01-10-03
#[test]
fn findings_surfaces_speak_the_shared_diagnostics_contract() {
    // The last command-specific findings shapes converge: every findings
    // surface answers --format json with the one diagnostics payload.
    let repo = common::scratch_copy(
        "minimal",
        "findings_surfaces_speak_the_shared_diagnostics_contract",
    );
    std::fs::create_dir_all(repo.join("docs/ontology")).unwrap();
    std::fs::create_dir_all(repo.join("docs/en/architecture/stories")).unwrap();
    let req_dir = repo.join("docs/en/architecture/req");
    std::fs::create_dir_all(&req_dir).unwrap();
    // One violation per surface: a forbidden keyword for lint requirements,
    // a frontmatter contract breach for lint frontmatter, and a marker-less
    // test function for trace markers.
    std::fs::write(
        req_dir.join("REQ-09-09-09-01-sample.md"),
        "---\nid: REQ-09-09-09-01\ntitle: Sample\nslug: sample\niri: arqix:requirements/req-09-09-09-01\n\nrdf:\n  type:\n    - arqix:classes/functional-requirement\n\ntriples: []\n\nproperties: {}\n\nexternal-references: []\n\nmeta:\n  lifecycle-status: active\n  owner: hcf\n  created: 2026-07-13\n  updated: 2026-07-13\n  lang: en\n  generated: false\n---\n\n## Requirement\n\nThe tool MUST reject unknown flags.\n",
    )
    .unwrap();
    std::fs::create_dir_all(repo.join("tests")).unwrap();
    std::fs::write(
        repo.join("tests/sample.rs"),
        "#[test]\nfn needs_a_marker() {}\n",
    )
    .unwrap();

    for command in [
        vec!["lint", "frontmatter"],
        vec!["lint", "requirements"],
        vec!["trace", "markers"],
    ] {
        let mut args = command.clone();
        args.extend(["--format", "json"]);
        let out = common::run_arqix_in(&repo, &args);
        let report = common::stdout_json(&out);
        assert_eq!(
            report["schema_version"], 1,
            "{command:?} must version its payload: {report}"
        );
        let diagnostics = report["diagnostics"]
            .as_array()
            .unwrap_or_else(|| panic!("{command:?} must carry a diagnostics array: {report}"));
        assert!(
            diagnostics.iter().all(|d| d["severity"].is_string()
                && d["code"].is_string()
                && d["message"].is_string()),
            "{command:?} entries carry severity, code, message: {report}"
        );
        assert!(
            report.get("findings").is_none() && report.get("summary").is_none(),
            "{command:?} must not keep a private findings shape: {report}"
        );
    }
}
