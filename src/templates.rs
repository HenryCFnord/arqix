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
        // The ontology namespace is "user-stories", not the mechanical
        // plural — the generated iri must match the corpus namespaces.
        "user-story" => ("USER-STORY-".to_string(), 4, "user-stories".to_string()),
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

/// The shipped default template — a real template file embedded at build
/// time, not a string literal in the engine (US-01-01-20).
const DEFAULT_TEMPLATE: &str = include_str!("templates/default.tpl.md");

/// The agent-instructions scaffold `doc init` writes to the repository
/// root (US-08-01-24): a starting point for the repository's own
/// normative rules, naming the verification loop and the corpus entry
/// points.
const AGENTS_TEMPLATE: &str = include_str!("templates/agents.tpl.md");

/// The document families `doc init` scaffolds template files for.
const TEMPLATE_FAMILIES: [&str; 8] = [
    "adr",
    "page",
    "persona",
    "report",
    "requirement",
    "unit",
    "user-story",
    "workflow",
];

/// A requirement's lifecycle vocabulary is active/retired only (LNT-004):
/// an unfinished obligation is not yet a requirement, so the scaffold
/// declares what the default gates accept.
fn lifecycle_for(kind: &str) -> &'static str {
    if kind == "requirement" {
        "active"
    } else {
        "draft"
    }
}

/// The default template for one kind: the embedded template with the
/// kind-level placeholders baked in, the per-document ones kept.
fn default_template(kind: &str, namespace: &str) -> String {
    DEFAULT_TEMPLATE
        .replace("{kind}", kind)
        .replace("{namespace}", namespace)
        .replace("{lifecycle}", lifecycle_for(kind))
}

/// The template directory: the configured `[templates] dir`, or the
/// package-local `templates/` next to the first root. The bool says whether
/// it was configured explicitly — only then is a missing file an error.
fn template_dir() -> (PathBuf, bool) {
    match crate::config::templates_dir(Path::new(".")) {
        Some(dir) => (PathBuf::from(dir), true),
        None => {
            let root = crate::config::roots(Path::new("."))
                .first()
                .cloned()
                .unwrap_or_else(|| "docs".to_string());
            (Path::new(&root).join("templates"), false)
        }
    }
}

// arqix:implements REQ-01-01-20-01
// arqix:implements REQ-01-01-20-03
/// Instantiate the template for `kind`: the template file from the
/// configured directory (missing is a config error), the package-local
/// scaffold if one exists, or the embedded default — so an unconfigured
/// repository produces byte-identical documents.
fn template(
    id: &str,
    kind: &str,
    namespace: &str,
    title: &str,
    slug: &str,
    sets: &[(String, String)],
    format: OutputFormat,
) -> Result<String, ExitCode> {
    // A declared [kinds.<family>].template wins (REQ-08-01-25-02): the
    // contract names the file directly, and only this path validates the
    // placeholder vocabulary (REQ-08-01-25-03) — the directory and embedded
    // paths keep their present behaviour.
    if let Some(declared) = crate::config::kind_contracts(Path::new("."))
        .into_iter()
        .find(|contract| contract.family == kind)
        .and_then(|contract| contract.template)
    {
        let text = match std::fs::read_to_string(&declared) {
            Ok(text) => text,
            Err(err) => {
                eprintln!(
                    "error: no template for kind '{kind}': [kinds.{kind}] declares {declared}: {err}"
                );
                return Err(ExitCode::from(2));
            }
        };
        let text = apply_sets(&text, sets, &declared, format)?;
        let rendered = substitute(&text, id, kind, namespace, title, slug);
        if let Some(unknown) = leftover_placeholder(&rendered) {
            // A typo in a placeholder must become a finding, never a silent
            // literal in the created document.
            let diagnostic = Diagnostic::error(
                "TPL-002",
                format!(
                    "template {declared} uses unknown placeholder '{{{unknown}}}' \
                     (known: id, title, slug, iri_slug, kind, namespace, lifecycle)"
                ),
            )
            .at(&declared);
            diag::emit(&[diagnostic], format);
            return Err(ExitCode::from(1));
        }
        return Ok(rendered);
    }

    let (dir, configured) = template_dir();
    let path = dir.join(format!("{kind}.tpl.md"));
    let text = match std::fs::read_to_string(&path) {
        Ok(text) => text,
        Err(_) if configured => {
            eprintln!(
                "error: no template for kind '{kind}': expected {} (scaffold the defaults with `arqix doc init`)",
                path.display()
            );
            return Err(ExitCode::from(2));
        }
        Err(_) => default_template(kind, namespace),
    };
    let text = apply_sets(&text, sets, &path.display().to_string(), format)?;
    Ok(substitute(&text, id, kind, namespace, title, slug))
}

