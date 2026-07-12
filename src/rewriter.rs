//! Formatter & Finaliser: the only component that mutates existing source
//! documents, and only mechanically (ADR-0004). `fmt` canonicalises
//! frontmatter key order over a lossless line-based CST — the body is never
//! touched (REQ-01-01-03-*); `finalise` sets `updated` to an injected date
//! (REQ-01-01-06-*, the clock is never read ambiently).

use crate::OutputFormat;
use crate::diag::{self, Diagnostic};
use std::fs;
use std::process::ExitCode;

/// Canonical top-level frontmatter key orders — the same per-family orders
/// the frontmatter checker enforces as a subsequence (check_frontmatter.py
/// FAMILIES), so already-conforming documents are left byte-identical.
/// Every architecture/documentation family shares one order; the three
/// ontology families interleave label/rdfs/owl differently.
const DOC_KEYS: [&str; 9] = [
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
const ONT_CLASS_KEYS: [&str; 10] = [
    "id",
    "label",
    "iri",
    "rdf",
    "rdfs",
    "triples",
    "properties",
    "external-references",
    "owl",
    "meta",
];
const ONT_PROPERTY_KEYS: [&str; 10] = [
    "id",
    "label",
    "iri",
    "rdf",
    "rdfs",
    "owl",
    "triples",
    "properties",
    "external-references",
    "meta",
];
const ONT_INDIVIDUAL_KEYS: [&str; 8] = [
    "id",
    "label",
    "iri",
    "rdf",
    "triples",
    "properties",
    "external-references",
    "meta",
];

// arqix:implements REQ-01-01-19-01
/// The canonical key order for a document: the configured family contract
/// whose directory contains the file (longest match — US-01-01-19), or the
/// built-in defaults by the same directory mapping the frontmatter checker
/// uses to pick the family.
fn key_order(file: &str) -> Vec<String> {
    let posix = file.replace('\\', "/");
    for contract in crate::config::kind_contracts(std::path::Path::new(".")) {
        if posix.starts_with(&format!("{}/", contract.dir)) {
            return contract.key_order;
        }
    }
    let builtin: &[&str] = if file.contains("docs/ontology/classes/") {
        &ONT_CLASS_KEYS
    } else if file.contains("docs/ontology/properties/") {
        &ONT_PROPERTY_KEYS
    } else if file.contains("docs/ontology/individuals/") {
        &ONT_INDIVIDUAL_KEYS
    } else {
        &DOC_KEYS
    };
    builtin.iter().map(|k| k.to_string()).collect()
}

/// Split leading frontmatter into the exact opening-fence bytes (including
/// their line terminator), its raw lines, and the verbatim remainder (the
/// closing `---` onward, byte-exact). Returns None when there is no
/// frontmatter block. The opening fence is preserved rather than
/// reconstructed so a CRLF (`---\r\n`) or trailing-whitespace fence survives
/// the round trip — losslessness must not depend on the line ending.
fn split_frontmatter(text: &str) -> Option<(&str, Vec<String>, &str)> {
    let first_nl = text.find('\n')?;
    if text[..first_nl].trim() != "---" {
        return None;
    }
    let open = &text[..=first_nl];
    let mut offset = first_nl + 1;
    let mut fm_lines = Vec::new();
    for line in text[first_nl + 1..].split_inclusive('\n') {
        let content = line.strip_suffix('\n').unwrap_or(line);
        if content.trim() == "---" {
            return Some((open, fm_lines, &text[offset..]));
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

fn is_known(block: &(Option<String>, Vec<String>), order: &[String]) -> bool {
    block
        .0
        .as_deref()
        .is_some_and(|k| order.iter().any(|o| o == k))
}

fn canonical_index(block: &(Option<String>, Vec<String>), order: &[String]) -> usize {
    block
        .0
        .as_deref()
        .and_then(|k| order.iter().position(|c| *c == k))
        .unwrap_or(usize::MAX)
}

/// Format the document text, or None when it has no frontmatter to canonicalise.
///
/// Only the keys known to the document's family order are reordered, and
/// only into their own original slots, so their sequence follows the family
/// order while every unknown key stays exactly where it is. This matches
/// the frontmatter checker's subsequence rule, so an already-conforming
/// document is left byte-identical.
fn format_text(file: &str, text: &str) -> Option<String> {
    let order = key_order(file);
    let (open, fm_lines, rest) = split_frontmatter(text)?;
    let blocks = group_blocks(&fm_lines);

    let mut known: Vec<&(Option<String>, Vec<String>)> =
        blocks.iter().filter(|b| is_known(b, &order)).collect();
    known.sort_by_key(|b| canonical_index(b, &order));
    let mut known = known.into_iter();

    let mut out = String::from(open);
    for block in &blocks {
        let emitted = if is_known(block, &order) {
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
    let mut write_failed = false;
    for doc in crate::store::documents() {
        let text = match fs::read_to_string(&doc.file) {
            Ok(text) => text,
            Err(_) => continue,
        };
        if let Some(formatted) = format_text(&doc.file, &text)
            && formatted != text
        {
            changed.push(doc.file.clone());
            if !check && let Err(err) = fs::write(&doc.file, formatted) {
                // A failed source mutation must never look like success; a
                // system error is exit 2 (the codebase's I/O convention),
                // distinct from the exit-1 findings channel.
                eprintln!("error: cannot write {}: {err}", doc.file);
                write_failed = true;
            }
        }
    }

    if check {
        let diagnostics: Vec<Diagnostic> = changed
            .iter()
            .map(|f| Diagnostic::error("FMT-001", "frontmatter is not canonically formatted").at(f))
            .collect();
        diag::emit(&diagnostics, format);
        return diag::exit_code(&diagnostics);
    }

    if write_failed {
        return ExitCode::from(2);
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
    let (open, fm_lines, rest) = match split_frontmatter(text) {
        Some(parts) => parts,
        None => return (FinaliseResult::Unsupported, None),
    };

    let mut found = false;
    let mut changed = false;
    let mut in_meta = false;
    let mut new_fm = Vec::with_capacity(fm_lines.len());
    for line in &fm_lines {
        // A CRLF line keeps its `\r` in the stored content; preserve it so an
        // already-current doc is left byte-identical.
        let (body, eol) = match line.strip_suffix('\r') {
            Some(body) => (body, "\r"),
            None => (line.as_str(), ""),
        };
        if is_top_level(body) {
            in_meta = body.split(':').next() == Some("meta");
            new_fm.push(line.clone());
            continue;
        }
        // Only the lifecycle field meta.updated is finalised; an `updated:`
        // key under any other section is document data.
        if in_meta && body.trim_start().starts_with("updated:") {
            found = true;
            let indent = &body[..body.len() - body.trim_start().len()];
            let rewritten = format!("{indent}updated: {date}{eol}");
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

    let mut out = String::from(open);
    for line in &new_fm {
        out.push_str(line);
        out.push('\n');
    }
    out.push_str(rest);
    (FinaliseResult::Updated, Some(out))
}

/// A real ISO-8601 calendar date in `YYYY-MM-DD` — shape and calendar both
/// (REQ-01-01-06-01), so finalise can never stamp a value its own
/// frontmatter rules reject.
fn valid_iso_date(s: &str) -> bool {
    let bytes = s.as_bytes();
    if bytes.len() != 10 || bytes[4] != b'-' || bytes[7] != b'-' {
        return false;
    }
    let digits = |r: std::ops::Range<usize>| -> Option<u32> {
        s.get(r)
            .filter(|p| p.bytes().all(|b| b.is_ascii_digit()))?
            .parse()
            .ok()
    };
    let (Some(year), Some(month), Some(day)) = (digits(0..4), digits(5..7), digits(8..10)) else {
        return false;
    };
    let leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
    let days = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap => 29,
        2 => 28,
        _ => return false,
    };
    (1..=days).contains(&day)
}

// arqix:implements REQ-01-01-06-01
// arqix:implements REQ-01-01-06-02
// arqix:implements REQ-01-01-06-03
// arqix:implements REQ-00-00-00-08
/// `arqix finalise --date <iso>`
pub fn finalise(date: &str, format: OutputFormat) -> ExitCode {
    if !valid_iso_date(date) {
        eprintln!(
            "error: invalid --date '{date}': expected an ISO-8601 calendar date (YYYY-MM-DD)"
        );
        return ExitCode::from(2);
    }
    let mut diagnostics = Vec::new();
    let mut write_failed = false;
    for doc in crate::store::documents() {
        let text = match fs::read_to_string(&doc.file) {
            Ok(text) => text,
            Err(_) => continue,
        };
        match set_updated(&text, date) {
            (FinaliseResult::Updated, Some(out)) => {
                if let Err(err) = fs::write(&doc.file, out) {
                    eprintln!("error: cannot write {}: {err}", doc.file);
                    write_failed = true;
                }
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
    diag::emit(&diagnostics, format);
    // A failed source mutation is a system error (exit 2), and takes
    // precedence over the exit-1 findings channel.
    if write_failed {
        return ExitCode::from(2);
    }
    diag::exit_code(&diagnostics)
}

#[cfg(test)]
mod tests {
    use super::*;

    // arqix:no-requirement
    #[test]
    fn already_ordered_document_is_byte_identical() {
        let text = "---\nid: X\ntitle: T\nmeta:\n  updated: 2026-07-04\n---\n\n## Body\n";
        assert_eq!(format_text("docs/x.md", text).as_deref(), Some(text));
    }

    // arqix:no-requirement
    #[test]
    fn foreign_family_keys_are_left_in_place() {
        // Ontology docs interleave label/rdfs/owl among the known keys; those
        // must not be moved (regression: a naive full sort mangled them).
        let text = "---\nid: p\nlabel: prop\niri: arqix:properties/p\nrdf:\n  type:\n    - rdf:Property\nrdfs:\n  domain:\n    - arqix:classes/artefact\nowl:\n  inverse-of: arqix:properties/q\ntriples: []\nmeta:\n  updated: 2026-03-27\n---\nbody\n";
        assert_eq!(format_text("docs/x.md", text).as_deref(), Some(text));
    }

    // arqix:no-requirement
    #[test]
    fn out_of_order_known_keys_are_canonicalised_and_idempotent() {
        let text = "---\nid: X\nmeta:\n  updated: 2026-07-04\ntitle: T\n---\nbody\n";
        let once = format_text("docs/x.md", text).unwrap();
        assert!(once.find("title:").unwrap() < once.find("meta:").unwrap());
        assert_eq!(
            format_text("docs/x.md", &once).as_deref(),
            Some(once.as_str())
        );
    }

    // arqix:no-requirement
    #[test]
    fn crlf_document_round_trips_byte_identically() {
        // Losslessness must not depend on the line ending: a CRLF,
        // already-ordered document is left byte-for-byte identical (the
        // opening fence must not be normalised to LF).
        let text =
            "---\r\nid: X\r\ntitle: T\r\nmeta:\r\n  updated: 2026-07-04\r\n---\r\n\r\n## Body\r\n";
        assert_eq!(format_text("docs/x.md", text).as_deref(), Some(text));
    }

    // arqix:no-requirement
    #[test]
    fn finalise_preserves_crlf_line_endings() {
        let text = "---\r\nid: X\r\nmeta:\r\n  updated: 2026-07-04\r\n---\r\nbody\r\n";
        // An already-current CRLF doc is untouched, not rewritten to LF.
        assert!(matches!(
            set_updated(text, "2026-07-04").0,
            FinaliseResult::AlreadyCurrent
        ));
        // A stale CRLF doc is updated while keeping its CRLF endings.
        match set_updated(text, "2027-01-31") {
            (FinaliseResult::Updated, Some(out)) => {
                assert!(out.contains("updated: 2027-01-31\r\n"));
                assert!(out.starts_with("---\r\n"));
                assert!(out.ends_with("body\r\n"));
            }
            _ => panic!("expected an update"),
        }
    }

    // arqix:no-requirement
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
