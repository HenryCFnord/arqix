//! Report & Export: `report bundle` exports a scoped evidence bundle —
//! linked requirements, stories, and trace evidence for a chosen scope,
//! deterministic for identical inputs (REQ-03-01-04-01..03), in the audit
//! formats Markdown, CSV, and JSON with stable schemas and caller-provided
//! generation metadata (REQ-04-01-12-01..03; the injected-clock discipline
//! keeps the wall clock out of the engine).

use crate::OutputFormat;
use crate::diag::SCHEMA_VERSION;
use serde_json::{Value, json};
use std::collections::BTreeSet;
use std::path::Path;
use std::process::ExitCode;

// arqix:implements REQ-03-01-04-01
// arqix:implements REQ-03-01-04-02
// arqix:implements REQ-03-01-04-03
// arqix:implements REQ-04-01-12-01
// arqix:implements REQ-04-01-12-02
// arqix:implements REQ-04-01-12-03
/// `arqix report bundle <ID>... [--out <dir>] [--stamp <text>]`
pub fn bundle(
    ids: &[String],
    out: Option<&str>,
    stamp: Option<&str>,
    format: OutputFormat,
) -> ExitCode {
    let model = crate::trace::corpus_model();
    let (coverage, _) = crate::trace::coverage_report(&model);

    // Resolve the scope: requirement IDs stand for themselves; a story ID
    // pulls in every requirement derived from it (the declared triples are
    // the source of truth, ADR-0012).
    let scope: BTreeSet<String> = match crate::trace::resolve_scope(&model, ids) {
        Ok(scope) => scope,
        Err(message) => {
            eprintln!("error: {message}");
            return ExitCode::from(2);
        }
    };

    let rows: Vec<Value> = coverage["requirements"]
        .as_array()
        .map(|rows| {
            rows.iter()
                .filter(|row| row["id"].as_str().is_some_and(|id| scope.contains(id)))
                .cloned()
                .collect()
        })
        .unwrap_or_default();
    let inputs = crate::trace::requirement_files(&model, &scope);

    let mut scope_list: Vec<String> = ids.to_vec();
    scope_list.sort();
    let bundle = json!({
        "schema_version": SCHEMA_VERSION,
        "stamp": stamp.unwrap_or("unstamped"),
        "scope": scope_list,
        "requirements": rows,
        "inputs": inputs,
    });

    // The bundle directory: JSON for automation, Markdown for review, the
    // scoped matrix as CSV — reviewable without reshaping.
    let out_dir = Path::new(out.unwrap_or("bundle"));
    if let Err(err) = std::fs::create_dir_all(out_dir) {
        eprintln!("error: cannot create {}: {err}", out_dir.display());
        return ExitCode::from(2);
    }
    let json_text = serde_json::to_string_pretty(&bundle).expect("valid JSON");
    let mut markdown = String::from(
        "| requirement | kind | story | verified by | planned by | implemented by |\n\
         | --- | --- | --- | --- | --- | --- |\n",
    );
    for row in &rows {
        markdown.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} |\n",
            row["id"].as_str().unwrap_or("?"),
            row["kind"].as_str().unwrap_or("?"),
            row["story"].as_str().unwrap_or("—"),
            joined(&row["verified_by"]),
            joined(&row["planned_by"]),
            joined(&row["implemented_by"]),
        ));
    }
    let csv = crate::trace::matrix_csv_scoped(&model, &scope);
    for (name, content) in [
        ("bundle.json", json_text.as_str()),
        ("evidence.md", markdown.as_str()),
        ("matrix.csv", csv.as_str()),
    ] {
        if let Err(err) = std::fs::write(out_dir.join(name), content) {
            eprintln!(
                "error: cannot write {}: {err}",
                out_dir.join(name).display()
            );
            return ExitCode::from(2);
        }
    }

    match format {
        OutputFormat::Json => println!("{json_text}"),
        OutputFormat::Text => println!(
            "bundled {} requirement(s) to {}",
            rows.len(),
            out_dir.display()
        ),
    }
    ExitCode::SUCCESS
}

fn joined(value: &Value) -> String {
    let items: Vec<&str> = value
        .as_array()
        .map(|a| a.iter().filter_map(Value::as_str).collect())
        .unwrap_or_default();
    if items.is_empty() {
        "—".to_string()
    } else {
        items.join("; ")
    }
}
