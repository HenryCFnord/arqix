//! Template Engine: package scaffolding (`doc init`) and template-governed
//! document creation with a deterministic unique-ID policy (`doc new`,
//! `unit new` — REQ-00-00-00-04/05, REQ-01-01-01/13-*). It creates new files
//! only, never overwriting existing ones (the containment/no-overwrite
//! discipline, REQ-00-00-00-08). Dates are left for `finalise` so the clock
//! is never read here.

use crate::OutputFormat;
use crate::diag::{self, Diagnostic, SCHEMA_VERSION};
use crate::parser::Document;
use serde_json::json;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

/// Per-kind ID scheme: (prefix, zero-padded width, ontology namespace).
fn scheme(kind: &str) -> (String, usize, String) {
    match kind {
        "adr" => ("ADR-".to_string(), 4, "adrs".to_string()),
        "unit" => ("unit-arc42-".to_string(), 2, "units".to_string()),
        other => (format!("{}-", other.to_uppercase()), 4, format!("{other}s")),
    }
}

/// The next free counter for a prefix, scanning existing document IDs.
fn next_counter(docs: &[Document], prefix: &str) -> u64 {
    let mut max = 0;
    for doc in docs {
        if let Some(id) = &doc.id
            && let Some(rest) = id.strip_prefix(prefix)
        {
            let digits: String = rest.chars().take_while(char::is_ascii_digit).collect();
            if let Ok(n) = digits.parse::<u64>() {
                max = max.max(n);
            }
        }
    }
    max
}

fn template(id: &str, kind: &str, namespace: &str) -> String {
    let slug = id.to_lowercase();
    format!(
        "---\n\
         id: {id}\n\
         title: New {Kind}\n\
         slug: {slug}\n\
         iri: arqix:{namespace}/{slug}\n\
         rdf:\n  type:\n    - arqix:classes/{kind}\n\
         triples: []\n\
         properties: {{}}\n\
         external-references: []\n\
         meta:\n  lifecycle-status: draft\n  owner: TODO\n  created: TODO\n  \
         updated: TODO\n  lang: en\n  generated: false\n\
         ---\n\n## New {Kind}\n\nTODO: write this {kind}.\n",
        Kind = capitalise(kind),
    )
}

fn capitalise(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

/// A kind is a plain lowercase slug — it becomes a path component and an
/// ID prefix, so anything else (separators, dots, uppercase) is a usage
/// error, not a write target (containment, REQ-00-00-00-13).
fn valid_kind(kind: &str) -> bool {
    !kind.is_empty()
        && kind
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

// arqix:implements REQ-00-00-00-05
// arqix:implements REQ-00-00-00-13
// arqix:implements REQ-01-01-13-01
// arqix:implements REQ-01-01-13-02
/// Create a document of `kind` from its template. Shared by `doc new` and
/// `unit new`.
pub fn new_document(kind: &str, format: OutputFormat) -> ExitCode {
    if !valid_kind(kind) {
        eprintln!("error: invalid kind '{kind}': expected a lowercase slug ([a-z0-9-])");
        return ExitCode::from(2);
    }
    let (prefix, width, namespace) = scheme(kind);
    let docs = crate::store::documents();
    let counter = next_counter(&docs, &prefix) + 1;
    let id = format!("{prefix}{counter:0width$}");

    let roots = crate::config::roots(Path::new("."));
    let root = roots.first().cloned().unwrap_or_else(|| "docs".to_string());
    let dir = PathBuf::from(&root).join(kind);
    let path = dir.join(format!("{id}.md"));

    if path.exists() {
        let diagnostic = Diagnostic::error(
            "TPL-001",
            format!("refusing to overwrite {}", path.display()),
        )
        .at(path.to_string_lossy());
        diag::emit(&[diagnostic], format);
        return ExitCode::from(1);
    }
    if let Err(err) = std::fs::create_dir_all(&dir) {
        eprintln!("error: cannot create {}: {err}", dir.display());
        return ExitCode::from(2);
    }
    if let Err(err) = std::fs::write(&path, template(&id, kind, &namespace)) {
        eprintln!("error: cannot write {}: {err}", path.display());
        return ExitCode::from(2);
    }

    let path_str = path.to_string_lossy().to_string();
    match format {
        OutputFormat::Json => {
            let result = json!({
                "schema_version": SCHEMA_VERSION,
                "id": id,
                "kind": kind,
                "path": path_str,
            });
            println!(
                "{}",
                serde_json::to_string_pretty(&result).expect("valid JSON")
            );
        }
        OutputFormat::Text => println!("created {id} at {path_str}"),
    }
    ExitCode::SUCCESS
}

/// The index.md a fresh package starts with (REQ-01-01-01-02: frontmatter
/// carrying id, kind=doc_index, and title).
const INDEX_TEMPLATE: &str = "---\n\
    id: doc-index\n\
    kind: doc_index\n\
    title: Documentation Index\n\
    ---\n\n\
    ## Documentation Index\n\n\
    TODO: introduce this documentation package.\n";

// arqix:implements REQ-01-01-01-01
// arqix:implements REQ-01-01-01-02
/// `arqix doc init [path]` — scaffold the standard package (REQ-01-01-01-01:
/// index.md, units/, pages/, artefacts/, logs/, .arqix/) at `path` or the
/// first configured root; never overwrites.
pub fn init(path: Option<&str>, format: OutputFormat) -> ExitCode {
    let mut roots = crate::config::roots(Path::new("."));
    let package = match path {
        Some(p) => p.to_string(),
        None => roots.first().cloned().unwrap_or_else(|| "docs".to_string()),
    };
    if path.is_none() {
        // A bare init also materialises the other configured roots.
        roots.retain(|r| *r != package);
        for root in &roots {
            if let Err(err) = std::fs::create_dir_all(root) {
                eprintln!("error: cannot create {root}: {err}");
                return ExitCode::from(2);
            }
        }
    }

    let package = Path::new(&package);
    for dir in ["units", "pages", "artefacts", "logs", ".arqix"] {
        let dir = package.join(dir);
        if let Err(err) = std::fs::create_dir_all(&dir) {
            eprintln!("error: cannot create {}: {err}", dir.display());
            return ExitCode::from(2);
        }
    }
    let index = package.join("index.md");
    if !index.exists()
        && let Err(err) = std::fs::write(&index, INDEX_TEMPLATE)
    {
        eprintln!("error: cannot write {}: {err}", index.display());
        return ExitCode::from(2);
    }

    let config = Path::new("arqix.toml");
    if !config.exists()
        && let Err(err) = std::fs::write(config, "# arqix configuration (schema v1)\n")
    {
        eprintln!("error: cannot write arqix.toml: {err}");
        return ExitCode::from(2);
    }
    if matches!(format, OutputFormat::Text) {
        println!("initialised documentation package");
    }
    ExitCode::SUCCESS
}
