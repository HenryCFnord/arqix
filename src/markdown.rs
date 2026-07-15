//! Neutral Markdown scanning helpers shared across the assembler and the
//! publisher (Phase A slice 2 dedup). Behaviour-preserving: every function here
//! is a verbatim move of logic that previously lived — byte-identical — in
//! `src/assembler.rs` and `src/publisher.rs`. This module owns no policy and no
//! spec; it is the single home for the ATX-heading and include-directive scans
//! both modules depend on. It is deliberately *not* the oracle-mirrored
//! `first_heading` in `checkers/frontmatter.rs`, which stays non-fence-aware.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// The ATX heading level of a line (column-zero `#`s followed by a space), or
/// None for anything else. Promoted verbatim from the identical copies in the
/// assembler and the publisher.
pub(crate) fn heading_level(line: &str) -> Option<i64> {
    let hashes = line.len() - line.trim_start_matches('#').len();
    ((1..=6).contains(&hashes) && line[hashes..].starts_with(' ')).then_some(hashes as i64)
}

/// The ATX headings of `text` that lie outside fenced code, in source order,
/// each as its `(level, trimmed-text)` pair (the text is the line with its
/// leading `#`-run and surrounding whitespace stripped). A ```` ``` ```` line
/// toggles the fence and is never itself a heading. Both the assembler's
/// first-heading probe and the publisher's two-heading staging scan are
/// expressed over this one iterator.
pub(crate) fn headings_outside_fences(text: &str) -> impl Iterator<Item = (i64, &str)> + '_ {
    let mut in_fence = false;
    text.lines().filter_map(move |line| {
        if line.trim_start().starts_with("```") {
            in_fence = !in_fence;
            return None;
        }
        if in_fence {
            return None;
        }
        heading_level(line).map(|level| (level, line.trim_start_matches('#').trim()))
    })
}

/// The canonical paths every `arqix:include` directive in `files` points at.
/// Each file is read (unreadable files are skipped), scanned fence-aware, and
/// every directive target is resolved against the file's own directory and
/// canonicalised; targets that fail to canonicalise are skipped. Shared by the
/// publisher's corpus-wide `included_targets` and its per-document
/// `drop_included_fragments`.
pub(crate) fn included_target_set(files: &[PathBuf]) -> HashSet<PathBuf> {
    let mut targets = HashSet::new();
    for file in files {
        let Ok(text) = std::fs::read_to_string(file) else {
            continue;
        };
        let dir = file.parent().unwrap_or_else(|| Path::new("."));
        let mut in_fence = false;
        for line in text.lines() {
            if line.trim_start().starts_with("```") {
                in_fence = !in_fence;
                continue;
            }
            if in_fence {
                continue;
            }
            if let Some((target, _)) = crate::linter::include_directive(line)
                && let Ok(canon) = std::fs::canonicalize(dir.join(&target))
            {
                targets.insert(canon);
            }
        }
    }
    targets
}

#[cfg(test)]
mod tests {
    use super::{heading_level, headings_outside_fences, included_target_set};
    use std::collections::HashSet;
    use std::path::PathBuf;

    // arqix:no-requirement
    #[test]
    fn heading_level_accepts_one_through_six_hashes_with_a_space() {
        assert_eq!(heading_level("# H"), Some(1));
        assert_eq!(heading_level("## H"), Some(2));
        assert_eq!(heading_level("### H"), Some(3));
        assert_eq!(heading_level("#### H"), Some(4));
        assert_eq!(heading_level("##### H"), Some(5));
        assert_eq!(heading_level("###### H"), Some(6));
    }

    // arqix:no-requirement
    #[test]
    fn heading_level_rejects_non_headings() {
        // Seven hashes overflow the range.
        assert_eq!(heading_level("####### H"), None);
        // A `#`-run with no following space is not a heading.
        assert_eq!(heading_level("#H"), None);
        assert_eq!(heading_level("###no-space"), None);
        // Not column-zero, empty, and plain prose.
        assert_eq!(heading_level(" # indented"), None);
        assert_eq!(heading_level(""), None);
        assert_eq!(heading_level("plain text"), None);
    }

    // arqix:no-requirement
    #[test]
    fn headings_outside_fences_yields_levels_and_trimmed_text_in_order() {
        let text = "# Title\n\nprose\n\n## Section\n\n### Deeper\n";
        let got: Vec<(i64, &str)> = headings_outside_fences(text).collect();
        assert_eq!(
            got,
            vec![(1, "Title"), (2, "Section"), (3, "Deeper")],
            "every heading, in source order, as (level, trimmed text)"
        );
    }

    // arqix:no-requirement
    #[test]
    fn headings_outside_fences_ignores_headings_inside_fenced_code() {
        // The `## Not a heading` and the trailing `# Also fenced` sit inside
        // fences and must be skipped; only the two real headings survive.
        let text = "# Real\n\n```\n## Not a heading\n```\n\n## Also Real\n\n```sh\n# Also fenced\n```\n";
        let got: Vec<(i64, &str)> = headings_outside_fences(text).collect();
        assert_eq!(got, vec![(1, "Real"), (2, "Also Real")]);
    }

    // arqix:no-requirement
    #[test]
    fn included_target_set_collects_canonical_directive_targets() {
        let dir = std::env::temp_dir().join(format!(
            "arqix-md-{}-included",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let fragment = dir.join("fragment.md");
        let other = dir.join("other.md");
        std::fs::write(&fragment, "# Fragment\n").unwrap();
        std::fs::write(&other, "# Other\n").unwrap();
        // A parent that includes `fragment.md` outside a fence, plus a fenced
        // directive that must NOT count as an include.
        let parent = dir.join("parent.md");
        std::fs::write(
            &parent,
            "# Parent\n\n<!-- arqix:include fragment.md -->\n\n```\n<!-- arqix:include other.md -->\n```\n",
        )
        .unwrap();

        let got = included_target_set(std::slice::from_ref(&parent));
        let mut want: HashSet<PathBuf> = HashSet::new();
        want.insert(std::fs::canonicalize(&fragment).unwrap());
        assert_eq!(got, want, "only the unfenced include target is collected");
    }
}