// arqix:implements REQ-08-01-32-01
/// Fill the template's own placeholders from `--set key=value` pairs. A key
/// the template does not use is a TPL-003 finding — a typo never vanishes
/// silently.
fn apply_sets(
    text: &str,
    sets: &[(String, String)],
    source: &str,
    format: OutputFormat,
) -> Result<String, ExitCode> {
    let mut rendered = text.to_string();
    for (key, value) in sets {
        let placeholder = format!("{{{key}}}");
        if !rendered.contains(&placeholder) {
            let diagnostic = Diagnostic::error(
                "TPL-003",
                format!("--set {key}: template {source} does not use placeholder '{{{key}}}'"),
            )
            .at(source);
            diag::emit(&[diagnostic], format);
            return Err(ExitCode::from(1));
        }
        rendered = rendered.replace(&placeholder, value);
    }
    Ok(rendered)
}

/// Fill a kind's id/dir template from the `--set` pairs, the derived slug,
/// and the kind name; the first uncovered placeholder is the error value.
fn fill_kind_template(
    template: &str,
    pairs: &[(String, String)],
    slug: Option<&str>,
    kind: &str,
) -> Result<String, String> {
    let mut rendered = template.to_string();
    for (key, value) in pairs {
        rendered = rendered.replace(&format!("{{{key}}}"), value);
    }
    if let Some(slug) = slug {
        rendered = rendered.replace("{slug}", slug);
    }
    rendered = rendered.replace("{kind}", kind);
    match leftover_placeholder(&rendered) {
        Some(placeholder) => Err(placeholder),
        None => Ok(rendered),
    }
}

/// The documented placeholder vocabulary, applied to any template source.
fn substitute(
    text: &str,
    id: &str,
    kind: &str,
    namespace: &str,
    title: &str,
    slug: &str,
) -> String {
    text.replace("{id}", id)
        .replace("{title}", title)
        .replace("{slug}", slug)
        .replace("{iri_slug}", &id.to_lowercase())
        .replace("{kind}", kind)
        .replace("{namespace}", namespace)
        .replace("{lifecycle}", lifecycle_for(kind))
}

/// The first braced lowercase identifier left after substitution — an
/// unknown placeholder. YAML literals such as `{}` never match.
fn leftover_placeholder(text: &str) -> Option<String> {
    regex::Regex::new(r"\{([a-z_]+)\}")
        .expect("static regex")
        .captures(text)
        .map(|c| c[1].to_string())
}

/// Mint the next ID from a configured id-pattern (ADR-0012): the pattern
/// must be generative — literal text around one `(?P<seq>\d{N})` group —
/// and the next sequence value is one past the highest among the IDs that
/// match the pattern. Anything else needs an explicit `--id`.
fn next_id_from_pattern(pattern: &str, docs: &[Document]) -> Result<String, String> {
    let regex = regex::Regex::new(pattern)
        .map_err(|err| format!("id-pattern is not a valid regex: {err}"))?;
    let seq_marker = regex::Regex::new(r"\(\?P<seq>\\d\{(\d+)\}\)").expect("static regex");
    let Some(found) = seq_marker.captures(pattern) else {
        return Err(format!(
            "id-pattern '{pattern}' has no (?P<seq>\\d{{N}}) group to count; pass --id"
        ));
    };
    let width: usize = found[1].parse().expect("digits");
    let span = found.get(0).expect("match");
    let (Some(prefix), Some(suffix)) = (
        literal_fragment(&pattern[..span.start()]),
        literal_fragment(&pattern[span.end()..]),
    ) else {
        return Err(format!(
            "id-pattern '{pattern}' is not generative (only literal text may surround the seq group); pass --id"
        ));
    };

    let mut highest = 0u64;
    for doc in docs {
        if let Some(id) = &doc.id
            && let Some(caps) = regex.captures(id)
            && let Some(seq) = caps.name("seq")
        {
            highest = highest.max(seq.as_str().parse().unwrap_or(0));
        }
    }
    let next = highest + 1;
    Ok(format!("{prefix}{next:0width$}{suffix}"))
}

