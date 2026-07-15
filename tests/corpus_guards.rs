//! Corpus guards: constraint and quality requirements whose subject is the
//! repository's own documents.
//!
//! Each test reads the real repository files via `CARGO_MANIFEST_DIR`, like
//! `release_documents_stay_consistent_with_the_crate_version` in `tests/cli.rs`.
//! It also holds the config-wiring guards for the PDF render defaults
//! (`docs/pandoc`), asserting the repository's own Pandoc configuration stays
//! wired together.

use std::path::{Path, PathBuf};

fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

fn read(rel: &str) -> String {
    std::fs::read_to_string(root().join(rel))
        .unwrap_or_else(|e| panic!("expected repository file {rel}: {e}"))
}

fn dir_entries(rel: &str) -> Vec<PathBuf> {
    let dir = root().join(rel);
    let mut entries: Vec<PathBuf> = std::fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("expected repository directory {rel}: {e}"))
        .map(|entry| entry.unwrap().path())
        .collect();
    entries.sort();
    entries
}

/// True when `text` contains `word` as a whole word (no alphanumeric or
/// underscore neighbours), matched case-sensitively.
fn contains_word(text: &str, word: &str) -> bool {
    let is_word_char = |c: char| c.is_ascii_alphanumeric() || c == '_';
    text.match_indices(word).any(|(start, _)| {
        let before_ok = !text[..start].chars().next_back().is_some_and(is_word_char);
        let after_ok = !text[start + word.len()..]
            .chars()
            .next()
            .is_some_and(is_word_char);
        before_ok && after_ok
    })
}

// arqix:verifies REQ-01-01-09-01
#[test]
fn agent_instructions_define_story_by_story_scope_rules() {
    let agents = read("AGENTS.md");
    assert!(
        agents.contains("one story at a time"),
        "AGENTS.md must state the one-story-at-a-time scope rule"
    );
    assert!(
        agents.contains("opportunistic refactors"),
        "AGENTS.md must rule out opportunistic refactors during story execution"
    );
    assert!(
        agents.contains("avoid unrelated cleanup"),
        "AGENTS.md must rule out unrelated cleanup while editing"
    );
}

// arqix:verifies REQ-01-01-09-02
#[test]
fn agent_instructions_define_plan_editing_constraints_and_the_verification_loop() {
    let agents = read("AGENTS.md");
    assert!(
        agents.contains("do not rewrite the reviewed `PLANS.md`"),
        "AGENTS.md must constrain how agents edit the plan document"
    );
    assert!(
        agents.contains("record progress in `STATUS.md`"),
        "AGENTS.md must say where agents record implementation progress"
    );
    assert!(
        agents.contains("scripts/arqix verify"),
        "AGENTS.md must name the required arqix verification loop"
    );
}

// arqix:verifies REQ-01-01-09-06
#[test]
fn agent_extension_points_carry_no_normative_process_rules() {
    for rel in ["CLAUDE.md", ".claude/skills/arqix/SKILL.md"] {
        let text = read(rel);
        assert!(
            text.contains("AGENTS.md"),
            "{rel} must point back at AGENTS.md for process rules"
        );
        for keyword in ["SHALL", "MUST"] {
            assert!(
                !contains_word(&text, keyword),
                "{rel} is an agent-specific extension point and must not carry \
                 the normative keyword {keyword}"
            );
        }
    }
}

// arqix:verifies REQ-01-01-11-01
#[test]
fn arc42_documentation_is_structured_into_assemblable_units_per_chapter() {
    let unit_names: Vec<String> = dir_entries("docs/en/architecture/arc42/units")
        .iter()
        .map(|p| p.file_name().unwrap().to_string_lossy().into_owned())
        .collect();
    let page = read("docs/en/architecture/arc42/page-arc42-arqix-architecture.md");
    for chapter in 1..=12 {
        let prefix = format!("unit-arc42-{chapter:02}-");
        assert!(
            unit_names.iter().any(|name| name.starts_with(&prefix)),
            "arc42 chapter {chapter} must exist as a unit ({prefix}*.md)"
        );
        let include = format!("arqix:include units/{prefix}");
        assert!(
            page.contains(&include),
            "the arc42 page must carry an include directive for chapter {chapter}"
        );
    }
}

