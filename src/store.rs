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
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };
    let mut paths: Vec<_> = entries.flatten().map(|e| e.path()).collect();
    paths.sort();
    for path in paths {
        if path.is_dir() {
            // Never traverse directory symlinks: a parent link forms a
            // cycle that makes the walk unbounded, and the trace oracle's
            // rglob does not follow them either.
            if path.symlink_metadata().is_ok_and(|m| m.is_symlink()) {
                continue;
            }
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if !skip.iter().any(|s| s == name) {
                walk(&path, skip, docs);
            }
            continue;
        }
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if !name.ends_with(".md") || name.ends_with(".tpl.md") {
            continue;
        }
        if let Ok(text) = std::fs::read_to_string(&path) {
            // Normalise to forward slashes so the `file` field is identical
            // across platforms and consistent with the trace engine
            // (trace.rs) and assembler, which do the same.
            let rel = path.to_string_lossy().replace('\\', "/");
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

/// The catalog as data: shared by the CLI (`doc list`) and the MCP `list`
/// tool, so both surfaces answer identically (REQ-05-01-12-03).
pub(crate) fn catalog_json(kind: Option<&str>) -> Value {
    let docs = documents();
    let entries: Vec<Value> = docs
        .iter()
        .filter(|d| d.id.is_some())
        .filter(|d| kind.is_none_or(|k| d.kind() == k))
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

/// Full-text search as data.
pub(crate) fn search_json(query: &str) -> Value {
    let mut hits = Vec::new();
    for d in &documents() {
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
    let catalog = catalog_json(kind);
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
    let result = search_json(query);

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
