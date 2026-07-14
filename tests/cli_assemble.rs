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

// arqix:verifies REQ-02-01-11-01
#[test]
fn assemble_build_fails_on_output_collisions_across_roots() {
    let repo = scratch_copy(
        "minimal",
        "assemble_build_fails_on_output_collisions_across_roots",
    );
    // Two roots with the same root-relative file name map to the same
    // pages/ output; the second page must not silently overwrite the first.
    std::fs::write(repo.join("arqix.toml"), "roots = [\"docs\", \"extra\"]\n").unwrap();
    std::fs::create_dir_all(repo.join("extra")).unwrap();
    std::fs::write(
        repo.join("extra/REQ-99-99-99-01-fixture-requirement.md"),
        "---\nid: DOC-88\ntitle: Clash\n---\nother body\n",
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
        combined.contains("collision") || combined.contains("collide"),
        "the diagnostic must name the output collision: {combined}"
    );
}

// arqix:verifies REQ-00-00-00-13
#[test]
fn assemble_build_refuses_includes_outside_the_repository() {
    let repo = scratch_copy(
        "minimal",
        "assemble_build_refuses_includes_outside_the_repository",
    );
    // A file OUTSIDE the repository (the scratch dir's parent).
    let outside = repo
        .parent()
        .unwrap()
        .join("assemble-containment-secret.md");
    std::fs::write(&outside, "SECRET CONTENT\n").unwrap();
    std::fs::write(
        repo.join("docs/esc.md"),
        "<!-- arqix:include ../../assemble-containment-secret.md -->\n",
    )
    .unwrap();

    let out = run_arqix_in(&repo, &["assemble", "build"]);
    common::assert_findings(&out);
    let page = repo.join("pages/esc.md");
    if page.exists() {
        let content = std::fs::read_to_string(&page).unwrap();
        assert!(
            !content.contains("SECRET CONTENT"),
            "content outside the repository must never be inlined"
        );
    }
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

// arqix:verifies REQ-02-01-12-05
#[test]
fn assemble_omits_included_fragment_frontmatter() {
    let repo = scratch_copy("minimal", "assemble_omits_included_fragment_frontmatter");
    std::fs::write(
        repo.join("docs/fragment.md"),
        "---\nid: frag-unit\ntitle: Fragment Unit\nkind: unit\n---\n\n## Fragment Body\n\nProse.\n",
    )
    .unwrap();
    std::fs::write(
        repo.join("docs/page.md"),
        "---\nid: stitch-fm\ntitle: Page\n---\n\n## Page\n\n<!-- arqix:include fragment.md level=+1 -->\n",
    )
    .unwrap();
    assert_success(&run_arqix_in(&repo, &["assemble", "build"]));
    let page = std::fs::read_to_string(repo.join("pages/page.md")).unwrap();
    assert!(
        page.contains("Fragment Body"),
        "the fragment's content is stitched in: {page}"
    );
    assert!(
        !page.contains("id: frag-unit"),
        "the fragment's frontmatter must not appear in the assembled page: {page}"
    );
    assert!(
        !page.contains("kind: unit"),
        "no fragment frontmatter key may leak into the body: {page}"
    );
}

// arqix:verifies REQ-02-01-12-01
#[test]
fn assemble_shifts_included_headings_to_the_declared_level() {
    let repo = scratch_copy(
        "minimal",
        "assemble_shifts_included_headings_to_the_declared_level",
    );
    std::fs::write(
        repo.join("docs/fragment.md"),
        "## Fragment Title\n\nText.\n\n### Fragment Detail\n\nMore.\n",
    )
    .unwrap();
    std::fs::write(
        repo.join("docs/page.md"),
        "---\nid: stitch-abs\ntitle: Page\n---\n\n## Page\n\n<!-- arqix:include fragment.md level=3 -->\n",
    )
    .unwrap();
    assert_success(&run_arqix_in(&repo, &["assemble", "build"]));
    let page = std::fs::read_to_string(repo.join("pages/page.md")).unwrap();
    assert!(
        page.contains("\n### Fragment Title\n"),
        "the fragment's first heading lands at the declared level: {page}"
    );
    assert!(
        page.contains("\n#### Fragment Detail\n"),
        "every heading shifts by the same delta — internal structure preserved: {page}"
    );
}

// arqix:verifies REQ-02-01-12-02
#[test]
fn assemble_resolves_relative_levels_at_the_include_position() {
    let repo = scratch_copy(
        "minimal",
        "assemble_resolves_relative_levels_at_the_include_position",
    );
    std::fs::write(repo.join("docs/fragment.md"), "## Reused Unit\n\nText.\n").unwrap();
    std::fs::write(
        repo.join("docs/page.md"),
        "---\nid: stitch-rel\ntitle: Page\n---\n\n## Shallow\n\n<!-- arqix:include fragment.md level=+1 -->\n\n### Deeper\n\n<!-- arqix:include fragment.md level=+1 -->\n",
    )
    .unwrap();
    assert_success(&run_arqix_in(&repo, &["assemble", "build"]));
    let page = std::fs::read_to_string(repo.join("pages/page.md")).unwrap();
    assert!(
        page.contains("\n### Reused Unit\n"),
        "+1 under an h2 yields h3: {page}"
    );
    assert!(
        page.contains("\n#### Reused Unit\n"),
        "the same fragment under an h3 yields h4 — moving an include re-levels without editing: {page}"
    );
}

// arqix:verifies REQ-02-01-12-03
#[test]
fn assemble_fails_on_heading_overflow() {
    let repo = scratch_copy("minimal", "assemble_fails_on_heading_overflow");
    std::fs::write(
        repo.join("docs/fragment.md"),
        "## Top\n\n### Too Deep For Six\n",
    )
    .unwrap();
    std::fs::write(
        repo.join("docs/page.md"),
        "---\nid: stitch-overflow\ntitle: Page\n---\n\n## Page\n\n<!-- arqix:include fragment.md level=6 -->\n",
    )
    .unwrap();
    let out = run_arqix_in(&repo, &["assemble", "build"]);
    assert_eq!(
        out.status.code(),
        Some(1),
        "a shift beyond h6 is a structural finding, never a silent clamp"
    );
    let stderr = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stderr),
        String::from_utf8_lossy(&out.stdout)
    );
    assert!(
        stderr.contains("ASM-005") && stderr.contains("Too Deep For Six"),
        "the diagnostic names the heading: {stderr}"
    );
    assert!(
        stderr.contains("fragment.md"),
        "the diagnostic names the fragment: {stderr}"
    );
}

