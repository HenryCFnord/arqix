//! Verifying evidence for the quality and constraint requirements whose
//! subject is observable CLI behaviour: containment, no-execution,
//! diagnostics quality, log parseability, performance budgets, and the
//! creation aliases. Gap-fill where the behaviour already exists,
//! red-first where it does not (include-root containment, the aliases,
//! the `unit new` help).

mod common;

use common::{assert_success, run_arqix_in, scratch_copy, stdout_json};

/// The scratch tree outside the corpus root, snapshotted as (path, bytes).
fn outside_state(repo: &std::path::Path) -> Vec<(String, Vec<u8>)> {
    let mut state = Vec::new();
    for entry in std::fs::read_dir(repo).unwrap().flatten() {
        let path = entry.path();
        if path.is_file() {
            state.push((
                path.to_string_lossy().to_string(),
                std::fs::read(&path).unwrap(),
            ));
        }
    }
    state.sort();
    state
}

// arqix:verifies REQ-00-00-00-07
#[test]
fn mutating_commands_leave_files_outside_the_roots_untouched() {
    let repo = scratch_copy(
        "minimal",
        "mutating_commands_leave_files_outside_the_roots_untouched",
    );
    std::fs::write(repo.join("canary.txt"), "untouched\n").unwrap();
    std::fs::write(repo.join("code.rs"), "fn library() {}\n").unwrap();
    let before = outside_state(&repo);

    assert_success(&run_arqix_in(&repo, &["fmt"]));
    assert_success(&run_arqix_in(
        &repo,
        &["doc", "new", "requirement", "--title", "Scoped"],
    ));
    assert_success(&run_arqix_in(&repo, &["assemble", "build"]));

    // The corpus root gained files; everything beside it is byte-identical
    // (pages/ is a declared generated-artefact target, not a source).
    let after: Vec<_> = outside_state(&repo)
        .into_iter()
        .filter(|(p, _)| before.iter().any(|(bp, _)| bp == p))
        .collect();
    assert_eq!(before, after, "no file outside the roots may change");
}

// arqix:verifies REQ-00-00-00-08
#[test]
fn creation_never_overwrites_an_existing_document() {
    let repo = scratch_copy("minimal", "creation_never_overwrites_an_existing_document");
    let target = repo.join("docs/REQ-99-99-99-01-fixture-requirement.md");
    let before = std::fs::read_to_string(&target).unwrap();

    let out = run_arqix_in(
        &repo,
        &[
            "doc",
            "new",
            "requirement",
            "--title",
            "Fixture Requirement",
            "--id",
            "REQ-99-99-99-01",
        ],
    );
    assert_eq!(
        out.status.code(),
        Some(1),
        "reusing an existing id is a finding, never an overwrite"
    );
    assert_eq!(
        std::fs::read_to_string(&target).unwrap(),
        before,
        "the existing document stays byte-identical"
    );
}

// arqix:verifies REQ-00-00-00-14
#[test]
fn processed_content_is_never_executed() {
    let repo = scratch_copy("minimal", "processed_content_is_never_executed");
    std::fs::write(
        repo.join("arqix.toml"),
        "[policies.publish]\nsite-command = \"true\"\n",
    )
    .unwrap();
    // Shell-shaped content in a document: substitution, backticks, a
    // script tag. Processing may copy it — it must never evaluate it.
    let payload = "$(touch pwned-subst) `touch pwned-tick`\n\n<script>fetch('x')</script>\n";
    std::fs::write(
        repo.join("docs/hostile.md"),
        format!("---\nid: hostile\ntitle: Hostile\n---\n\n## Hostile\n\n{payload}"),
    )
    .unwrap();

    assert_success(&run_arqix_in(&repo, &["fmt"]));
    assert_success(&run_arqix_in(&repo, &["assemble", "build"]));
    assert_success(&run_arqix_in(&repo, &["publish", "site"]));

    assert!(
        !repo.join("pwned-subst").exists() && !repo.join("pwned-tick").exists(),
        "embedded shell syntax must never run"
    );
    let staged = std::fs::read_to_string(repo.join("site-src/hostile.md")).unwrap();
    assert!(
        staged.contains("$(touch pwned-subst)"),
        "content passes through verbatim, uninterpreted: {staged}"
    );
}

