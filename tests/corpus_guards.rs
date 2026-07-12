//! Corpus guards: constraint and quality requirements whose subject is the
//! repository's own documents.
//!
//! Each test reads the real repository files via `CARGO_MANIFEST_DIR`, like
//! `release_documents_stay_consistent_with_the_crate_version` in `tests/cli.rs`.

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
#[test]
fn architecture_views_use_c4_oriented_mermaid_diagrams() {
    assert!(
        root()
            .join("docs/en/architecture/model/workspace.dsl")
            .is_file(),
        "the C4 model source docs/en/architecture/model/workspace.dsl must exist"
    );
    let has_c4_mermaid_block = dir_entries("docs/en/architecture/arc42/units")
        .iter()
        .map(|p| std::fs::read_to_string(p).unwrap())
        .any(|text| {
            text.split("```mermaid").skip(1).any(|after_open| {
                after_open
                    .split("```")
                    .next()
                    .is_some_and(|block| block.contains("C4"))
            })
        });
    assert!(
        has_c4_mermaid_block,
        "at least one arc42 unit must embed a fenced mermaid block using a C4 macro"
    );
}

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
