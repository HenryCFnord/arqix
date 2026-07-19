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

// arqix:verifies REQ-08-01-29-01
#[test]
fn lint_frontmatter_resolves_the_configured_section_kinds() {
    // Vocabulary binding is configuration (ADR-0017): FM-007 gates against
    // [frontmatter].section-kinds when the corpus configures it.
    let repo = scratch_copy(
        "minimal",
        "lint_frontmatter_resolves_the_configured_section_kinds",
    );
    std::fs::write(
        repo.join("arqix.toml"),
        "[frontmatter]\nsection-kinds = [\"term-page\"]\n",
    )
    .unwrap();
    std::fs::create_dir_all(repo.join("docs/ontology")).unwrap();
    let dir = repo.join("docs/en/architecture/stories");
    std::fs::create_dir_all(&dir).unwrap();
    let doc = STORY
        .replace("lang: de", "lang: en")
        .replace("properties: {}", "properties:\n  section-kind: term-page");
    std::fs::write(dir.join("US-09-09-09-sample-story.md"), &doc).unwrap();

    // The configured kind passes.
    let out = run_arqix_in(&repo, &["lint", "frontmatter"]);
    common::assert_success(&out);

    // A built-in kind outside the configured vocabulary is now a finding.
    std::fs::write(
        dir.join("US-09-09-09-sample-story.md"),
        doc.replace("section-kind: term-page", "section-kind: arc42-chapter"),
    )
    .unwrap();
    let out = run_arqix_in(&repo, &["lint", "frontmatter"]);
    assert_findings(&out);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("FM-007") && stdout.contains("arc42-chapter"),
        "expected FM-007 against the unlisted kind: {stdout}"
    );
}

// arqix:verifies REQ-08-01-29-02
#[test]
fn lint_frontmatter_resolves_the_configured_external_types() {
    // Vocabulary binding is configuration (ADR-0017): ONT-002 accepts a
    // non-arqix rdf.type only from [frontmatter].allowed-external-types.
    let repo = scratch_copy(
        "minimal",
        "lint_frontmatter_resolves_the_configured_external_types",
    );
    std::fs::write(
        repo.join("arqix.toml"),
        "[frontmatter]\nallowed-external-types = [\"skos:Concept\"]\n",
    )
    .unwrap();
    std::fs::create_dir_all(repo.join("docs/ontology")).unwrap();
    let dir = repo.join("docs/en/architecture/stories");
    std::fs::create_dir_all(&dir).unwrap();
    let doc = STORY
        .replace("lang: de", "lang: en")
        .replace("rdfs:Class", "skos:Concept");
    std::fs::write(dir.join("US-09-09-09-sample-story.md"), &doc).unwrap();

    // The configured external type passes.
    let out = run_arqix_in(&repo, &["lint", "frontmatter"]);
    common::assert_success(&out);

    // A built-in type outside the configured vocabulary is now a finding.
    std::fs::write(
        dir.join("US-09-09-09-sample-story.md"),
        doc.replace("skos:Concept", "rdfs:Class"),
    )
    .unwrap();
    let out = run_arqix_in(&repo, &["lint", "frontmatter"]);
    assert_findings(&out);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("ONT-002") && stdout.contains("rdfs:Class"),
        "expected ONT-002 against the unlisted type: {stdout}"
    );
}

// arqix:verifies REQ-08-01-30-01
#[test]
fn lint_frontmatter_reports_dangling_triple_objects() {
    // The frontmatter graph resolves like the body markers (LNT-003): an
    // arqix-namespace triple object no document carries is ONT-003. The rule
    // predates this test; the test pins it against the FR-A1 reproduction.
    let repo = scratch_copy(
        "minimal",
        "lint_frontmatter_reports_dangling_triple_objects",
    );
    let prop_dir = repo.join("docs/ontology/properties");
    std::fs::create_dir_all(&prop_dir).unwrap();
    std::fs::write(
        prop_dir.join("points-at.md"),
        "---\nid: property-points-at\nlabel: points-at\niri: arqix:properties/points-at\n\nrdf:\n  type:\n    - rdf:Property\n\ntriples: []\n\nproperties: {}\n\nexternal-references: []\n\nmeta:\n  lifecycle-status: draft\n  owner: hcf\n  created: 2026-07-19\n  updated: 2026-07-19\n  lang: en\n  generated: false\n---\n\n## points-at\n\nA selftest property.\n",
    )
    .unwrap();
    let dir = repo.join("docs/en/architecture/stories");
    std::fs::create_dir_all(&dir).unwrap();
    let doc = STORY.replace("lang: de", "lang: en").replace(
        "triples: []",
        "triples:\n  - predicate: arqix:properties/points-at\n    object: arqix:user-stories/us-99-99-99",
    );
    std::fs::write(dir.join("US-09-09-09-sample-story.md"), &doc).unwrap();

    // The dangling arqix-namespace object is a finding.
    let out = run_arqix_in(&repo, &["lint", "frontmatter"]);
    assert_findings(&out);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("ONT-003") && stdout.contains("arqix:user-stories/us-99-99-99"),
        "expected ONT-003 naming the dangling object: {stdout}"
    );

    // A non-arqix object stays an opaque external reference.
    std::fs::write(
        dir.join("US-09-09-09-sample-story.md"),
        doc.replace("arqix:user-stories/us-99-99-99", "skos:Concept"),
    )
    .unwrap();
    let out = run_arqix_in(&repo, &["lint", "frontmatter"]);
    common::assert_success(&out);
}