// arqix:verifies REQ-02-01-09-02
#[test]
fn includes_never_resolve_outside_the_configured_roots() {
    let repo = scratch_copy(
        "minimal",
        "includes_never_resolve_outside_the_configured_roots",
    );
    // Inside the repository, outside the configured root: REQ-00-00-00-13's
    // repository containment does not catch this — REQ-02-01-09-02 must.
    std::fs::write(repo.join("outside.md"), "secret outside the corpus\n").unwrap();
    std::fs::write(
        repo.join("docs/escape.md"),
        "---\nid: escape\ntitle: Escape\n---\n\n## Escape\n\n<!-- arqix:include ../outside.md -->\n",
    )
    .unwrap();

    let out = run_arqix_in(&repo, &["assemble", "build"]);
    assert_eq!(
        out.status.code(),
        Some(1),
        "an include escaping the configured roots is refused as a finding"
    );
    let output = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        output.contains("outside.md") || output.contains("escape"),
        "the diagnostic names the offending include: {output}"
    );
    assert!(
        !repo.join("pages/escape.md").exists()
            || !std::fs::read_to_string(repo.join("pages/escape.md"))
                .unwrap()
                .contains("secret"),
        "escaped content never reaches an assembled page"
    );
}

// arqix:verifies REQ-03-01-05-05
#[test]
fn unresolved_references_stay_visible_in_trace_outputs() {
    let repo = scratch_copy(
        "minimal",
        "unresolved_references_stay_visible_in_trace_outputs",
    );
    std::fs::write(
        repo.join("dangling.rs"),
        "// arqix:verifies REQ-77-77-77-77\n#[test]\nfn dangling() {}\n",
    )
    .unwrap();
    let out = run_arqix_in(&repo, &["trace", "scan", "--format", "json"]);
    let graph = stdout_json(&out);
    assert!(
        graph["edges"]
            .as_array()
            .is_some_and(|edges| edges.iter().any(|e| e["to"] == "REQ-77-77-77-77")),
        "a marker to a nonexistent requirement stays a visible edge: {graph}"
    );
}

/// A scratch corpus of one thousand minimal documents.
fn thousand_documents(name: &str) -> std::path::PathBuf {
    let repo = scratch_copy("minimal", name);
    for i in 0..1000 {
        std::fs::write(
            repo.join(format!("docs/doc-{i:04}.md")),
            format!("---\nid: doc-{i:04}\ntitle: Doc {i:04}\n---\n\n## Doc {i:04}\n\nBody {i:04} searchable.\n"),
        )
        .unwrap();
    }
    repo
}

// arqix:verifies REQ-00-00-00-11
#[test]
fn search_answers_within_a_second_on_a_thousand_documents() {
    let repo = thousand_documents("search_answers_within_a_second_on_a_thousand_documents");
    let start = std::time::Instant::now();
    assert_success(&run_arqix_in(&repo, &["doc", "search", "searchable"]));
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs_f64() < 1.0,
        "search took {elapsed:?}, budget is one second (REQ-00-00-00-11)"
    );
}

// arqix:verifies REQ-00-00-00-12
#[test]
fn the_verification_loop_completes_within_ten_seconds_on_a_thousand_documents() {
    let repo = thousand_documents(
        "the_verification_loop_completes_within_ten_seconds_on_a_thousand_documents",
    );
    let start = std::time::Instant::now();
    assert_success(&run_arqix_in(&repo, &["verify"]));
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs_f64() < 10.0,
        "verify took {elapsed:?}, budget is ten seconds (REQ-00-00-00-12)"
    );
}

