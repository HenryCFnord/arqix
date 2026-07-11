//! Assembler: expands `<!-- arqix:include <path> -->` directives into
//! assembled pages under `pages/` and records a JSONL assembly log
//! (REQ-02-01-11-*, REQ-04-01-01-*). It only ever writes generated
//! artefacts — source documents are never mutated (the single-mutator
//! discipline stays with the rewriter, ADR-0004). Include cycles are a hard
//! structural error (ASM-001): the run fails with a message that names the
//! cycle.

use crate::OutputFormat;
use crate::diag::{self, Diagnostic};
use crate::linter::include_target;
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
        match expand(src, 0, &doc.file, &out_rel, &mut stack, &mut doc_records) {
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
    let rel = file.to_string_lossy().replace('\\', "/");
    expand(file, 0, &rel, "", &mut stack, &mut records)
}

/// Expand one source fragment, following include directives depth-first. Each
/// fragment read is one assembly step and appends exactly one log record. A
/// path already on the DFS stack is a cycle (ASM-001).
fn expand(
    file: &Path,
    at_line: usize,
    doc_rel: &str,
    out_rel: &str,
    stack: &mut Vec<PathBuf>,
    records: &mut Vec<Value>,
) -> Result<String, Diagnostic> {
    let text = match std::fs::read_to_string(file) {
        Ok(text) => text,
        Err(err) => {
            return Err(
                Diagnostic::error("ASM-002", format!("cannot read {}: {err}", rel(file)))
                    .at(rel(file)),
            );
        }
    };

    // The stack keeps each fragment's relative path; cycle comparison
    // canonicalises on the fly so aliased paths still match, while the
    // diagnostic chain stays in consistent relative form.
    stack.push(file.to_path_buf());

    // REQ-04-01-01-04/-05: one stable record per step, carrying the required
    // fields. `include` is the fragment read; `at_line` is where its parent
    // pulled it in (0 for the page root).
    records.push(json!({
        "doc": doc_rel,
        "chapter_id": chapter_id(file, &text),
        "out": out_rel,
        "include": rel(file),
        "sha256": sha256_hex(text.as_bytes()),
        "bytes": text.len(),
        "at_line": at_line,
    }));

    let dir = file.parent().unwrap_or_else(|| Path::new("."));
    let mut out = String::new();
    for (idx, line) in text.lines().enumerate() {
        if let Some(target) = include_target(line) {
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
                let nested = expand(&target_path, idx + 1, doc_rel, out_rel, stack, records)?;
                out.push_str(&nested);
                if !nested.ends_with('\n') {
                    out.push('\n');
                }
                continue;
            }
            // A missing include target is left verbatim; the linter (LNT-001)
            // is the tool that flags it.
        }
        out.push_str(line);
        out.push('\n');
    }

    stack.pop();
    Ok(out)
}

/// The output page path for a source file: its path relative to the matching
/// configured root, remapped under `pages/`.
fn out_path(file: &str, roots: &[String]) -> String {
    let normalized = file.replace('\\', "/");
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
    file.to_string_lossy().replace('\\', "/")
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
