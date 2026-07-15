//! Assembler: expands `<!-- arqix:include <path> -->` directives into
//! assembled pages under `pages/` and records a JSONL assembly log
//! (REQ-02-01-11-*, REQ-04-01-01-*). It only ever writes generated
//! artefacts — source documents are never mutated (the single-mutator
//! discipline stays with the rewriter, ADR-0004). Include cycles are a hard
//! structural error (ASM-001): the run fails with a message that names the
//! cycle.

use crate::OutputFormat;
use crate::diag::{self, Diagnostic};
use crate::linter::{IncludeLevel, include_directive};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

/// The generated-artefact root and the assembly log within it. The log path
/// is derived from this root; making it independently configurable lands with
/// the render/publish stories (REQ-04-01-01-03, v1 default).
const OUT_ROOT: &str = "pages";
const LOG_NAME: &str = "assembly.jsonl";

// arqix:implements REQ-02-01-11-01
// arqix:implements REQ-02-01-11-03
// arqix:implements REQ-04-01-01-02
// arqix:implements REQ-04-01-01-03
// arqix:implements REQ-04-01-01-04
// arqix:implements REQ-04-01-01-05
/// `arqix assemble build`
pub fn build(format: OutputFormat) -> ExitCode {
    let roots = crate::config::roots(Path::new("."));
    let docs = crate::store::documents();

    let mut records: Vec<Value> = Vec::new();
    let mut pages: Vec<(PathBuf, String)> = Vec::new();
    let mut diagnostics: Vec<Diagnostic> = Vec::new();
    let mut out_owners: std::collections::HashMap<String, String> =
        std::collections::HashMap::new();

    for doc in &docs {
        let src = Path::new(&doc.file);
        let out_rel = out_path(&doc.file, &roots);
        // Two sources mapping to one output would silently overwrite each
        // other (overlapping root-relative names across configured roots);
        // the first (path-sorted) source owns the page, the rest are a
        // structural error (ASM-003).
        if let Some(owner) = out_owners.get(&out_rel) {
            diagnostics.push(
                Diagnostic::error(
                    "ASM-003",
                    format!("output collision: {out_rel} is already generated from {owner}"),
                )
                .at(&doc.file),
            );
            continue;
        }
        out_owners.insert(out_rel.clone(), doc.file.clone());
        let mut stack: Vec<PathBuf> = Vec::new();
        let mut doc_records: Vec<Value> = Vec::new();
        let walk = Walk {
            doc_rel: &doc.file,
            out_rel: &out_rel,
            root_dir: src.parent().unwrap_or_else(|| Path::new(".")).to_path_buf(),
            ownership: crate::config::heading_ownership(Path::new(".")),
        };
        match expand(src, 0, &walk, 0, None, &mut stack, &mut doc_records) {
            Ok(content) => {
                records.append(&mut doc_records);
                pages.push((PathBuf::from(&out_rel), content));
            }
            // A failed page contributes no partial output or log records.
            Err(diagnostic) => diagnostics.push(diagnostic),
        }
    }

    if let Err(code) = write_outputs(&pages, &records) {
        return code;
    }

    diagnostics.sort_by(|a, b| (&a.file, a.line, a.code).cmp(&(&b.file, b.line, b.code)));
    diag::emit(&diagnostics, format);
    if diagnostics.is_empty() && matches!(format, OutputFormat::Text) {
        println!(
            "assembled {} page(s) in {} step(s)",
            pages.len(),
            records.len()
        );
    }
    diag::exit_code(&diagnostics)
}

/// Write the assembled pages and the JSONL log. Any I/O failure is a
/// system-level error (exit 2), distinct from the findings exit code.
fn write_outputs(pages: &[(PathBuf, String)], records: &[Value]) -> Result<(), ExitCode> {
    for (out, content) in pages {
        if let Some(parent) = out.parent()
            && let Err(err) = std::fs::create_dir_all(parent)
        {
            eprintln!("error: cannot create {}: {err}", parent.display());
            return Err(ExitCode::from(2));
        }
        if let Err(err) = std::fs::write(out, content) {
            eprintln!("error: cannot write {}: {err}", out.display());
            return Err(ExitCode::from(2));
        }
    }

    if let Err(err) = std::fs::create_dir_all(OUT_ROOT) {
        eprintln!("error: cannot create {OUT_ROOT}: {err}");
        return Err(ExitCode::from(2));
    }
    let mut log = String::new();
    for record in records {
        log.push_str(&serde_json::to_string(record).expect("valid JSON"));
        log.push('\n');
    }
    let log_path = Path::new(OUT_ROOT).join(LOG_NAME);
    if let Err(err) = std::fs::write(&log_path, log) {
        eprintln!("error: cannot write {}: {err}", log_path.display());
        return Err(ExitCode::from(2));
    }
    Ok(())
}