/// The literal text of a pattern fragment, or None when it carries regex
/// machinery generation cannot fill.
fn literal_fragment(fragment: &str) -> Option<String> {
    let fragment = fragment.trim_start_matches('^').trim_end_matches('$');
    let mut out = String::new();
    let mut chars = fragment.chars();
    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                let escaped = chars.next()?;
                if escaped.is_ascii_alphanumeric() {
                    return None;
                }
                out.push(escaped);
            }
            '(' | ')' | '[' | ']' | '{' | '}' | '?' | '*' | '+' | '|' | '.' => return None,
            c => out.push(c),
        }
    }
    Some(out)
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
    /// Explicit repository-relative target directory (REQ-08-01-27-01);
    /// wins over the declared kind dir and the default placement.
    pub dir: Option<&'a str>,
    pub dry_run: bool,
    /// Raw `--set key=value` pairs (REQ-08-01-32-01); each fills the
    /// template's own `{key}` placeholder.
    pub sets: &'a [String],
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
    let mut pairs: Vec<(String, String)> = Vec::new();
    for raw in options.sets {
        match raw.split_once('=') {
            Some((key, value)) if !key.is_empty() => {
                pairs.push((key.to_string(), value.to_string()));
            }
            _ => {
                eprintln!("error: invalid --set '{raw}': expected key=value");
                return ExitCode::from(2);
            }
        }
    }
    let contract = crate::config::kind_contracts(Path::new("."))
        .into_iter()
        .find(|contract| contract.family == kind);
    let slug_opt = options.title.map(slugify);
    let id = match options.id {
        Some(explicit) => {
            if !valid_id(explicit) {
                eprintln!("error: invalid id '{explicit}': expected an ID slug ([A-Za-z0-9-])");
                return ExitCode::from(2);
            }
            if let Some(holder) = docs.iter().find(|doc| doc.id.as_deref() == Some(explicit)) {
                // arqix:implements REQ-08-01-39-01
                let diagnostic = Diagnostic::error(
                    "TPL-004",
                    format!("id {explicit} is already used by {}", holder.file),
                )
                .at(&holder.file);
                diag::emit(&[diagnostic], format);
                return ExitCode::from(1);
            }
            explicit.to_string()
        }
        // arqix:implements REQ-08-01-25-04
        None if contract.as_ref().is_some_and(|c| c.id_template.is_some()) => {
            let template = contract
                .as_ref()
                .and_then(|c| c.id_template.as_deref())
                .expect("guarded by the match arm");
            let minted = match fill_kind_template(template, &pairs, slug_opt.as_deref(), kind) {
                Ok(minted) => minted,
                Err(placeholder) => {
                    eprintln!(
                        "error: id-template '{template}' placeholder '{{{placeholder}}}' has no value (pass --set {placeholder}=... or --title for slug)"
                    );
                    return ExitCode::from(2);
                }
            };
            if !valid_id(&minted) {
                eprintln!("error: invalid id '{minted}': expected an ID slug ([A-Za-z0-9-])");
                return ExitCode::from(2);
            }
            if let Some(holder) = docs
                .iter()
                .find(|doc| doc.id.as_deref() == Some(minted.as_str()))
            {
                let diagnostic = Diagnostic::error(
                    "TPL-004",
                    format!("id {minted} is already used by {}", holder.file),
                )
                .at(&holder.file);
                diag::emit(&[diagnostic], format);
                return ExitCode::from(1);
            }
            minted
        }
        None => match crate::config::id_pattern_for_kind(Path::new("."), kind) {
            // arqix:implements REQ-01-01-18-01
            // The configured pattern mints the ID: the seq group tells
            // generation what to count (ADR-0012).
            Some(pattern) => match next_id_from_pattern(&pattern, &docs) {
                Ok(id) => id,
                Err(message) => {
                    eprintln!("error: {message}");
                    return ExitCode::from(2);
                }
            },
            None => {
                let counter = next_counter(&docs, &prefix) + 1;
                format!("{prefix}{counter:0width$}")
            }
        },
    };
    let title = match options.title {
        Some(title) => title.to_string(),
        None => format!("New {}", capitalise(kind)),
    };
    let slug = slug_opt.clone().unwrap_or_else(|| id.to_lowercase());

    let roots = crate::config::roots(Path::new("."));
    let root = roots.first().cloned().unwrap_or_else(|| "docs".to_string());
    // Placement precedence: the explicit --dir (REQ-08-01-27-01), then the
    // declared [kinds.<family>] dir (REQ-08-01-25-01, one source with
    // validation per ADR-0011), then the <first-root>/<kind>/ default.
    let dir = match options.dir {
        Some(explicit) => {
            let path = Path::new(explicit);
            if path.is_absolute()
                || path
                    .components()
                    .any(|c| matches!(c, std::path::Component::ParentDir))
            {
                // Containment (REQ-00-00-00-13): the repository is the only
                // write target, so escapes are usage errors, never writes.
                eprintln!(
                    "error: --dir must be a repository-relative path without '..': {explicit}"
                );
                return ExitCode::from(2);
            }
            PathBuf::from(explicit)
        }
        // arqix:implements REQ-08-01-25-05
        None if contract.as_ref().is_some_and(|c| c.dir_template.is_some()) => {
            let template = contract
                .as_ref()
                .and_then(|c| c.dir_template.as_deref())
                .expect("guarded by the match arm");
            let rendered = match fill_kind_template(template, &pairs, slug_opt.as_deref(), kind) {
                Ok(rendered) => rendered,
                Err(placeholder) => {
                    eprintln!(
                        "error: dir-template '{template}' placeholder '{{{placeholder}}}' has no value (pass --set {placeholder}=... or --title for slug)"
                    );
                    return ExitCode::from(2);
                }
            };
            let path = Path::new(&rendered);
            if path.is_absolute()
                || path
                    .components()
                    .any(|c| matches!(c, std::path::Component::ParentDir))
            {
                eprintln!(
                    "error: dir-template must render a repository-relative path without '..': {rendered}"
                );
                return ExitCode::from(2);
            }
            PathBuf::from(rendered)
        }
        None => contract
            .as_ref()
            .map(|contract| PathBuf::from(contract.dir.clone()))
            .unwrap_or_else(|| PathBuf::from(&root).join(kind)),
    };
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
        let content = match template(&id, kind, &namespace, &title, &slug, &pairs, format) {
            Ok(content) => content,
            Err(code) => return code,
        };
        if let Err(err) = std::fs::write(&path, content) {
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
    // arqix:implements REQ-01-01-20-02
    // The default template files: into the configured template directory,
    // or the package-local templates/ — never overwriting a shaped one.
    let (tpl_dir, configured) = template_dir();
    let tpl_dir = if configured {
        tpl_dir
    } else {
        package.join("templates")
    };
    if let Err(err) = std::fs::create_dir_all(&tpl_dir) {
        eprintln!("error: cannot create {}: {err}", tpl_dir.display());
        return ExitCode::from(2);
    }
    for kind in TEMPLATE_FAMILIES {
        let file = tpl_dir.join(format!("{kind}.tpl.md"));
        if file.exists() {
            continue;
        }
        let (_, _, namespace) = scheme(kind);
        if let Err(err) = std::fs::write(&file, default_template(kind, &namespace)) {
            eprintln!("error: cannot write {}: {err}", file.display());
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

    // arqix:implements REQ-08-01-24-01
    // arqix:implements REQ-08-01-24-02
    // The agent-instructions scaffold at the repository root; authored
    // instructions always win, so an existing file is never touched.
    let agents = Path::new("AGENTS.md");
    if !agents.exists()
        && let Err(err) = std::fs::write(agents, AGENTS_TEMPLATE)
    {
        eprintln!("error: cannot write AGENTS.md: {err}");
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
