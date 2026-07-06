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

/// Directories never descended into during discovery (mirrors the oracle).
const SKIP_DIRS: [&str; 5] = [".git", "target", "node_modules", "__pycache__", "fixtures"];

fn discover() -> Vec<Document> {
    let mut docs = Vec::new();
    for root in crate::config::roots(Path::new(".")) {
        walk(Path::new(&root), &mut docs);
    }
    docs.sort_by(|a, b| a.file.cmp(&b.file));
    docs
}

fn walk(dir: &Path, docs: &mut Vec<Document>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };
    let mut paths: Vec<_> = entries.flatten().map(|e| e.path()).collect();
    paths.sort();
    for path in paths {
        if path.is_dir() {
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if !SKIP_DIRS.contains(&name) {
                walk(&path, docs);
            }
            continue;
        }
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if !name.ends_with(".md") || name.ends_with(".tpl.md") {
            continue;
        }
        if let Ok(text) = std::fs::read_to_string(&path) {
            docs.push(parser::parse(&path.to_string_lossy(), &text));
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

// arqix:implements REQ-05-01-08-01
// arqix:implements REQ-05-01-08-03
/// `arqix doc list [--kind <kind>]`
pub fn list(kind: Option<&str>, format: OutputFormat) -> ExitCode {
    let docs = discover();
    let entries: Vec<Value> = docs
        .iter()
        .filter(|d| d.id.is_some())
        .filter(|d| kind.is_none_or(|k| d.kind() == k))
        .map(catalog_entry)
        .collect();

    match format {
        OutputFormat::Json => {
            let catalog = json!({ "schema_version": SCHEMA_VERSION, "documents": entries });
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
    let docs = discover();
    match docs.iter().find(|d| d.id.as_deref() == Some(id)) {
        Some(d) => {
            match format {
                OutputFormat::Json => {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&document_json(d)).expect("valid JSON")
                    );
                }
                OutputFormat::Text => {
                    println!("{}", d.title.as_deref().unwrap_or(id));
                    println!();
                    println!("{}", d.body);
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
    let docs = discover();
    let mut hits = Vec::new();
    for d in &docs {
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

    match format {
        OutputFormat::Json => {
            let result = json!({ "schema_version": SCHEMA_VERSION, "query": query, "hits": hits });
            println!(
                "{}",
                serde_json::to_string_pretty(&result).expect("valid JSON")
            );
        }
        OutputFormat::Text => {
            for hit in &hits {
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