/// Expand one source document for a downstream consumer (the publisher's
/// staging): the same include walk as `build`, without keeping log records.
pub(crate) fn expand_document(file: &Path) -> Result<String, Diagnostic> {
    let mut stack = Vec::new();
    let mut records = Vec::new();
    let rel = crate::util::to_posix(file);
    let walk = Walk {
        doc_rel: &rel,
        out_rel: "",
        root_dir: file
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .to_path_buf(),
        ownership: crate::config::heading_ownership(Path::new(".")),
    };
    expand(file, 0, &walk, 0, None, &mut stack, &mut records)
}

/// One document's expansion context, shared by every fragment of the walk.
struct Walk<'a> {
    doc_rel: &'a str,
    out_rel: &'a str,
    /// The root document's directory: every fragment link is rebased here
    /// so the assembled page is artefact-ready (REQ-04-01-03-02).
    root_dir: PathBuf,
    /// `[policies.assemble] heading-ownership` (ADR-0013): `child` or
    /// `parent` — governs what a bare include means.
    ownership: String,
}

// arqix:implements REQ-02-01-12-01
// arqix:implements REQ-02-01-12-02
// arqix:implements REQ-02-01-12-03
/// Expand one source fragment, following include directives depth-first. Each
/// fragment read is one assembly step and appends exactly one log record. A
/// path already on the DFS stack is a cycle (ASM-001). `base_level` is the
/// heading level in effect at the include position; `target_level` is the
/// resolved level the fragment's first heading must land on (None inlines
/// verbatim — the root document and parent-owned bare includes).
/// The number of leading lines a YAML frontmatter block occupies, through its
/// closing `---` (0 when the text has no frontmatter). Used to skip a stitched
/// fragment's own frontmatter so it never lands in the assembled body.
fn frontmatter_line_count(text: &str) -> usize {
    let mut lines = text.lines();
    if lines.next() != Some("---") {
        return 0;
    }
    let mut n = 1;
    for line in lines {
        n += 1;
        if line == "---" {
            return n;
        }
    }
    0
}

