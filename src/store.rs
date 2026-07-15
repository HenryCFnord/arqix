//! Document Store & Catalog: discovery over the configured roots, the JSON
//! catalog (REQ-05-01-08-*), reading documents by id (REQ-05-01-10-*), and
//! full-text search (REQ-02-01-06-*). Reads through the Document Parser;
//! never mutates source documents.

use crate::OutputFormat;
use crate::diag::{self, Diagnostic, SCHEMA_VERSION};
use crate::parser::{self, Document};
use serde_json::{Value, json};
use std::path::Path;
use std::process::ExitCode;

// arqix:implements REQ-01-01-17-01
/// Discover and parse every document under the configured roots, sorted by
/// path for determinism, skipping the configured `skip-dirs`
/// (REQ-01-01-17-01/-02). Shared by the linter, trace engine, and verifier.
pub fn documents() -> Vec<Document> {
    let skip = crate::config::skip_dirs(Path::new("."));
    let mut docs = Vec::new();
    for root in crate::config::roots(Path::new(".")) {
        walk(Path::new(&root), &skip, &mut docs);
    }
    docs.sort_by(|a, b| a.file.cmp(&b.file));
    // Overlapping roots (e.g. `docs` and `docs/en`) discover the same file
    // once per containing root; the catalog lists it once.
    docs.dedup_by(|a, b| a.file == b.file);
    docs
}

fn walk(dir: &Path, skip: &[String], docs: &mut Vec<Document>) {
    // The traversal (sorted, depth-first, symlink-safe, skip-dirs-pruned,
    // `.md` / not-`.tpl.md`) is the shared walker; the store-specific leaf
    // action is to read and parse each file.
    let mut paths = Vec::new();
    crate::util::collect_markdown(dir, skip, &mut paths);
    for path in paths {
        if let Ok(text) = std::fs::read_to_string(&path) {
            // Normalise to forward slashes so the `file` field is identical
            // across platforms and consistent with the trace engine
            // (trace.rs) and assembler, which do the same.
            let rel = crate::util::to_posix(&path);
            docs.push(parser::parse(&rel, &text));
        }
    }
}

fn catalog_entry(d: &Document) -> Value {
    json!({
        "id": d.id,
        "title": d.title,
        "kind": d.kind(),
        "file": d.file,
        "lang": d.lang,
    })
}

fn document_json(d: &Document) -> Value {
    json!({
        "schema_version": SCHEMA_VERSION,
        "id": d.id,
        "title": d.title,
        "iri": d.iri,
        "kind": d.kind(),
        "lang": d.lang,
        "file": d.file,
        "body": d.body,
    })
}

/// The declared `meta.lifecycle-status` frontmatter value, if any.
fn lifecycle_status(d: &Document) -> Option<&str> {
    d.frontmatter
        .iter()
        .find_map(|line| line.trim().strip_prefix("lifecycle-status:"))
        .map(str::trim)
        .filter(|v| !v.is_empty())
}

/// The catalog as data: shared by the CLI (`doc list`) and the MCP `list`
/// tool, so both surfaces answer identically (REQ-05-01-12-03). `lifecycle`
/// keeps only documents whose declared lifecycle-status equals the value; a
/// document without a lifecycle line never matches a lifecycle filter.
pub(crate) fn catalog_json(kind: Option<&str>, lifecycle: Option<&str>) -> Value {
    let docs = documents();
    let entries: Vec<Value> = docs
        .iter()
        .filter(|d| d.id.is_some())
        .filter(|d| kind.is_none_or(|k| d.kind() == k))
        .filter(|d| lifecycle.is_none_or(|l| lifecycle_status(d) == Some(l)))
        .map(catalog_entry)
        .collect();
    json!({ "schema_version": SCHEMA_VERSION, "documents": entries })
}

/// A document by id as data; `None` when no document has the id.
pub(crate) fn read_json(id: &str) -> Option<Value> {
    documents()
        .iter()
        .find(|d| d.id.as_deref() == Some(id))
        .map(document_json)
}

/// Full-text search as data. `kind` keeps only hits in documents of that
/// catalog kind; `path` keeps only hits in files whose repository-relative
/// path starts with the prefix. Both combine with each other and the query.
pub(crate) fn search_json(query: &str, kind: Option<&str>, path: Option<&str>) -> Value {
    let mut hits = Vec::new();
    for d in &documents() {
        if !kind.is_none_or(|k| d.kind() == k) || !path.is_none_or(|p| d.file.starts_with(p)) {
            continue;
        }
        if let Ok(text) = std::fs::read_to_string(&d.file) {
            for (idx, line) in text.lines().enumerate() {
                if line.contains(query) {
                    hits.push(json!({
                        "id": d.id,
                        "file": d.file,
                        "line": idx + 1,
                    }));
                }
            }
        }
    }
    json!({ "schema_version": SCHEMA_VERSION, "query": query, "hits": hits })
}