// arqix:verifies REQ-01-01-11-02
#[test]
fn adrs_follow_the_path_model_in_the_canonical_governance_language() {
    let adrs: Vec<PathBuf> = dir_entries("docs/en/architecture/adr")
        .into_iter()
        .filter(|p| p.file_name().unwrap().to_string_lossy().starts_with("ADR-"))
        .collect();
    assert!(
        !adrs.is_empty(),
        "the ADR directory must contain ADR documents"
    );
    for path in adrs {
        let name = path.file_name().unwrap().to_string_lossy().into_owned();
        // Filename shape: ADR-\d{4}-[a-z0-9-]+\.md
        let rest = name
            .strip_prefix("ADR-")
            .and_then(|r| r.strip_suffix(".md"))
            .unwrap_or_else(|| panic!("{name} must match ADR-NNNN-<slug>.md"));
        let (digits, slug) = rest.split_at(4);
        assert!(
            digits.chars().all(|c| c.is_ascii_digit()),
            "{name} must carry a four-digit ADR number"
        );
        let slug = slug
            .strip_prefix('-')
            .unwrap_or_else(|| panic!("{name} must separate number and slug with a dash"));
        assert!(
            !slug.is_empty()
                && slug
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-'),
            "{name} must carry a lowercase slug ([a-z0-9-]+)"
        );
        let text = std::fs::read_to_string(&path).unwrap();
        assert!(
            text.lines().any(|l| l.trim() == "lang: en"),
            "{name} must be authored in the canonical governance language (lang: en)"
        );
        assert!(
            text.lines()
                .any(|l| l.trim_start().starts_with("decision-status:")),
            "{name} must carry a decision-status in its frontmatter"
        );
    }
}

// arqix:verifies REQ-01-01-11-04
// arqix:verifies REQ-04-01-18-01
#[test]
fn architecture_views_are_generated_from_the_c4_model() {
    assert!(
        root()
            .join("docs/en/architecture/model/workspace.dsl")
            .is_file(),
        "the C4 model source docs/en/architecture/model/workspace.dsl must exist"
    );
    // The views are rendered from the model (ADR-0016): an arc42 unit embeds a
    // generated SVG via a real Markdown image, the image carries the
    // derived-from provenance marker, and the referenced file exists on disk
    // and is non-empty — a bare provenance comment or a dangling path is not a
    // rendered view.
    let mut embeds = 0;
    for unit in dir_entries("docs/en/architecture/arc42/units") {
        let text = std::fs::read_to_string(&unit).unwrap();
        if !text.contains("derived from") {
            continue;
        }
        for line in text.lines() {
            // A Markdown image `![...](<path>.svg)` targeting the generated dir.
            let Some(open) = line.find("](") else {
                continue;
            };
            if !line[..open].contains("![") {
                continue;
            }
            let rest = &line[open + 2..];
            let Some(close) = rest.find(')') else {
                continue;
            };
            let target = &rest[..close];
            if !(target.contains("model/generated/") && target.ends_with(".svg")) {
                continue;
            }
            // Resolve the relative embed against the embedding unit's directory.
            let resolved = unit.parent().unwrap().join(target);
            let meta = std::fs::metadata(&resolved).unwrap_or_else(|e| {
                panic!(
                    "embedded generated view {} must exist on disk: {e}",
                    resolved.display()
                )
            });
            assert!(
                meta.len() > 0,
                "embedded generated view {} must be non-empty",
                resolved.display()
            );
            embeds += 1;
        }
    }
    assert!(
        embeds > 0,
        "at least one arc42 unit must embed a generated SVG view \
         (![...](...model/generated/*.svg)) carrying derived-from provenance"
    );
}

// The blocking CI freshness gate for the generated views (regenerate and diff
// against the committed SVGs) is planned, not yet enforced: the diagrams
// workflow is manual-dispatch and non-blocking while the render is shaken out
// (roadmap items 5 and 8, ADR-0016). Recorded as a planned claim so the
// requirement is traceable rather than silently unreferenced.
// arqix:plans REQ-04-01-18-02

