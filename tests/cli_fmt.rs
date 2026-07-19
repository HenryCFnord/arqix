//! Command contract: `fmt` — owned by the Formatter & Finaliser, the only
//! mutator of existing source documents (arc42 chapter 5, ADR-0004).

mod common;

use common::{assert_success, run_arqix_in, scratch_copy};
use std::fs;

// arqix:verifies REQ-01-01-03-01
#[test]
fn fmt_sorts_frontmatter_keys_canonically() {
    let repo = scratch_copy("minimal", "fmt_sorts_frontmatter_keys_canonically");
    let out = run_arqix_in(&repo, &["fmt"]);
    assert_success(&out);
}

// arqix:verifies REQ-01-01-03-02
// arqix:verifies REQ-01-01-03-03
#[test]
fn fmt_never_changes_body_text() {
    let repo = scratch_copy("minimal", "fmt_never_changes_body_text");
    let doc = repo.join("docs/REQ-99-99-99-01-fixture-requirement.md");
    let before = fs::read_to_string(&doc).unwrap();
    let body_before = before.split("---").nth(2).unwrap().to_string();

    assert_success(&run_arqix_in(&repo, &["fmt"]));

    let after = fs::read_to_string(&doc).unwrap();
    let body_after = after.split("---").nth(2).unwrap();
    assert_eq!(body_before.trim(), body_after.trim());
}

// arqix:verifies REQ-01-01-03-01
#[test]
fn fmt_orders_ontology_frontmatter_by_family() {
    let repo = scratch_copy("minimal", "fmt_orders_ontology_frontmatter_by_family");
    // The ontology property family orders `owl` between `rdfs` and
    // `triples` (check_frontmatter.py FAMILIES) — a single global key list
    // cannot express that, so fmt must pick the canonical order by family.
    fs::create_dir_all(repo.join("docs/ontology/properties")).unwrap();
    let doc = repo.join("docs/ontology/properties/documented-by.md");
    fs::write(
        &doc,
        "---\nid: documented-by\nlabel: documented-by\niri: arqix:properties/documented-by\nrdf:\n  type:\n    - rdf:Property\nrdfs:\n  domain:\n    - arqix:classes/artefact\ntriples: []\nproperties: {}\nexternal-references: []\nowl:\n  inverse-of: arqix:properties/documents-artefact\nmeta:\n  updated: 2026-01-01\n---\nbody\n",
    )
    .unwrap();

    assert_success(&run_arqix_in(&repo, &["fmt"]));

    let after = fs::read_to_string(&doc).unwrap();
    let owl = after.find("\nowl:").expect("owl key kept");
    let triples = after.find("\ntriples:").expect("triples key kept");
    assert!(
        owl < triples,
        "ont-property order puts owl before triples: {after}"
    );
    // And the fix is stable: a second run is a no-op.
    assert_success(&run_arqix_in(&repo, &["fmt"]));
    assert_eq!(after, fs::read_to_string(&doc).unwrap());
}

// arqix:verifies REQ-00-00-00-01
#[test]
fn fmt_is_idempotent() {
    let repo = scratch_copy("minimal", "fmt_is_idempotent");
    let doc = repo.join("docs/REQ-99-99-99-01-fixture-requirement.md");

    assert_success(&run_arqix_in(&repo, &["fmt"]));
    let first = fs::read_to_string(&doc).unwrap();
    assert_success(&run_arqix_in(&repo, &["fmt"]));
    let second = fs::read_to_string(&doc).unwrap();

    assert_eq!(first, second, "a second fmt run must be a no-op");
}

// arqix:verifies REQ-01-01-19-01
#[test]
fn fmt_orders_keys_from_the_configured_contract() {
    let repo = scratch_copy("minimal", "fmt_orders_keys_from_the_configured_contract");
    std::fs::write(
        repo.join("arqix.toml"),
        "[kinds.note]\ndir = \"docs/notes\"\nkey-order = [\"title\", \"id\", \"meta\"]\n",
    )
    .unwrap();
    std::fs::create_dir_all(repo.join("docs/notes")).unwrap();
    std::fs::write(
        repo.join("docs/notes/n.md"),
        "---\nid: note-1\nmeta:\n  lang: en\ntitle: A Note\n---\n\nBody.\n",
    )
    .unwrap();
    assert_success(&run_arqix_in(&repo, &["fmt"]));
    let formatted = std::fs::read_to_string(repo.join("docs/notes/n.md")).unwrap();
    let title = formatted.find("title:").unwrap();
    let id = formatted.find("id:").unwrap();
    let meta = formatted.find("meta:").unwrap();
    assert!(
        title < id && id < meta,
        "the configured key order governs the family's frontmatter: {formatted}"
    );
}

