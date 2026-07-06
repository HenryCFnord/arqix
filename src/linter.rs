//! Linter: contract checks over the corpus (REQ-01-01-04-*, REQ-00-00-00-10).
//! Reads through the store and parser; reports findings in the shared
//! diagnostics shape with file and line context. Never mutates documents.

use crate::OutputFormat;
use crate::diag::{self, Diagnostic};
use crate::parser::Document;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::process::ExitCode;

// arqix:implements REQ-01-01-04-01
// arqix:implements REQ-01-01-04-03
// arqix:implements REQ-01-01-04-04
// arqix:implements REQ-00-00-00-10
/// `arqix lint run`
pub fn run(format: OutputFormat) -> ExitCode {
    let docs = crate::store::documents();
    let mut diagnostics = Vec::new();
    check_duplicate_ids(&docs, &mut diagnostics);
    check_includes(&docs, &mut diagnostics);
    check_translations(&docs, &mut diagnostics);

    diagnostics.sort_by(|a, b| (&a.file, a.line, a.code).cmp(&(&b.file, b.line, b.code)));

    let code = diag::exit_code(&diagnostics);
    diag::emit(&diagnostics, format);
    if diagnostics.is_empty() && matches!(format, OutputFormat::Text) {
        println!("lint ok");
    }
    code
}

/// LNT-002: a document ID must be globally unique (REQ-01-01-04-03).
fn check_duplicate_ids(docs: &[Document], diags: &mut Vec<Diagnostic>) {
    let mut by_id: HashMap<&str, Vec<&Document>> = HashMap::new();
    for d in docs {
        if let Some(id) = &d.id {
            by_id.entry(id).or_default().push(d);
        }
    }
    let mut ids: Vec<&str> = by_id.keys().copied().collect();
    ids.sort();
    for id in ids {
        let group = &by_id[id];
        if group.len() > 1 {
            for d in group {
                diags.push(
                    Diagnostic::error(
                        "LNT-002",
                        format!("duplicate id {id} ({} documents share it)", group.len()),
                    )
                    .at_line(&d.file, d.id_line()),
                );
            }
        }
    }
}

/// LNT-001: every include directive must resolve to an existing file
/// (REQ-01-01-04-01).
fn check_includes(docs: &[Document], diags: &mut Vec<Diagnostic>) {
    for d in docs {
        let dir = Path::new(&d.file)
            .parent()
            .unwrap_or_else(|| Path::new("."));
        for (idx, line) in d.body.lines().enumerate() {
            if let Some(target) = include_target(line)
                && !dir.join(&target).exists()
            {
                diags.push(
                    Diagnostic::error(
                        "LNT-001",
                        format!("include target does not exist: {target}"),
                    )
                    .at_line(&d.file, d.body_offset + idx),
                );
            }
        }
    }
}

/// LNT-010: a declared translation source must exist (REQ-00-00-00-10). The
/// full i18n drift model (missing, unresolved, outdated) lands with the i18n
/// story; v1 checks the source link.
fn check_translations(docs: &[Document], diags: &mut Vec<Diagnostic>) {
    let ids: HashSet<&str> = docs.iter().filter_map(|d| d.id.as_deref()).collect();
    for d in docs {
        if let Some(source) = &d.translation_of
            && !source.is_empty()
            && !ids.contains(source.as_str())
        {
            diags.push(
                Diagnostic::error(
                    "LNT-010",
                    format!("translation-of points to unknown source: {source}"),
                )
                .at_line(&d.file, d.id_line()),
            );
        }
    }
}

/// Extract the path from a genuine `<!-- arqix:include PATH -->` directive.
/// The whole line must be the HTML comment with a single-token path, so
/// prose that merely mentions the directive is not matched. Shared with the
/// assembler, which expands the same directives.
pub(crate) fn include_target(line: &str) -> Option<String> {
    let inner = line
        .trim()
        .strip_prefix("<!--")?
        .strip_suffix("-->")?
        .trim();
    let path = inner.strip_prefix("arqix:include")?.trim();
    if path.is_empty() || path.split_whitespace().count() != 1 {
        return None;
    }
    Some(path.to_string())
}