// arqix:verifies REQ-01-01-02-03
#[test]
fn unit_new_help_explains_location_metadata_and_ids() {
    let repo = scratch_copy(
        "minimal",
        "unit_new_help_explains_location_metadata_and_ids",
    );
    let out = run_arqix_in(&repo, &["unit", "new", "--help"]);
    let help = String::from_utf8_lossy(&out.stdout);
    assert!(
        help.contains("units/"),
        "the help names where units are created: {help}"
    );
    assert!(
        help.to_lowercase().contains("optional"),
        "the help says which metadata is optional: {help}"
    );
    assert!(
        help.to_lowercase().contains("id"),
        "the help explains how IDs are supplied: {help}"
    );
}

// arqix:verifies REQ-01-01-05-02
#[test]
fn creation_aliases_mirror_doc_new() {
    let repo = scratch_copy("minimal", "creation_aliases_mirror_doc_new");
    for (alias, prefix) in [("req", "REQ"), ("us", "US"), ("adr", "ADR")] {
        let out = run_arqix_in(&repo, &[alias, "new", "--title", "Aliased", "--dry-run"]);
        assert_eq!(
            out.status.code(),
            Some(0),
            "`{alias} new` is the alias for template-based creation"
        );
        let plan = String::from_utf8_lossy(&out.stdout);
        assert!(
            plan.contains(prefix),
            "`{alias} new` plans a {prefix} id: {plan}"
        );
    }
}

// arqix:verifies REQ-02-01-03-01
#[test]
fn fmt_keeps_diffs_focused_on_content() {
    let repo = scratch_copy("minimal", "fmt_keeps_diffs_focused_on_content");
    let path = repo.join("docs/REQ-99-99-99-01-fixture-requirement.md");
    // A pure body edit on a canonical document: fmt must change nothing —
    // the diff a reviewer sees is exactly the content edit.
    assert_success(&run_arqix_in(&repo, &["fmt"]));
    let canonical = std::fs::read_to_string(&path).unwrap();
    let edited = canonical.replace(
        "## Fixture Requirement",
        "## Fixture Requirement\n\nNew prose.",
    );
    std::fs::write(&path, &edited).unwrap();
    assert_success(&run_arqix_in(&repo, &["fmt"]));
    assert_eq!(
        std::fs::read_to_string(&path).unwrap(),
        edited,
        "fmt never rewrites beyond the mechanical canon"
    );
}

// arqix:verifies REQ-03-01-02-04
#[test]
fn empty_link_cases_stay_visible_in_the_matrix() {
    let repo = scratch_copy("minimal", "empty_link_cases_stay_visible_in_the_matrix");
    std::fs::write(
        repo.join("docs/REQ-55-55-55-55-uncovered.md"),
        "---\nid: REQ-55-55-55-55\ntitle: Uncovered\niri: arqix:requirements/req-55-55-55-55\nrdf:\n  type:\n    - arqix:classes/functional-requirement\n---\n\n## Requirement\n\nThe system SHALL stay visible while uncovered.\n",
    )
    .unwrap();
    let out = run_arqix_in(&repo, &["trace", "matrix"]);
    let csv = String::from_utf8_lossy(&out.stdout);
    let row = csv
        .lines()
        .find(|l| l.starts_with("REQ-55-55-55-55"))
        .expect("the uncovered requirement keeps its row");
    assert!(
        row.contains(",,"),
        "empty link cells stay reviewer-visible instead of being dropped: {row}"
    );
}

/// Run `assemble build` on a scratch corpus with one include and return
/// the parsed JSONL log records.
fn assembly_log(name: &str) -> (std::path::PathBuf, Vec<serde_json::Value>) {
    let repo = scratch_copy("minimal", name);
    std::fs::write(repo.join("docs/fragment.md"), "fragment text\n").unwrap();
    std::fs::write(
        repo.join("docs/page.md"),
        "---\nid: page-l\ntitle: Logged Page\n---\n\n## Logged Page\n\n<!-- arqix:include fragment.md -->\n",
    )
    .unwrap();
    assert_success(&run_arqix_in(&repo, &["assemble", "build"]));
    let text = std::fs::read_to_string(repo.join("pages/assembly.jsonl")).expect("assembly log");
    let records = text
        .lines()
        .map(|l| serde_json::from_str(l).expect("every log line is JSON"))
        .collect();
    (repo, records)
}