fn expand(
    file: &Path,
    at_line: usize,
    walk: &Walk,
    base_level: i64,
    target_level: Option<i64>,
    stack: &mut Vec<PathBuf>,
    records: &mut Vec<Value>,
) -> Result<String, Diagnostic> {
    let text = match std::fs::read_to_string(file) {
        Ok(text) => text,
        Err(err) => {
            return Err(
                Diagnostic::error("ASM-002", format!("cannot read {}: {err}", rel(file)))
                    .at_line(rel(file), at_line.max(1)),
            );
        }
    };

    // The stack keeps each fragment's relative path; cycle comparison
    // canonicalises on the fly so aliased paths still match, while the
    // diagnostic chain stays in consistent relative form.
    stack.push(file.to_path_buf());

    // The whole fragment shifts by one delta: declared level minus the
    // first heading (ADR-0013). A headingless fragment has nothing to
    // shift, whatever was declared.
    let shift = match (target_level, first_heading_level(&text)) {
        (Some(target), Some(first)) => target - first,
        _ => 0,
    };

    // REQ-04-01-01-04/-05: one stable record per step, carrying the required
    // fields. `include` is the fragment read; `at_line` is where its parent
    // pulled it in (0 for the page root); `level` is the resolved outline
    // decision (null for verbatim inlining and the root).
    records.push(json!({
        "doc": walk.doc_rel,
        "chapter_id": chapter_id(file, &text),
        "out": walk.out_rel,
        "include": rel(file),
        "sha256": sha256_hex(text.as_bytes()),
        "bytes": text.len(),
        "at_line": at_line,
        "level": target_level,
    }));

    let dir = file.parent().unwrap_or_else(|| Path::new("."));
    let is_fragment = at_line > 0;
    // A fragment carries its own YAML frontmatter; a stitched document must not
    // inline it, or it renders as a stray metadata block in the assembled page
    // (found on arqix.dev). Skip those leading lines rather than slicing, so
    // `idx` stays aligned to the source file for diagnostics. The root document
    // keeps its frontmatter for the toolchain to consume.
    let frontmatter_end = if is_fragment {
        frontmatter_line_count(&text)
    } else {
        0
    };
    let mut current = base_level;
    let mut in_fence = false;
    let mut out = String::new();
    for (idx, line) in text.lines().enumerate() {
        if idx < frontmatter_end {
            continue;
        }
        // Fenced code is opaque: no headings, no directives, no links.
        if line.trim_start().starts_with("```") {
            in_fence = !in_fence;
            out.push_str(line);
            out.push('\n');
            continue;
        }
        if in_fence {
            out.push_str(line);
            out.push('\n');
            continue;
        }

        if let Some((target, declared)) = include_directive(line) {
            let target_path = dir.join(&target);
            if target_path.exists() {
                let target_key = canonical(&target_path);
                // Containment (REQ-00-00-00-13): a target resolving outside
                // the repository root is never read or inlined (ASM-004).
                if !target_key.starts_with(canonical(Path::new("."))) {
                    return Err(Diagnostic::error(
                        "ASM-004",
                        format!("include target escapes the repository root: {target}"),
                    )
                    .at_line(rel(file), idx + 1));
                }
                // The tighter fence (REQ-02-01-09-02): inside the repository
                // but outside every configured root is still out of bounds —
                // corpus composition only ever reads the corpus (ASM-006).
                if !crate::config::roots(Path::new("."))
                    .iter()
                    .any(|root| target_key.starts_with(canonical(Path::new(root))))
                {
                    return Err(Diagnostic::error(
                        "ASM-006",
                        format!("include target escapes the configured roots: {target}"),
                    )
                    .at_line(rel(file), idx + 1));
                }
                // Detect the cycle here, where the re-including directive's
                // own location (this file at this line) is known, so ASM-001
                // anchors the real directive rather than the child fragment.
                if stack.iter().any(|p| canonical(p) == target_key) {
                    let mut chain: Vec<String> = stack.iter().map(|p| rel(p)).collect();
                    chain.push(rel(&target_path));
                    return Err(Diagnostic::error(
                        "ASM-001",
                        format!("include cycle detected: {}", chain.join(" -> ")),
                    )
                    .at_line(rel(file), idx + 1));
                }
                // Resolve where the child's headings land (ADR-0013): an
                // absolute level stands, a relative one counts from the
                // heading in effect here, and a bare include follows the
                // configured ownership — child owns its heading one level
                // below (+1), parent-owned corpora inline verbatim.
                let child_target = match declared {
                    Some(IncludeLevel::Absolute(level)) => Some(level),
                    Some(IncludeLevel::Relative(delta)) => Some(current + delta),
                    None if walk.ownership == "parent" => None,
                    None => Some(current + 1),
                };
                // `current` stays the parent's own outline position
                // (ADR-0013: "the last heading the assembler has seen in
                // the parent"): sibling includes under one section land at
                // the same level, whatever depth the previous fragment
                // reached.
                let nested = expand(
                    &target_path,
                    idx + 1,
                    walk,
                    current,
                    child_target,
                    stack,
                    records,
                )?;
                out.push_str(&nested);
                if !nested.ends_with('\n') {
                    out.push('\n');
                }
                continue;
            }
            // A missing include target is left verbatim; the linter (LNT-001)
            // is the tool that flags it.
        }

        if let Some(level) = crate::markdown::heading_level(line) {
            let effective = level + shift;
            // A shift out of the h1..h6 range is a structural error, never
            // a silent clamp (ASM-005).
            if !(1..=6).contains(&effective) {
                return Err(Diagnostic::error(
                    "ASM-005",
                    format!(
                        "heading shift overflows the outline: '{}' would land at level {effective} in {}",
                        line.trim_matches('#').trim(),
                        rel(file)
                    ),
                )
                .at_line(rel(file), idx + 1));
            }
            current = effective;
            out.push_str(&"#".repeat(effective as usize));
            let rest = &line[level as usize..];
            if is_fragment {
                out.push_str(&rebase_links(rest, dir, &walk.root_dir));
            } else {
                out.push_str(rest);
            }
            out.push('\n');
            continue;
        }

        if is_fragment {
            out.push_str(&rebase_links(line, dir, &walk.root_dir));
        } else {
            out.push_str(line);
        }
        out.push('\n');
    }

    stack.pop();
    Ok(out)
}