// arqix:verifies REQ-01-01-11-05
#[test]
fn architecture_documentation_records_the_consistency_check_extension_path() {
    let chapter_11 =
        read("docs/en/architecture/arc42/units/unit-arc42-11-risks-and-technical-debt.md");
    assert!(
        chapter_11.contains("documentation consistency check"),
        "arc42 chapter 11 must record the future documentation consistency check"
    );
    assert!(
        chapter_11.contains("extension path"),
        "arc42 chapter 11 must record the consistency check as an extension path"
    );
}

// arqix:verifies REQ-01-01-15-02
#[test]
fn release_process_documents_semver_and_the_versioned_contracts() {
    let releasing = read("RELEASING.md");
    assert!(
        releasing.contains("SemVer"),
        "RELEASING.md must document the SemVer policy for the product version"
    );
    assert!(
        releasing.contains("`schema_version`"),
        "RELEASING.md must document the per-interface schema_version contract"
    );
    assert!(
        releasing.contains("`config_version`"),
        "RELEASING.md must document the separate configuration schema version"
    );
}

// arqix:verifies REQ-01-01-15-03
#[test]
fn agent_instructions_restrict_agents_to_release_preparation() {
    let agents = read("AGENTS.md");
    assert!(
        agents.contains("tag or publish a release"),
        "AGENTS.md must forbid agent-initiated release tagging and publishing"
    );
    assert!(
        agents.contains("without explicit approval from the repository owner"),
        "AGENTS.md must reserve releases for explicit owner approval"
    );
    let releasing = read("RELEASING.md");
    assert!(
        releasing.contains("Agents prepare releases; a human releases"),
        "RELEASING.md must keep the automation boundary explicit"
    );
}

// arqix:verifies REQ-01-01-15-04
#[test]
fn breaking_releases_require_migration_notes_and_changelog_entries() {
    let changelog = read("CHANGELOG.md");
    assert!(
        changelog.contains("Breaking changes carry a **Migration** note in their entry."),
        "the CHANGELOG header must document the Migration-note rule"
    );
    let releasing = read("RELEASING.md");
    assert!(
        releasing.contains("requires a **Migration** note"),
        "RELEASING.md must require migration notes for breaking changes"
    );
    assert!(
        releasing.contains("CHANGELOG.md"),
        "RELEASING.md must tie migration notes to changelog entries"
    );
}

// arqix:no-requirement
#[test]
fn pandoc_defaults_wire_the_checkbox_glyph_header() {
    let defaults = read("docs/pandoc/defaults.yaml");
    assert!(
        defaults.contains("include-in-header:") && defaults.contains("docs/pandoc/header.tex"),
        "defaults.yaml must wire in the checkbox-glyph header: {defaults}"
    );
    let header = read("docs/pandoc/header.tex");
    assert!(
        header.contains("IfFontExistsTF"),
        "header.tex maps the ballot glyphs behind a font-existence guard: {header}"
    );
    assert!(
        header.contains("\\fbox") && header.contains("catcode"),
        "header.tex keeps a base-LaTeX no-tofu fallback and remaps the ballot code points: {header}"
    );
}

// arqix:no-requirement
#[test]
fn pandoc_defaults_wire_the_c4_png_filter() {
    let defaults = read("docs/pandoc/defaults.yaml");
    assert!(
        defaults.contains("filters:") && defaults.contains("docs/pandoc/svg-to-png.lua"),
        "defaults.yaml must wire in the SVG->PNG filter: {defaults}"
    );
    let filter = read("docs/pandoc/svg-to-png.lua");
    assert!(
        filter.contains("model/generated") && filter.contains("png"),
        "the filter rewrites the C4 view embeds to their PNG variant"
    );
    let render = read("scripts/render_views.sh");
    assert!(
        render.contains("structurizr/$fmt") && render.contains("\"svg\" \"png\""),
        "render_views.sh renders both the svg and png variant: {render}"
    );
}