// arqix:verifies REQ-04-01-01-01
#[test]
fn the_assembly_log_is_a_collectable_artefact() {
    // One fixed path, ready to attach as a CI artefact without
    // post-processing: pages/assembly.jsonl next to the assembled pages.
    let (repo, records) = assembly_log("the_assembly_log_is_a_collectable_artefact");
    assert!(repo.join("pages/assembly.jsonl").is_file());
    assert!(!records.is_empty(), "the log carries the run's records");
}

// arqix:verifies REQ-05-01-02-01
#[test]
fn assembly_log_records_carry_stable_field_names() {
    let (_repo, records) = assembly_log("assembly_log_records_carry_stable_field_names");
    for record in &records {
        assert!(
            record.get("include").is_some() && record.get("at_line").is_some(),
            "downstream tooling relies on the fixed field names: {record}"
        );
    }
}

// arqix:verifies REQ-06-01-02-01
// arqix:verifies REQ-08-01-02-01
#[test]
fn assembly_outcomes_are_reviewable_from_log_and_exit_code() {
    let (_repo, records) = assembly_log("assembly_outcomes_are_reviewable_from_log_and_exit_code");
    // The include step is in the log — composition is reviewable without
    // inferring hidden steps; the command result said ok (asserted in the
    // helper), so log + exit code carry the outcome.
    assert!(
        records.iter().any(|r| r["include"]
            .as_str()
            .is_some_and(|i| i.contains("fragment.md"))),
        "every composition step is recorded: {records:?}"
    );
}

// arqix:verifies REQ-08-01-01-01
#[test]
fn failure_diagnostics_name_the_stop_condition() {
    let repo = scratch_copy("minimal", "failure_diagnostics_name_the_stop_condition");
    std::fs::write(
        repo.join("docs/a.md"),
        "---\nid: cyc-a\ntitle: A\n---\n\n## A\n\n<!-- arqix:include b.md -->\n",
    )
    .unwrap();
    std::fs::write(repo.join("docs/b.md"), "<!-- arqix:include a.md -->\n").unwrap();
    let out = run_arqix_in(&repo, &["assemble", "build"]);
    assert_eq!(
        out.status.code(),
        Some(1),
        "a cycle is a finding, inside the stable exit contract"
    );
    let output = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        output.contains("ASM-001") && output.contains("a.md") && output.contains("cycle"),
        "the stop condition is actionable without reading source code: {output}"
    );
}

// arqix:verifies REQ-08-01-01-02
#[test]
fn a_fresh_package_passes_the_verification_loop_directly() {
    let repo = scratch_copy(
        "minimal",
        "a_fresh_package_passes_the_verification_loop_directly",
    );
    let fresh = repo.join("fresh");
    std::fs::create_dir_all(&fresh).unwrap();
    assert_success(&run_arqix_in(&fresh, &["doc", "init"]));
    assert_success(&run_arqix_in(&fresh, &["verify"]));
}

// arqix:verifies REQ-01-01-10-03
#[test]
fn templates_and_validation_share_the_contract_source() {
    let repo = scratch_copy(
        "minimal",
        "templates_and_validation_share_the_contract_source",
    );
    // One declared contract: what doc new produces, fmt --check accepts —
    // no second source to drift from.
    std::fs::write(
        repo.join("arqix.toml"),
        "[kinds.requirement]\ndir = \"docs\"\nkey-order = [\"id\", \"title\", \"slug\", \"iri\", \"rdf\", \"triples\", \"properties\", \"external-references\", \"meta\"]\n",
    )
    .unwrap();
    assert_success(&run_arqix_in(
        &repo,
        &["doc", "new", "requirement", "--title", "Contracted"],
    ));
    assert_success(&run_arqix_in(&repo, &["fmt", "--check"]));
}