// arqix:implements REQ-05-01-08-01
// arqix:implements REQ-05-01-08-03
/// `arqix doc list [--kind <kind>]`
pub fn list(kind: Option<&str>, format: OutputFormat) -> ExitCode {
    let catalog = catalog_json(kind, None);
    let entries = catalog["documents"].as_array().cloned().unwrap_or_default();

    match format {
        OutputFormat::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(&catalog).expect("valid JSON")
            );
        }
        OutputFormat::Text => {
            for entry in &entries {
                println!(
                    "{}\t{}\t{}",
                    entry["id"].as_str().unwrap_or("?"),
                    entry["kind"].as_str().unwrap_or("?"),
                    entry["title"].as_str().unwrap_or(""),
                );
            }
        }
    }
    ExitCode::SUCCESS
}

// arqix:implements REQ-05-01-10-01
// arqix:implements REQ-05-01-10-03
/// `arqix doc read <id>`
pub fn read(id: &str, format: OutputFormat) -> ExitCode {
    match read_json(id) {
        Some(doc) => {
            match format {
                OutputFormat::Json => {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&doc).expect("valid JSON")
                    );
                }
                OutputFormat::Text => {
                    println!("{}", doc["title"].as_str().unwrap_or(id));
                    println!();
                    println!("{}", doc["body"].as_str().unwrap_or(""));
                }
            }
            ExitCode::SUCCESS
        }
        None => {
            // A missing document is a different miss from a missing selector
            // within an existing document (REQ-05-01-10-03).
            let diagnostic = Diagnostic::error("DOC-001", format!("no document has id {id}"));
            diag::emit(&[diagnostic], format);
            ExitCode::from(1)
        }
    }
}

// arqix:implements REQ-02-01-06-01
/// `arqix doc search <query>`
pub fn search(query: &str, format: OutputFormat) -> ExitCode {
    let result = search_json(query, None, None);

    match format {
        OutputFormat::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(&result).expect("valid JSON")
            );
        }
        OutputFormat::Text => {
            for hit in result["hits"].as_array().map(Vec::as_slice).unwrap_or(&[]) {
                println!(
                    "{}:{}: {}",
                    hit["file"].as_str().unwrap_or("?"),
                    hit["line"],
                    hit["id"].as_str().unwrap_or("?"),
                );
            }
        }
    }
    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fresh_dir(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("arqix-store-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    /// Characterization pin for `walk` (slice 3): it applies the shared
    /// traversal (sorted, depth-first, `skip-dirs`-pruned, `.md` /
    /// not-`.tpl.md`) and its store-specific leaf action — parse each file
    /// and record its posix-normalized path as `doc.file`. Pins the behaviour
    /// before the traversal core moves into `crate::util`.
    // arqix:no-requirement
    #[test]
    fn walk_parses_the_filtered_tree_in_traversal_order() {
        let root = fresh_dir("walk");
        std::fs::create_dir_all(root.join("sub")).unwrap();
        std::fs::create_dir_all(root.join("node_modules")).unwrap();
        for rel in [
            "z.md",
            "a.md",
            "b.tpl.md",
            "c.txt",
            "sub/m.md",
            "node_modules/skipped.md",
        ] {
            std::fs::write(root.join(rel), "---\nid: X\n---\n\nBody.\n").unwrap();
        }
        let mut docs = Vec::new();
        walk(&root, &["node_modules".to_string()], &mut docs);
        // Same filtered, sorted, depth-first set as the raw walker: `sub`
        // sorts before `z.md`, so its member is parsed before it; `.tpl.md`,
        // `.txt`, and the skip-dir are excluded.
        assert_eq!(docs.len(), 3);
        assert!(docs[0].file.ends_with("/a.md"), "first: {}", docs[0].file);
        assert!(
            docs[1].file.ends_with("/sub/m.md"),
            "second: {}",
            docs[1].file
        );
        assert!(docs[2].file.ends_with("/z.md"), "third: {}", docs[2].file);
        // Each recorded `file` is posix-normalized (no backslashes).
        assert!(
            docs.iter().all(|d| !d.file.contains('\\')),
            "paths are posix-normalized"
        );
    }
}