/// The first heading of a fragment, skipping fenced code. Thin delegate over
/// the shared `markdown::headings_outside_fences` scan (Phase A slice 2).
fn first_heading_level(text: &str) -> Option<i64> {
    crate::markdown::headings_outside_fences(text)
        .next()
        .map(|(level, _)| level)
}

/// Rewrite the inline link targets of one fragment line so they resolve from
/// the root document's directory instead of the fragment's (artefact-ready
/// pages, REQ-04-01-03-02). Absolute targets, anchors, and URLs pass through.
fn rebase_links(line: &str, from_dir: &Path, root_dir: &Path) -> String {
    let mut out = String::new();
    let mut rest = line;
    while let Some(pos) = rest.find("](") {
        let (head, tail) = rest.split_at(pos + 2);
        out.push_str(head);
        let Some(end) = tail.find(')') else {
            out.push_str(tail);
            return out;
        };
        out.push_str(&rebased_target(&tail[..end], from_dir, root_dir));
        out.push(')');
        rest = &tail[end + 1..];
    }
    out.push_str(rest);
    out
}

fn rebased_target(target: &str, from_dir: &Path, root_dir: &Path) -> String {
    if target.is_empty()
        || target.starts_with('#')
        || target.starts_with('/')
        || target.starts_with('<')
        || target.contains("://")
        || target.starts_with("mailto:")
    {
        return target.to_string();
    }
    let (path, anchor) = match target.split_once('#') {
        Some((path, anchor)) => (path, Some(anchor)),
        None => (target, None),
    };
    if path.is_empty() {
        return target.to_string();
    }
    let joined = normalized_parts(&from_dir.join(path));
    let base = normalized_parts(root_dir);
    let common = joined
        .iter()
        .zip(base.iter())
        .take_while(|(a, b)| a == b)
        .count();
    let mut parts: Vec<String> = vec!["..".to_string(); base.len() - common];
    parts.extend(joined[common..].iter().cloned());
    let rebased = if parts.is_empty() {
        ".".to_string()
    } else {
        parts.join("/")
    };
    match anchor {
        Some(anchor) => format!("{rebased}#{anchor}"),
        None => rebased,
    }
}

/// Path components with `.` and `..` folded, in posix form.
fn normalized_parts(path: &Path) -> Vec<String> {
    let mut parts: Vec<String> = Vec::new();
    for component in path.components() {
        match component {
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                if parts.last().is_some_and(|p| p != "..") {
                    parts.pop();
                } else {
                    parts.push("..".to_string());
                }
            }
            std::path::Component::Normal(part) => {
                parts.push(part.to_string_lossy().to_string());
            }
            _ => {}
        }
    }
    parts
}

/// The output page path for a source file: its path relative to the matching
/// configured root, remapped under `pages/`.
fn out_path(file: &str, roots: &[String]) -> String {
    let normalized = crate::util::to_posix_str(file);
    for root in roots {
        let prefix = format!("{}/", root.trim_end_matches('/'));
        if let Some(rest) = normalized.strip_prefix(&prefix) {
            return format!("{OUT_ROOT}/{rest}");
        }
    }
    let name = Path::new(&normalized)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("page.md");
    format!("{OUT_ROOT}/{name}")
}

/// The fragment's chapter identity: its frontmatter `id`, else its file stem.
fn chapter_id(file: &Path, text: &str) -> String {
    let doc = crate::parser::parse(&file.to_string_lossy(), text);
    doc.id.unwrap_or_else(|| {
        file.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string()
    })
}

/// A stable relative display path (forward slashes) for a fragment.
fn rel(file: &Path) -> String {
    crate::util::to_posix(file)
}

/// Canonicalise a path for cycle comparison, falling back to the path itself
/// when it cannot be resolved (e.g. it does not exist).
fn canonical(file: &Path) -> PathBuf {
    std::fs::canonicalize(file).unwrap_or_else(|_| file.to_path_buf())
}

/// The lowercase hex SHA-256 of `data` — the content-identity fingerprint in
/// the assembly log (REQ-04-01-01-05), computed by the `sha2` crate.
fn sha256_hex(data: &[u8]) -> String {
    let mut out = String::with_capacity(64);
    for byte in Sha256::digest(data) {
        write!(out, "{byte:02x}").expect("writing to a String never fails");
    }
    out
}

#[cfg(test)]
mod tests {
    use super::sha256_hex;

    // arqix:no-requirement
    #[test]
    fn sha256_hex_matches_the_nist_vector() {
        // Pins the crate wiring and the hex rendering the log depends on.
        assert_eq!(
            sha256_hex(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }
}
