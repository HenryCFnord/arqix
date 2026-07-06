//! Formatter & Finaliser: the only component that mutates existing source
//! documents, and only mechanically (ADR-0004). `fmt` canonicalises
//! frontmatter key order over a lossless line-based CST — the body is never
//! touched (REQ-01-01-03-*); `finalise` sets `updated` to an injected date
//! (REQ-01-01-06-*, the clock is never read ambiently).

use crate::OutputFormat;
use crate::diag::{self, Diagnostic};
use std::fs;
use std::process::ExitCode;

/// Canonical top-level frontmatter key order — the same order the frontmatter
/// checker enforces as a subsequence, so already-conforming documents are
/// left byte-identical.
const CANONICAL_KEYS: [&str; 9] = [
    "id",
    "title",
    "slug",
    "iri",
    "rdf",
    "triples",
    "properties",
    "external-references",
    "meta",
];

/// Split leading frontmatter into its raw lines and the verbatim remainder
/// (the closing `---` onward, byte-exact). Returns None when there is no
/// frontmatter block.
fn split_frontmatter(text: &str) -> Option<(Vec<String>, &str)> {
    let first_nl = text.find('\n')?;
    if text[..first_nl].trim() != "---" {
        return None;
    }
    let mut offset = first_nl + 1;
    let mut fm_lines = Vec::new();
    for line in text[first_nl + 1..].split_inclusive('\n') {
        let content = line.strip_suffix('\n').unwrap_or(line);
        if content.trim() == "---" {
            return Some((fm_lines, &text[offset..]));
        }
        fm_lines.push(content.to_string());
        offset += line.len();
    }
    None
}

fn is_top_level(line: &str) -> bool {
    !line.is_empty() && !line.starts_with(char::is_whitespace) && !line.starts_with('-')
}

/// Group frontmatter lines into blocks keyed by their top-level key; blank and
/// indented lines attach to the current block (so trailing blanks travel with
/// their key when reordered).
fn group_blocks(fm: &[String]) -> Vec<(Option<String>, Vec<String>)> {
    let mut blocks: Vec<(Option<String>, Vec<String>)> = Vec::new();
    let mut current: Option<(Option<String>, Vec<String>)> = None;
    for line in fm {
        if is_top_level(line) {
            if let Some(block) = current.take() {
                blocks.push(block);
            }
            let key = line.split(':').next().unwrap_or("").to_string();
            current = Some((Some(key), vec![line.clone()]));
        } else if let Some((_, lines)) = &mut current {
            lines.push(line.clone());
        } else {
            current = Some((None, vec![line.clone()]));
        }
    }
    if let Some(block) = current {
        blocks.push(block);
    }
    blocks
}

fn is_known(block: &(Option<String>, Vec<String>)) -> bool {
    block
        .0
        .as_deref()
        .is_some_and(|k| CANONICAL_KEYS.contains(&k))
}

fn canonical_index(block: &(Option<String>, Vec<String>)) -> usize {
    block
        .0
        .as_deref()
        .and_then(|k| CANONICAL_KEYS.iter().position(|c| *c == k))
        .unwrap_or(usize::MAX)
}

/// Format the document text, or None when it has no frontmatter to canonicalise.
///
/// Only the known top-level keys are reordered, and only into their own
/// original slots, so their sequence follows CANONICAL_KEYS while every other
/// key (a different family's vocabulary such as `label`/`rdfs`/`owl`) stays
/// exactly where it is. This matches the frontmatter checker's subsequence
/// rule, so an already-conforming document is left byte-identical.
fn format_text(text: &str) -> Option<String> {
    let (fm_lines, rest) = split_frontmatter(text)?;
    let blocks = group_blocks(&fm_lines);

    let mut known: Vec<&(Option<String>, Vec<String>)> =
        blocks.iter().filter(|b| is_known(b)).collect();
    known.sort_by_key(|b| canonical_index(b));
    let mut known = known.into_iter();

    let mut out = String::from("---\n");
    for block in &blocks {
        let emitted = if is_known(block) {
            known.next().expect("known count is stable")
        } else {
            block
        };
        for line in &emitted.1 {
            out.push_str(line);
            out.push('\n');
        }
    }
    out.push_str(rest);
    Some(out)
}

// arqix:implements REQ-01-01-03-01
// arqix:implements REQ-01-01-03-02
// arqix:implements REQ-01-01-03-03
/// `arqix fmt [--check]`
pub fn fmt(check: bool, format: OutputFormat) -> ExitCode {
    let mut changed = Vec::new();
    for doc in crate::store::documents() {
        let text = match fs::read_to_string(&doc.file) {
            Ok(text) => text,
            Err(_) => continue,
        };
        if let Some(formatted) = format_text(&text)
            && formatted != text
        {
            changed.push(doc.file.clone());
            if !check {
                let _ = fs::write(&doc.file, formatted);
            }
        }
    }

    if check {
        let diagnostics: Vec<Diagnostic> = changed
            .iter()
            .map(|f| Diagnostic::error("FMT-001", "frontmatter is not canonically formatted").at(f))
            .collect();
        let code = diag::exit_code(&diagnostics);
        diag::emit(&diagnostics, format);
        return code;
    }

    if matches!(format, OutputFormat::Text) {
        println!("formatted {} document(s)", changed.len());
    }
    ExitCode::SUCCESS
}

