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
    check_references(&docs, &mut diagnostics);
    check_translations(&docs, &mut diagnostics);
    check_lifecycle_vocabulary(&docs, &mut diagnostics);
    check_done_claims(&docs, &mut diagnostics);

    diagnostics.sort_by(|a, b| (&a.file, a.line, a.code).cmp(&(&b.file, b.line, b.code)));

    diag::emit(&diagnostics, format);
    if diagnostics.is_empty() && matches!(format, OutputFormat::Text) {
        println!("lint ok");
    }
    diag::exit_code(&diagnostics)
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

/// LNT-003: an inline `<!-- arqix:references-artefact <iri> -->` body marker
/// must resolve to a known document IRI. Frontmatter triple targets are
/// guarded by the frontmatter checker (ONT-003); this extends the same
/// safety to the body markers the trace engine reads (ADR-0009).
fn check_references(docs: &[Document], diags: &mut Vec<Diagnostic>) {
    let known: HashSet<&str> = docs.iter().filter_map(|d| d.iri.as_deref()).collect();
    for d in docs {
        for (idx, line) in d.body.lines().enumerate() {
            if let Some(target) = crate::trace::md_reference_marker(line)
                && !known.contains(target.as_str())
            {
                diags.push(
                    Diagnostic::error(
                        "LNT-003",
                        format!("reference target does not resolve: {target}"),
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

/// The controlled lifecycle vocabularies per document nature (ADR-0010).
/// Requirements have no draft: the gate refutes it — nothing half-authored
/// can merge, so everything on main is fully authored and in force.
const STORY_VOCAB: [&str; 5] = ["draft", "specified", "in-implementation", "done", "retired"];
const REQUIREMENT_VOCAB: [&str; 2] = ["active", "retired"];
const PROSE_VOCAB: [&str; 3] = ["draft", "final", "retired"];

/// A document's declared `meta.lifecycle-status` and its file line, read
/// from the raw frontmatter (the value is contract-checked, not parsed).
fn lifecycle_status(doc: &Document) -> Option<(String, usize)> {
    for (idx, line) in doc.frontmatter.iter().enumerate() {
        if let Some(value) = line.trim().strip_prefix("lifecycle-status:") {
            let value = value.trim();
            if !value.is_empty() {
                // Frontmatter lines start at file line 2 (after the `---`).
                return Some((value.to_string(), idx + 2));
            }
        }
    }
    None
}

// arqix:implements REQ-03-01-09-02
/// LNT-004: a declared `lifecycle-status` must come from the controlled
/// vocabulary of the document's nature (ADR-0010). Documents without the
/// key are outside the lifecycle contract and stay unchecked here.
fn check_lifecycle_vocabulary(docs: &[Document], diags: &mut Vec<Diagnostic>) {
    for d in docs {
        let Some((value, line)) = lifecycle_status(d) else {
            continue;
        };
        let (nature, vocab): (&str, &[&str]) = match d.kind().as_str() {
            "user-story" => ("story", &STORY_VOCAB),
            "requirement" => ("requirement", &REQUIREMENT_VOCAB),
            _ => ("prose", &PROSE_VOCAB),
        };
        if !vocab.contains(&value.as_str()) {
            diags.push(
                Diagnostic::error(
                    "LNT-004",
                    format!(
                        "lifecycle-status '{value}' is not in the {nature} vocabulary ({})",
                        vocab.join(", ")
                    ),
                )
                .at_line(&d.file, line),
            );
        }
    }
}

// arqix:implements REQ-03-01-09-01
/// LNT-005: `done` is a claim the gate checks (ADR-0010) — every requirement
/// of a done story must be verified by an active test. The verified set
/// comes from the trace walk and is only computed when a done story exists.
fn check_done_claims(docs: &[Document], diags: &mut Vec<Diagnostic>) {
    let done: Vec<&Document> = docs
        .iter()
        .filter(|d| {
            d.kind() == "user-story"
                && lifecycle_status(d).is_some_and(|(value, _)| value == "done")
        })
        .collect();
    if done.is_empty() {
        return;
    }

    let verified = crate::trace::verified_requirement_ids();
    let id_by_iri: HashMap<&str, &str> = docs
        .iter()
        .filter_map(|d| Some((d.iri.as_deref()?, d.id.as_deref()?)))
        .collect();
    for story in done {
        let story_id = story.id.as_deref().unwrap_or("?");
        for triple in &story.triples {
            if !triple.predicate.ends_with("has-requirement") {
                continue;
            }
            let Some(req_id) = id_by_iri.get(triple.object.as_str()) else {
                continue; // an unresolved target is ONT-003 territory
            };
            if !verified.iter().any(|v| v == req_id) {
                diags.push(
                    Diagnostic::error(
                        "LNT-005",
                        format!(
                            "done claim violated: {story_id} is done but {req_id} has no active verifying test"
                        ),
                    )
                    .at_line(&story.file, triple.line),
                );
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    // arqix:no-requirement
    #[test]
    fn unresolved_reference_marker_is_lnt_003() {
        let target = parse(
            "docs/adr.md",
            "---\nid: ADR-0005\niri: arqix:adrs/adr-0005\n---\nbody\n",
        );
        let good = parse(
            "docs/u1.md",
            "---\nid: unit-a\niri: arqix:units/unit-a\n---\n\n<!-- arqix:references-artefact arqix:adrs/adr-0005 -->\n",
        );
        let bad = parse(
            "docs/u2.md",
            "---\nid: unit-b\niri: arqix:units/unit-b\n---\n\n<!-- arqix:references-artefact arqix:adrs/adr-9999 -->\n",
        );
        let docs = vec![target, good, bad];
        let mut diags = Vec::new();
        check_references(&docs, &mut diags);
        assert_eq!(diags.len(), 1, "only the unknown target is flagged");
        assert_eq!(diags[0].code, "LNT-003");
        assert!(diags[0].message.contains("adr-9999"));
    }
}
