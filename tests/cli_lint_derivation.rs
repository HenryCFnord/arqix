//! Command contract: the derivation check inside `lint run` (arc42 chapter 5,
//! US-04-01-18, ADR-0016). A committed C4 Mermaid view marked
//! `<!-- derived from … (view: X) -->` must be structurally derivable from the
//! referenced `workspace.dsl`: every element maps to a model element of
//! matching kind (by display name), every relationship to a direct or
//! Structurizr-implied model edge.

mod common;

use common::{run_arqix_in, stdout_json};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Output;

const DSL: &str = r#"workspace "t" "d" {
    model {
        agent = person "Coding Agent" "loops"
        sys = softwareSystem "The System" "core" {
            cli = container "CLI Binary" "all" "Rust"
        }
        agent -> sys "uses"
    }
    views {
        systemContext sys "SystemContext" {
            include *
        }
    }
}
"#;

fn doc_with(diagram: &str) -> String {
    format!(
        "---\nid: unit-x\ntitle: X\nslug: x\niri: arqix:units/unit-x\n---\n\n## X\n\n<!-- derived from model/workspace.dsl (view: SystemContext) -->\n```mermaid\n{diagram}```\n"
    )
}

/// A scratch corpus: `docs/unit-x.md` carrying the diagram, and the model it
/// claims to derive from at `docs/model/workspace.dsl`.
fn setup(name: &str, diagram: &str) -> PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    if dir.exists() {
        fs::remove_dir_all(&dir).expect("clear scratch dir");
    }
    fs::create_dir_all(dir.join("docs/model")).expect("create dirs");
    fs::write(dir.join("docs/model/workspace.dsl"), DSL).expect("write dsl");
    fs::write(dir.join("docs/unit-x.md"), doc_with(diagram)).expect("write doc");
    dir
}

fn codes(out: &Output) -> Vec<String> {
    stdout_json(out)["diagnostics"]
        .as_array()
        .map(|a| {
            a.iter()
                .filter_map(|d| d["code"].as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default()
}

// arqix:verifies REQ-04-01-18-01
#[test]
fn lint_flags_a_diagram_element_absent_from_the_model() {
    let dir = setup(
        "deriv_bad_element",
        "C4Context\n    System(x, \"Ghost System\", \"y\")\n",
    );
    let out = run_arqix_in(&dir, &["lint", "run", "--format", "json"]);
    let codes = codes(&out);
    assert!(
        codes.iter().any(|c| c == "LNT-DRV-001"),
        "an element absent from the model is flagged: {codes:?}"
    );
}

// arqix:verifies REQ-04-01-18-02
#[test]
fn lint_flags_a_relationship_absent_from_the_model() {
    // `sys -> agent` has no model edge — only `agent -> sys` exists.
    let dir = setup(
        "deriv_bad_rel",
        "C4Context\n    Person(a, \"Coding Agent\", \"loops\")\n    System(s, \"The System\", \"core\")\n    Rel(s, a, \"bogus\")\n",
    );
    let out = run_arqix_in(&dir, &["lint", "run", "--format", "json"]);
    let codes = codes(&out);
    assert!(
        codes.iter().any(|c| c == "LNT-DRV-002"),
        "a relationship the model does not justify is flagged: {codes:?}"
    );
}

// arqix:verifies REQ-04-01-18-01
#[test]
fn lint_passes_a_faithful_diagram() {
    let dir = setup(
        "deriv_good",
        "C4Context\n    Person(a, \"Coding Agent\", \"loops\")\n    System(s, \"The System\", \"core\")\n    Rel(a, s, \"uses\")\n",
    );
    let out = run_arqix_in(&dir, &["lint", "run", "--format", "json"]);
    let codes = codes(&out);
    assert!(
        !codes.iter().any(|c| c.starts_with("LNT-DRV")),
        "a faithful diagram raises no derivation finding: {codes:?}"
    );
}
