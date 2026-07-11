//! Publish & Render Orchestrator, site half: `publish site` stages
//! artefact-ready inputs and orchestrates the configured site toolchain —
//! it never renders HTML itself, exactly as `render pdf` orchestrates
//! Pandoc instead of rendering PDF (owner decision 2026-07-11). Staging is
//! per language root (REQ-04-01-07-01): includes expanded, directives gone,
//! the arqix frontmatter reduced to the toolchain-consumable part
//! (REQ-04-01-03-02). Toolchain failures are system errors, exit 2, never
//! findings (REQ-04-01-07-02). Like the assembler, it only writes generated
//! artefacts — sources are never mutated (ADR-0004).

use crate::OutputFormat;
use crate::diag::SCHEMA_VERSION;
use serde_json::json;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

// arqix:implements REQ-04-01-03-01
// arqix:implements REQ-04-01-03-02
// arqix:implements REQ-04-01-03-03
// arqix:implements REQ-04-01-07-01
// arqix:implements REQ-04-01-07-02
/// `arqix publish site [--lang <lang>]`
pub fn site(lang: Option<&str>, format: OutputFormat) -> ExitCode {
    let policy = crate::config::publish_policy(Path::new("."));
    // v1 stitches single-page: the assembled document is the unit of
    // publication. The split-on-outline mode is decided (stitching
    // discussion 2026-07-11) and lands with the ADR-0013 assembler slice.
    if policy.stitching != "single-page" {
        eprintln!(
            "error: [policies.publish] stitching '{}' is not available yet: v1 stitches single-page (split lands with the ADR-0013 slice)",
            policy.stitching
        );
        return ExitCode::from(2);
    }
    // No built-in renderer and no fallback: rendering is the toolchain's
    // job, and publishing without one is a config error — exactly like
    // `render pdf` without Pandoc.
    let Some(command) = &policy.site_command else {
        eprintln!(
            "error: no site toolchain configured: set [policies.publish] site-command (e.g. a pinned Zensical or MkDocs invocation; see docs/en/processes/configuration.md)"
        );
        return ExitCode::from(2);
    };

    let default_lang = crate::config::default_lang(Path::new("."));
    let lang = lang.unwrap_or(&default_lang);
    // The language's staging target: the default language owns the staging
    // root, every other language its own subdirectory (REQ-04-01-07-01).
    let staging = if lang == default_lang {
        PathBuf::from(&policy.staging_dir)
    } else {
        Path::new(&policy.staging_dir).join(lang)
    };

    let mut staged = 0usize;
    let skip = crate::config::skip_dirs(Path::new("."));
    for root in crate::config::roots(Path::new(".")) {
        // The language's root: <root>/<lang> where the layout has one; the
        // bare root serves the default language otherwise.
        let lang_root = Path::new(&root).join(lang);
        let lang_root = if lang_root.is_dir() {
            lang_root
        } else if lang == default_lang {
            PathBuf::from(&root)
        } else {
            continue;
        };

        let mut files = Vec::new();
        collect_markdown(&lang_root, &skip, &mut files);
        for file in files {
            let assembled = match crate::assembler::expand_document(&file) {
                Ok(text) => text,
                Err(diagnostic) => {
                    eprintln!(
                        "error: {}: {}",
                        diagnostic.file.as_deref().unwrap_or("?"),
                        diagnostic.message
                    );
                    return ExitCode::from(2);
                }
            };
            let doc = crate::parser::parse(&file.to_string_lossy(), &assembled);
            let rel = file.strip_prefix(&lang_root).unwrap_or(&file).to_path_buf();
            if let Err(code) = write_staged(&staging.join(&rel), &doc) {
                return code;
            }
            staged += 1;
        }
    }

    // Orchestrate the toolchain over the staged inputs, inheriting stdio so
    // its own output and errors surface transparently.
    let mut parts = command.split_whitespace();
    let Some(program) = parts.next() else {
        eprintln!("error: [policies.publish] site-command is empty");
        return ExitCode::from(2);
    };
    match Command::new(program).args(parts).status() {
        Ok(status) if status.success() => {}
        Ok(status) => {
            eprintln!(
                "error: site toolchain failed: '{command}' exited with {}",
                status
                    .code()
                    .map_or("signal".to_string(), |c| c.to_string())
            );
            return ExitCode::from(2);
        }
        Err(err) => {
            eprintln!("error: site toolchain failed: cannot run '{command}': {err}");
            return ExitCode::from(2);
        }
    }

    match format {
        OutputFormat::Json => {
            let result = json!({
                "schema_version": SCHEMA_VERSION,
                "lang": lang,
                "staging_dir": staging.to_string_lossy(),
                "staged": staged,
                "toolchain": command,
            });
            println!(
                "{}",
                serde_json::to_string_pretty(&result).expect("valid JSON")
            );
        }
        OutputFormat::Text => println!(
            "staged {staged} page(s) to {}; toolchain '{command}' ok",
            staging.display()
        ),
    }
    ExitCode::SUCCESS
}

/// Collect Markdown sources under `dir`, path-sorted, honouring the skip
/// set; templates (`.tpl.md`) are never published.
fn collect_markdown(dir: &Path, skip: &[String], files: &mut Vec<PathBuf>) {
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

/// Write one staged page: the assembled body under a minimal frontmatter
/// carrying only what site toolchains consume (the title). Dropping the
/// arqix identity keys also keeps staged copies out of document discovery
/// and the trace graph — no duplicate ids from generated artefacts.
fn write_staged(path: &Path, doc: &crate::parser::Document) -> Result<(), ExitCode> {
    let mut out = String::new();
    if let Some(title) = &doc.title {
        out.push_str(&format!("---\ntitle: {title}\n---\n"));
    }
    for line in doc.body.lines() {
        let trimmed = line.trim();
        // Directives and marker comments never reach the toolchain.
        if trimmed.starts_with("<!--") && trimmed.ends_with("-->") && trimmed.contains("arqix:") {
            continue;
        }
        out.push_str(line);
        out.push('\n');
    }

    if let Some(parent) = path.parent()
        && let Err(err) = std::fs::create_dir_all(parent)
    {
        eprintln!("error: cannot create {}: {err}", parent.display());
        return Err(ExitCode::from(2));
    }
    if let Err(err) = std::fs::write(path, out) {
        eprintln!("error: cannot write {}: {err}", path.display());
        return Err(ExitCode::from(2));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    // arqix:no-requirement
    #[test]
    fn staged_pages_carry_only_the_consumable_frontmatter() {
        let doc = crate::parser::parse(
            "docs/x.md",
            "---\nid: X-01\ntitle: A Title\niri: arqix:x/x-01\n---\n\n## A Title\n\nBody.\n<!-- arqix:references-artefact arqix:x/y -->\n",
        );
        let dir = std::env::temp_dir().join("arqix-staged-frontmatter-test");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("x.md");
        super::write_staged(&path, &doc).unwrap();
        let staged = std::fs::read_to_string(&path).unwrap();
        assert!(staged.contains("title: A Title"));
        assert!(!staged.contains("iri:") && !staged.contains("id: X-01"));
        assert!(!staged.contains("arqix:references-artefact"));
        assert!(staged.contains("Body."));
    }
}