// arqix:verifies REQ-08-01-34-01
#[test]
fn lint_frontmatter_verifies_the_local_copy_digest() {
    // FR-A2: a well-formed local-copy/sha256 pair is checked against the
    // file's actual bytes — a stale digest or a missing copy is SRC-006.
    let repo = scratch_copy("minimal", "lint_frontmatter_verifies_the_local_copy_digest");
    let class_dir = repo.join("docs/ontology/classes");
    std::fs::create_dir_all(&class_dir).unwrap();
    std::fs::write(
        class_dir.join("source.md"),
        "---\nid: class-source\nlabel: source\niri: arqix:classes/source\n\nrdf:\n  type:\n    - rdfs:Class\n\nrdfs:\n  sub-class-of:\n    - arqix:classes/source\n\ntriples: []\n\nproperties: {}\n\nexternal-references: []\n\nowl: {}\n\nmeta:\n  lifecycle-status: draft\n  owner: hcf\n  created: 2026-07-19\n  updated: 2026-07-19\n  lang: en\n  generated: false\n---\n\n## Source\n\nA selftest fixture class.\n",
    )
    .unwrap();
    std::fs::write(
        repo.join("arqix.toml"),
        "[kinds.source]\ndir = \"docs/en/sources\"\n",
    )
    .unwrap();
    let src_dir = repo.join("docs/en/sources");
    std::fs::create_dir_all(&src_dir).unwrap();
    std::fs::write(repo.join("sources-sample.txt"), "hello\n").unwrap();
    // sha256 of "hello\n".
    let good = "5891b5b522d5df086d0ff0b110fbd9d21bb4fc7163af34d08286a2e846f6be03";
    let record = format!(
        "---\nid: SRC-0001\ntitle: Sample Source\nslug: sample-source\niri: arqix:sources/src-0001\n\nrdf:\n  type:\n    - arqix:classes/source\n\ntriples: []\n\nproperties:\n  uri: https://example.org/sample\n  accessed: 2026-07-19\n  local-copy: sources-sample.txt\n  sha256: {good}\n\nexternal-references: []\n\nmeta:\n  lifecycle-status: final\n  owner: hcf\n  created: 2026-07-19\n  updated: 2026-07-19\n  lang: en\n  generated: false\n---\n\n## Sample Source\n\nA fixture record.\n"
    );
    std::fs::write(src_dir.join("SRC-0001.md"), &record).unwrap();

    // The matching digest passes.
    let out = run_arqix_in(&repo, &["lint", "frontmatter"]);
    common::assert_success(&out);

    // A flipped digest is SRC-006 naming the path.
    let flipped = record.replace(good, &format!("{}f", &good[..63]));
    std::fs::write(src_dir.join("SRC-0001.md"), &flipped).unwrap();
    let out = run_arqix_in(&repo, &["lint", "frontmatter"]);
    assert_findings(&out);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("SRC-006") && stdout.contains("sources-sample.txt"),
        "expected SRC-006 for the digest mismatch: {stdout}"
    );

    // A missing copy is SRC-006 too.
    std::fs::write(src_dir.join("SRC-0001.md"), &record).unwrap();
    std::fs::remove_file(repo.join("sources-sample.txt")).unwrap();
    let out = run_arqix_in(&repo, &["lint", "frontmatter"]);
    assert_findings(&out);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("SRC-006"),
        "expected SRC-006 for the missing copy: {stdout}"
    );
}

