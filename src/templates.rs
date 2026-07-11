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

fn template(id: &str, kind: &str, namespace: &str, title: &str, slug: &str) -> String {
    let iri_slug = id.to_lowercase();
    // A requirement's lifecycle vocabulary is active/retired only
    // (LNT-004): an unfinished obligation is not yet a requirement, so
    // the scaffold declares what the default gates accept.
    let lifecycle = if kind == "requirement" {
        "active"
    } else {
        "draft"
    };
    format!(
        "---\n\
         id: {id}\n\
         title: {title}\n\
         slug: {slug}\n\
         iri: arqix:{namespace}/{iri_slug}\n\
         rdf:\n  type:\n    - arqix:classes/{kind}\n\
         triples: []\n\
         properties: {{}}\n\
         external-references: []\n\
         meta:\n  lifecycle-status: {lifecycle}\n  owner: TODO\n  created: TODO\n  \
         updated: TODO\n  lang: en\n  generated: false\n\
         ---\n\n## {title}\n\nTODO: write this {kind}.\n",
    )
}

/// The `{slug}` placeholder value: the title lowered to a hyphen slug.
fn slugify(title: &str) -> String {
    let mut slug = String::new();
    for c in title.chars() {
        if c.is_ascii_alphanumeric() {
            slug.extend(c.to_lowercase());
        } else if !slug.ends_with('-') && !slug.is_empty() {
            slug.push('-');
        }
    }
    slug.trim_end_matches('-').to_string()
}

/// An explicit ID becomes a filename and an IRI segment, so it is held to
/// the same containment bar as a kind (REQ-00-00-00-13), plus uppercase.
fn valid_id(id: &str) -> bool {
    !id.is_empty() && id.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
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

/// Caller-chosen parts of `doc new`; everything unset falls back to the
/// configured policy (generated ID, template title).
#[derive(Default)]
pub struct NewOptions<'a> {
    pub title: Option<&'a str>,
    pub id: Option<&'a str>,
    pub dry_run: bool,
}

// arqix:implements REQ-00-00-00-05
// arqix:implements REQ-00-00-00-09
// arqix:implements REQ-00-00-00-13
// arqix:implements REQ-01-01-13-01
// arqix:implements REQ-01-01-13-02
/// Create a document of `kind` from its template. Shared by `doc new` and
/// `unit new`.
pub fn new_document(kind: &str, options: NewOptions, format: OutputFormat) -> ExitCode {
    if !valid_kind(kind) {
        eprintln!("error: invalid kind '{kind}': expected a lowercase slug ([a-z0-9-])");
        return ExitCode::from(2);
    }
    let (prefix, width, namespace) = scheme(kind);
    let docs = crate::store::documents();
    let id = match options.id {
        Some(explicit) => {
            if !valid_id(explicit) {
                eprintln!("error: invalid id '{explicit}': expected an ID slug ([A-Za-z0-9-])");
                return ExitCode::from(2);
            }
            if let Some(holder) = docs.iter().find(|doc| doc.id.as_deref() == Some(explicit)) {
                let diagnostic = Diagnostic::error(
                    "TPL-002",
                    format!("id {explicit} is already used by {}", holder.file),
                )
                .at(&holder.file);
                diag::emit(&[diagnostic], format);
                return ExitCode::from(1);
            }
            explicit.to_string()
        }
        None => {
            let counter = next_counter(&docs, &prefix) + 1;
            format!("{prefix}{counter:0width$}")
        }
    };
    let title = match options.title {
        Some(title) => title.to_string(),
        None => format!("New {}", capitalise(kind)),
    };
    let slug = match options.title {
        Some(title) => slugify(title),
        None => id.to_lowercase(),
    };

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
    if !options.dry_run {
        if let Err(err) = std::fs::create_dir_all(&dir) {
            eprintln!("error: cannot create {}: {err}", dir.display());
            return ExitCode::from(2);
        }
        if let Err(err) = std::fs::write(&path, template(&id, kind, &namespace, &title, &slug)) {
            eprintln!("error: cannot write {}: {err}", path.display());
            return ExitCode::from(2);
        }
    }

    let path_str = path.to_string_lossy().to_string();
    match format {
        OutputFormat::Json => {
            let result = json!({
                "schema_version": SCHEMA_VERSION,
                "id": id,
                "kind": kind,
                "title": title,
                "slug": slug,
                "path": path_str,
                "dry_run": options.dry_run,
            });
            println!(
                "{}",
                serde_json::to_string_pretty(&result).expect("valid JSON")
            );
        }
        OutputFormat::Text if options.dry_run => println!("would create {id} at {path_str}"),
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