// arqix:verifies REQ-02-01-12-04
#[test]
fn assemble_applies_the_configured_heading_ownership_default() {
    let repo = scratch_copy(
        "minimal",
        "assemble_applies_the_configured_heading_ownership_default",
    );
    std::fs::write(repo.join("docs/fragment.md"), "## Owned Title\n\nText.\n").unwrap();
    std::fs::write(
        repo.join("docs/page.md"),
        "---\nid: stitch-own\ntitle: Page\n---\n\n## Page Section\n\n<!-- arqix:include fragment.md -->\n",
    )
    .unwrap();

    // The child default: fragments own their headings, a bare include
    // behaves as level=+1 under the page's section.
    assert_success(&run_arqix_in(&repo, &["assemble", "build"]));
    let page = std::fs::read_to_string(repo.join("pages/page.md")).unwrap();
    assert!(
        page.contains("\n### Owned Title\n"),
        "child ownership: a bare include behaves as level=+1: {page}"
    );

    // Parent ownership: the page declares the outline; a bare include
    // inlines the fragment verbatim.
    std::fs::write(
        repo.join("arqix.toml"),
        "[policies.assemble]\nheading-ownership = \"parent\"\n",
    )
    .unwrap();
    assert_success(&run_arqix_in(&repo, &["assemble", "build"]));
    let page = std::fs::read_to_string(repo.join("pages/page.md")).unwrap();
    assert!(
        page.contains("\n## Owned Title\n"),
        "parent ownership: a bare include stays verbatim: {page}"
    );
}

// arqix:verifies REQ-04-01-03-02
#[test]
fn assemble_rebases_relative_links_from_included_fragments() {
    // Artefact-ready pages: a fragment's relative links must resolve from
    // the assembled page's location, not from where the fragment lives.
    let repo = scratch_copy(
        "minimal",
        "assemble_rebases_relative_links_from_included_fragments",
    );
    std::fs::create_dir_all(repo.join("docs/units")).unwrap();
    std::fs::create_dir_all(repo.join("docs/adr")).unwrap();
    std::fs::write(repo.join("docs/adr/ADR-X.md"), "## Decision X\n").unwrap();
    std::fs::write(
        repo.join("docs/units/unit-a.md"),
        "## Unit A\n\nSee [the decision](../adr/ADR-X.md) and [its section](../adr/ADR-X.md#decision-x).\n",
    )
    .unwrap();
    std::fs::write(
        repo.join("docs/page.md"),
        "---\nid: stitch-links\ntitle: Page\n---\n\n## Page\n\n<!-- arqix:include units/unit-a.md level=+1 -->\n",
    )
    .unwrap();
    assert_success(&run_arqix_in(&repo, &["assemble", "build"]));
    let page = std::fs::read_to_string(repo.join("pages/page.md")).unwrap();
    assert!(
        page.contains("(adr/ADR-X.md)"),
        "the link is rebased to the including page's directory: {page}"
    );
    assert!(
        page.contains("(adr/ADR-X.md#decision-x)"),
        "anchors survive the rebase: {page}"
    );
    assert!(
        !page.contains("../adr/"),
        "no fragment-relative link survives assembly: {page}"
    );
}