// arqix:verifies REQ-08-01-35-01
#[test]
fn lint_frontmatter_validates_declared_property_vocabularies() {
    // FR-C1: the kind declares controlled vocabularies for named properties
    // fields — the domain-state axis next to the guarded lifecycle.
    let repo = scratch_copy(
        "minimal",
        "lint_frontmatter_validates_declared_property_vocabularies",
    );
    std::fs::create_dir_all(repo.join("docs/ontology")).unwrap();
    std::fs::write(
        repo.join("arqix.toml"),
        "[kinds.term]\ndir = \"docs/terms\"\n\n[kinds.term.vocab]\nextraction-status = [\"extracted\", \"proposed\", \"decided\"]\n",
    )
    .unwrap();
    let dir = repo.join("docs/terms");
    std::fs::create_dir_all(&dir).unwrap();
    let doc = STORY.replace("lang: de", "lang: en").replace(
        "properties: {}",
        "properties:\n  extraction-status: proposed",
    );
    std::fs::write(dir.join("sample.md"), &doc).unwrap();

    // A declared value passes.
    let out = run_arqix_in(&repo, &["lint", "frontmatter"]);
    common::assert_success(&out);

    // A value outside the vocabulary is FM-009 naming field and value.
    std::fs::write(
        dir.join("sample.md"),
        doc.replace("extraction-status: proposed", "extraction-status: bogus"),
    )
    .unwrap();
    let out = run_arqix_in(&repo, &["lint", "frontmatter"]);
    assert_findings(&out);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("FM-009")
            && stdout.contains("extraction-status")
            && stdout.contains("bogus"),
        "expected FM-009 naming field and value: {stdout}"
    );
}

const THING_CLASS: &str = "---\nid: class-thing\nlabel: thing\niri: arqix:classes/thing\n\nrdf:\n  type:\n    - rdfs:Class\n\nrdfs:\n  sub-class-of:\n    - arqix:classes/thing\n\ntriples: []\n\nproperties: {}\n\nexternal-references: []\n\nowl: {}\n\nmeta:\n  lifecycle-status: draft\n  owner: hcf\n  created: 2026-07-19\n  updated: 2026-07-19\n  lang: en\n  generated: false\n---\n\n## Thing\n\nA fixture root class.\n";

// arqix:verifies REQ-08-01-36-01
#[test]
fn lint_frontmatter_checks_edges_against_domain_and_range() {
    // ONT-007: declaring rdfs.domain/range opts the property into the
    // contract; subject and resolvable object types must lie inside the
    // declared classes, subclass closure included.
    let repo = scratch_copy(
        "minimal",
        "lint_frontmatter_checks_edges_against_domain_and_range",
    );
    let classes = repo.join("docs/ontology/classes");
    let props = repo.join("docs/ontology/properties");
    std::fs::create_dir_all(&classes).unwrap();
    std::fs::create_dir_all(&props).unwrap();
    std::fs::write(classes.join("thing.md"), THING_CLASS).unwrap();
    std::fs::write(
        classes.join("item.md"),
        THING_CLASS
            .replace("class-thing", "class-item")
            .replace("label: thing", "label: item")
            .replace("iri: arqix:classes/thing", "iri: arqix:classes/item")
            .replace("## Thing", "## Item"),
    )
    .unwrap();
    std::fs::write(
        classes.join("other.md"),
        THING_CLASS
            .replace("class-thing", "class-other")
            .replace("label: thing", "label: other")
            .replace("arqix:classes/thing", "arqix:classes/other")
            .replace("## Thing", "## Other"),
    )
    .unwrap();
    let prop = "---\nid: property-points-at\nlabel: points-at\niri: arqix:properties/points-at\n\nrdf:\n  type:\n    - rdf:Property\n\nrdfs:\n  domain:\n    - arqix:classes/thing\n  range:\n    - arqix:classes/thing\n\ntriples: []\n\nproperties: {}\n\nexternal-references: []\n\nmeta:\n  lifecycle-status: draft\n  owner: hcf\n  created: 2026-07-19\n  updated: 2026-07-19\n  lang: en\n  generated: false\n---\n\n## points-at\n\nA fixture property.\n";
    std::fs::write(props.join("points-at.md"), prop).unwrap();
    let dir = repo.join("docs/en/architecture/stories");
    std::fs::create_dir_all(&dir).unwrap();
    let doc = STORY
        .replace("lang: de", "lang: en")
        .replace("rdfs:Class", "arqix:classes/item")
        .replace(
            "triples: []",
            "triples:\n  - predicate: arqix:properties/points-at\n    object: arqix:user-stories/us-09-09-09",
        );
    std::fs::write(dir.join("US-09-09-09-sample-story.md"), &doc).unwrap();

    // item is a subclass of thing: subject and object satisfy the contract.
    let out = run_arqix_in(&repo, &["lint", "frontmatter"]);
    common::assert_success(&out);

    // A domain outside the subject's closure is ONT-007.
    std::fs::write(
        props.join("points-at.md"),
        prop.replace(
            "domain:\n    - arqix:classes/thing",
            "domain:\n    - arqix:classes/other",
        ),
    )
    .unwrap();
    let out = run_arqix_in(&repo, &["lint", "frontmatter"]);
    assert_findings(&out);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("ONT-007") && stdout.contains("points-at"),
        "expected ONT-007 for the domain violation: {stdout}"
    );
}

