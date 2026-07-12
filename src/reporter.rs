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

// arqix:implements REQ-05-01-15-01
// arqix:implements REQ-05-01-15-02
// arqix:implements REQ-05-01-15-03
/// `arqix report knowledge [--out <dir>]` — the corpus as an Open Knowledge
/// Format bundle (US-05-01-15): one artefact-ready concept document per
/// living corpus document, OKF fields mapped from declared metadata, the
/// publish scope and the lifecycle honoured.
pub fn knowledge(out: Option<&str>, format: OutputFormat) -> ExitCode {
    let policy = crate::config::publish_policy(Path::new("."));
    let default_lang = crate::config::default_lang(Path::new("."));
    let out_dir = Path::new(out.unwrap_or("knowledge"));

    let mut exported = 0usize;
    for root in crate::config::roots(Path::new(".")) {
        // The default language's root, exactly as the publisher resolves it.
        let lang_root = Path::new(&root).join(&default_lang);
        let lang_root = if lang_root.is_dir() {
            lang_root
        } else {
            std::path::PathBuf::from(&root)
        };

        for doc in crate::store::documents() {
            let file = Path::new(&doc.file);
            let Ok(rel) = file.strip_prefix(&lang_root) else {
                continue;
            };
            let rel_posix = rel.to_string_lossy().replace('\\', "/");
            // The publish scope and the lifecycle: excluded subtrees and
            // retired documents never become living knowledge (ADR-0010).
            if policy.exclude.iter().any(|e| {
                let prefix = e.trim_end_matches('/');
                rel_posix == prefix || rel_posix.starts_with(&format!("{prefix}/"))
            }) {
                continue;
            }
            if doc
                .frontmatter
                .iter()
                .any(|line| line.trim() == "lifecycle-status: retired")
            {
                continue;
            }

            let assembled = match crate::assembler::expand_document(file) {
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
            let expanded = crate::parser::parse(&doc.file, &assembled);
            if let Err(code) = write_concept(&out_dir.join(rel), &doc, &expanded) {
                return code;
            }
            exported += 1;
        }
    }

    match format {
        OutputFormat::Json => println!(
            "{}",
            serde_json::to_string_pretty(&json!({
                "schema_version": SCHEMA_VERSION,
                "out": out_dir.to_string_lossy(),
                "concepts": exported,
            }))
            .expect("valid JSON")
        ),
        OutputFormat::Text => {
            println!("exported {exported} concept(s) to {}", out_dir.display());
        }
    }
    ExitCode::SUCCESS
}

/// Write one OKF concept document: fields mapped from declared metadata —
/// `type` from the declared class (the generic document type otherwise),
/// `title` verbatim, `timestamp` from the declared update date; absent
/// metadata is omitted, never fabricated (REQ-05-01-15-02).
fn write_concept(
    path: &Path,
    doc: &crate::parser::Document,
    expanded: &crate::parser::Document,
) -> Result<(), ExitCode> {
    let mut front = String::from("---\n");
    let concept_type = doc
        .classes
        .first()
        .cloned()
        .unwrap_or_else(|| "document".to_string());
    front.push_str(&format!("type: {concept_type}\n"));
    if let Some(title) = &doc.title {
        let quoted = title.replace('\\', "\\\\").replace('"', "\\\"");
        front.push_str(&format!("title: \"{quoted}\"\n"));
    }
    if let Some(updated) = doc.frontmatter.iter().find_map(|line| {
        line.trim()
            .strip_prefix("updated:")
            .map(str::trim)
            .filter(|v| !v.is_empty())
    }) {
        front.push_str(&format!("timestamp: {updated}\n"));
    }
    front.push_str("---\n");

    let mut body = String::new();
    for line in expanded.body.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("<!--") && trimmed.ends_with("-->") && trimmed.contains("arqix:") {
            continue;
        }
        body.push_str(line);
        body.push('\n');
    }

    if let Some(parent) = path.parent()
        && let Err(err) = std::fs::create_dir_all(parent)
    {
        eprintln!("error: cannot create {}: {err}", parent.display());
        return Err(ExitCode::from(2));
    }
    if let Err(err) = std::fs::write(path, format!("{front}{body}")) {
        eprintln!("error: cannot write {}: {err}", path.display());
        return Err(ExitCode::from(2));
    }
    Ok(())
}
