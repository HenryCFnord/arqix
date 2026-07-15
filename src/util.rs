//! Neutral, oracle-free helpers shared across the non-checker modules:
//! POSIX path normalization and the sorted markdown directory walk.
//!
//! This module deliberately lives outside `parser.rs` and `checkers/`, which
//! are faithful ports of the Python oracle and held under the oracle-fidelity
//! freeze. The helpers here are used by the assembler, store, reporter,
//! publisher, linter, rewriter, and trace engine — none of them oracle-mirrored
//! — so they can share one copy without pulling neutral code under the freeze.

use std::path::{Path, PathBuf};

/// Normalise a path to forward slashes so a `file` field is identical across
/// platforms and consistent with the trace engine, store, and assembler.
///
/// On Unix `\` is an ordinary filename byte, so this only rewrites the
/// Windows separator; the output matches the Python oracle's posix paths.
pub(crate) fn to_posix(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

/// The string-input variant for call sites that already hold a path string
/// (the `file` field a document carries). Identical result to [`to_posix`]:
/// for a valid-UTF-8 path string, `Path::new(s).to_string_lossy()` is `s`.
pub(crate) fn to_posix_str(path: &str) -> String {
    path.replace('\\', "/")
}

/// Collect the markdown files under `dir`, sorted and depth-first, into
/// `files`. Reproduces Python `sorted(dir.rglob('*.md'))` byte for byte:
/// each directory level is sorted, directory symlinks are never followed
/// (a parent link would make the walk unbounded, and the trace oracle's
/// rglob does not follow them either), the configured `skip` directories are
/// pruned by name, and only `*.md` that is not `*.tpl.md` is kept.
///
/// The traversal is shared by the store (which then parses each file) and the
/// publisher (which stages the raw paths). The corpus roots / member lists
/// stay in each caller; only this inner walk is shared (ADR-0011 boundary).
pub(crate) fn collect_markdown(dir: &Path, skip: &[String], files: &mut Vec<PathBuf>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };
    let mut paths: Vec<_> = entries.flatten().map(|e| e.path()).collect();
    paths.sort();
    for path in paths {
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if path.is_dir() {
            if !path.symlink_metadata().is_ok_and(|m| m.is_symlink())
                && !skip.iter().any(|s| s == name)
            {
                collect_markdown(&path, skip, files);
            }
        } else if name.ends_with(".md") && !name.ends_with(".tpl.md") {
            files.push(path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // arqix:no-requirement
    #[test]
    fn to_posix_rewrites_backslashes() {
        assert_eq!(to_posix(Path::new("a\\b\\c.md")), "a/b/c.md");
        assert_eq!(to_posix(Path::new("already/posix.md")), "already/posix.md");
    }

    // arqix:no-requirement
    #[test]
    fn to_posix_str_matches_the_path_variant() {
        assert_eq!(to_posix_str("a\\b\\c.md"), "a/b/c.md");
        assert_eq!(
            to_posix_str("a\\b\\c.md"),
            to_posix(Path::new("a\\b\\c.md"))
        );
    }
}