// arqix:verifies REQ-08-01-36-02
#[test]
fn lint_frontmatter_reports_subclass_cycles() {
    // ONT-008: a sub-class-of cycle longer than the root self-reference.
    let repo = scratch_copy("minimal", "lint_frontmatter_reports_subclass_cycles");
    let classes = repo.join("docs/ontology/classes");
    std::fs::create_dir_all(&classes).unwrap();
    std::fs::write(
        classes.join("a.md"),
        THING_CLASS
            .replace("class-thing", "class-a")
            .replace("label: thing", "label: a")
            .replace("iri: arqix:classes/thing", "iri: arqix:classes/a")
            .replace(
                "sub-class-of:\n    - arqix:classes/thing",
                "sub-class-of:\n    - arqix:classes/b",
            )
            .replace("## Thing", "## A"),
    )
    .unwrap();
    std::fs::write(
        classes.join("b.md"),
        THING_CLASS
            .replace("class-thing", "class-b")
            .replace("label: thing", "label: b")
            .replace("iri: arqix:classes/thing", "iri: arqix:classes/b")
            .replace(
                "sub-class-of:\n    - arqix:classes/thing",
                "sub-class-of:\n    - arqix:classes/a",
            )
            .replace("## Thing", "## B"),
    )
    .unwrap();

    let out = run_arqix_in(&repo, &["lint", "frontmatter"]);
    assert_findings(&out);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("ONT-008"),
        "expected ONT-008 for the a<->b cycle: {stdout}"
    );
}

// arqix:verifies REQ-08-01-38-01
#[test]
fn lint_frontmatter_checks_paths_against_the_dir_template() {
    // FM-010: the checker-side direction of the placement contract — the
    // parent directory must equal the dir-template rendered from the
    // document's own properties.
    let repo = scratch_copy(
        "minimal",
        "lint_frontmatter_checks_paths_against_the_dir_template",
    );
    std::fs::create_dir_all(repo.join("docs/ontology")).unwrap();
    std::fs::write(
        repo.join("arqix.toml"),
        "[kinds.term]\ndir = \"contexts\"\ndir-template = \"contexts/{context}/terms\"\n",
    )
    .unwrap();
    let right = repo.join("contexts/tmforum/terms");
    let wrong = repo.join("contexts/itu/terms");
    std::fs::create_dir_all(&right).unwrap();
    std::fs::create_dir_all(&wrong).unwrap();
    let doc = STORY
        .replace("lang: de", "lang: en")
        .replace("properties: {}", "properties:\n  context: tmforum");
    std::fs::write(right.join("sample.md"), &doc).unwrap();

    // Path and context agree: no finding.
    let out = run_arqix_in(&repo, &["lint", "frontmatter"]);
    common::assert_success(&out);

    // The same document under another context is FM-010.
    std::fs::rename(right.join("sample.md"), wrong.join("sample.md")).unwrap();
    let out = run_arqix_in(&repo, &["lint", "frontmatter"]);
    assert_findings(&out);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("FM-010") && stdout.contains("contexts/tmforum/terms"),
        "expected FM-010 naming the rendered directory: {stdout}"
    );

    // A document missing the property the template names is FM-010 too.
    std::fs::write(
        wrong.join("sample.md"),
        doc.replace("properties:\n  context: tmforum", "properties: {}"),
    )
    .unwrap();
    let out = run_arqix_in(&repo, &["lint", "frontmatter"]);
    assert_findings(&out);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("FM-010") && stdout.contains("context"),
        "expected FM-010 naming the unresolved placeholder: {stdout}"
    );
}