enum FinaliseResult {
    Updated,
    AlreadyCurrent,
    Unsupported,
}

/// Set `updated` to `date` over the CST, or classify why it can't.
fn set_updated(text: &str, date: &str) -> (FinaliseResult, Option<String>) {
    let (fm_lines, rest) = match split_frontmatter(text) {
        Some(parts) => parts,
        None => return (FinaliseResult::Unsupported, None),
    };

    let mut found = false;
    let mut changed = false;
    let mut new_fm = Vec::with_capacity(fm_lines.len());
    for line in &fm_lines {
        if line.trim_start().starts_with("updated:") {
            found = true;
            let indent = &line[..line.len() - line.trim_start().len()];
            let rewritten = format!("{indent}updated: {date}");
            changed |= rewritten != *line;
            new_fm.push(rewritten);
        } else {
            new_fm.push(line.clone());
        }
    }

    if !found {
        return (FinaliseResult::Unsupported, None);
    }
    if !changed {
        return (FinaliseResult::AlreadyCurrent, None);
    }

    let mut out = String::from("---\n");
    for line in &new_fm {
        out.push_str(line);
        out.push('\n');
    }
    out.push_str(rest);
    (FinaliseResult::Updated, Some(out))
}

// arqix:implements REQ-01-01-06-01
// arqix:implements REQ-01-01-06-02
// arqix:implements REQ-01-01-06-03
// arqix:implements REQ-00-00-00-08
/// `arqix finalise --date <iso>`
pub fn finalise(date: &str, format: OutputFormat) -> ExitCode {
    let mut diagnostics = Vec::new();
    for doc in crate::store::documents() {
        let text = match fs::read_to_string(&doc.file) {
            Ok(text) => text,
            Err(_) => continue,
        };
        match set_updated(&text, date) {
            (FinaliseResult::Updated, Some(out)) => {
                let _ = fs::write(&doc.file, out);
            }
            (FinaliseResult::AlreadyCurrent, _) | (FinaliseResult::Updated, None) => {}
            (FinaliseResult::Unsupported, _) => diagnostics.push(
                Diagnostic::error(
                    "FIN-001",
                    "unsupported frontmatter: no updatable metadata found",
                )
                .at(&doc.file),
            ),
        }
    }

    diagnostics.sort_by(|a, b| a.file.cmp(&b.file));
    let code = diag::exit_code(&diagnostics);
    diag::emit(&diagnostics, format);
    code
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn already_ordered_document_is_byte_identical() {
        let text = "---\nid: X\ntitle: T\nmeta:\n  updated: 2026-07-04\n---\n\n## Body\n";
        assert_eq!(format_text(text).as_deref(), Some(text));
    }

    #[test]
    fn foreign_family_keys_are_left_in_place() {
        // Ontology docs interleave label/rdfs/owl among the known keys; those
        // must not be moved (regression: a naive full sort mangled them).
        let text = "---\nid: p\nlabel: prop\niri: arqix:properties/p\nrdf:\n  type:\n    - rdf:Property\nrdfs:\n  domain:\n    - arqix:classes/artefact\nowl:\n  inverse-of: arqix:properties/q\ntriples: []\nmeta:\n  updated: 2026-03-27\n---\nbody\n";
        assert_eq!(format_text(text).as_deref(), Some(text));
    }

    #[test]
    fn out_of_order_known_keys_are_canonicalised_and_idempotent() {
        let text = "---\nid: X\nmeta:\n  updated: 2026-07-04\ntitle: T\n---\nbody\n";
        let once = format_text(text).unwrap();
        assert!(once.find("title:").unwrap() < once.find("meta:").unwrap());
        assert_eq!(format_text(&once).as_deref(), Some(once.as_str()));
    }

    #[test]
    fn finalise_sets_and_leaves_current_untouched() {
        let text = "---\nid: X\nmeta:\n  updated: 2026-07-04\n---\nbody\n";
        match set_updated(text, "2027-01-31") {
            (FinaliseResult::Updated, Some(out)) => assert!(out.contains("updated: 2027-01-31")),
            _ => panic!("expected an update"),
        }
        assert!(matches!(
            set_updated(text, "2026-07-04").0,
            FinaliseResult::AlreadyCurrent
        ));
        assert!(matches!(
            set_updated("no frontmatter\n", "2027-01-31").0,
            FinaliseResult::Unsupported
        ));
    }
}