// arqix:verifies REQ-01-01-19-02
#[test]
fn fmt_and_config_show_share_one_contract_source() {
    // The one-source rule (ADR-0011): what fmt applies is exactly what the
    // effective configuration exposes — no second copy of the contract.
    let repo = scratch_copy("minimal", "fmt_and_config_show_share_one_contract_source");
    std::fs::write(
        repo.join("arqix.toml"),
        "[kinds.note]\ndir = \"docs/notes\"\nkey-order = [\"title\", \"id\"]\nrequired-meta = [\"lang\"]\n",
    )
    .unwrap();
    let out = run_arqix_in(&repo, &["config", "show", "--format", "json"]);
    assert_success(&out);
    let config = common::stdout_json(&out);
    assert_eq!(
        config["kinds"]["note"]["key-order"][0], "title",
        "the effective configuration exposes the contract fmt applies: {config}"
    );
    assert_eq!(
        config["kinds"]["note"]["required-meta"][0], "lang",
        "the required meta keys come from the same source: {config}"
    );
}

// arqix:verifies REQ-01-01-19-03
#[test]
fn scaffolded_documents_satisfy_the_default_meta_contract() {
    // The template is the first document a contract sees: every default
    // required-meta key must be present in a fresh scaffold.
    let repo = scratch_copy(
        "minimal",
        "scaffolded_documents_satisfy_the_default_meta_contract",
    );
    assert_success(&run_arqix_in(&repo, &["doc", "new", "adr"]));
    let created = std::fs::read_to_string(repo.join("docs/adr/ADR-0001.md")).unwrap();
    for key in [
        "lifecycle-status",
        "owner",
        "created",
        "updated",
        "lang",
        "generated",
    ] {
        assert!(
            created.contains(&format!("{key}:")),
            "required meta key '{key}' missing from the scaffold: {created}"
        );
    }
}

// arqix:verifies REQ-08-01-40-02
#[test]
fn fmt_lifts_claim_markers_into_derived_triples() {
    // ADR-0018: the derived-triples section is formatter-owned — derived
    // from the claim markers, deduplicated, sorted, absent without markers.
    let repo = scratch_copy("minimal", "fmt_lifts_claim_markers_into_derived_triples");
    let dir = repo.join("docs/en/architecture/stories");
    std::fs::create_dir_all(&dir).unwrap();
    let doc = "---\nid: US-09-09-09\ntitle: Sample Story\nslug: sample-story\niri: arqix:user-stories/us-09-09-09\n\nrdf:\n  type:\n    - rdfs:Class\n\ntriples: []\n\nproperties: {}\n\nexternal-references: []\n\nmeta:\n  lifecycle-status: draft\n  owner: hcf\n  created: 2026-07-13\n  updated: 2026-07-13\n  lang: en\n  generated: false\n---\n\n## Sample Story\n\n<!-- arqix:claim supported-by=arqix:sources/src-0002 -->\nA supported sentence.\n\n<!-- arqix:claim supported-by=arqix:sources/src-0001 confidence=high -->\nAnother supported sentence.\n\n<!-- arqix:claim supported-by=arqix:sources/src-0001 anchor=\"2.1\" -->\nThe same source again.\n";
    let path = dir.join("US-09-09-09-sample-story.md");
    std::fs::write(&path, doc).unwrap();

    // The lifting: deduplicated, sorted targets in the derived section.
    assert_success(&run_arqix_in(&repo, &["fmt"]));
    let text = std::fs::read_to_string(&path).unwrap();
    let src1 = text
        .find("arqix:sources/src-0001")
        .expect("src-0001 lifted");
    let src2 = text
        .find("arqix:sources/src-0002")
        .expect("src-0002 lifted");
    assert!(
        text.contains("derived-triples:")
            && text.contains("arqix:properties/supported-by")
            && src1 < src2,
        "expected the sorted derived section: {text}"
    );
    assert_eq!(
        text.matches("arqix:sources/src-0001").count(),
        2,
        "one derived entry plus the body marker, no duplicate: {text}"
    );

    // Idempotent: a second run changes nothing.
    let before = std::fs::read_to_string(&path).unwrap();
    assert_success(&run_arqix_in(&repo, &["fmt"]));
    assert_eq!(before, std::fs::read_to_string(&path).unwrap());

    // Drift: a hand edit fails check mode and does not survive fmt.
    std::fs::write(
        &path,
        before.replace("arqix:sources/src-0002", "arqix:sources/src-0009"),
    )
    .unwrap();
    let out = run_arqix_in(&repo, &["fmt", "--check"]);
    assert!(
        out.status.code() != Some(0),
        "expected check mode to report the drift"
    );
    assert_success(&run_arqix_in(&repo, &["fmt"]));
    let healed = std::fs::read_to_string(&path).unwrap();
    assert!(
        healed.contains("arqix:sources/src-0002"),
        "the section is derived from the markers again: {healed}"
    );

    // No markers, no section.
    let stripped: String = healed
        .lines()
        .filter(|l| !l.starts_with("<!-- arqix:claim"))
        .collect::<Vec<_>>()
        .join("\n")
        + "\n";
    std::fs::write(&path, stripped).unwrap();
    assert_success(&run_arqix_in(&repo, &["fmt"]));
    let text = std::fs::read_to_string(&path).unwrap();
    assert!(
        !text.contains("derived-triples:"),
        "the section vanishes with the markers: {text}"
    );
}
