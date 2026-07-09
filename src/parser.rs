//! Document Parser: the shared reading layer (arc42 chapter 5). Parses a
//! Markdown document's YAML frontmatter into the semantic model consumed by
//! the store, the linter, the trace engine, and (via the raw frontmatter
//! lines) the mechanical rewriter. This is the Rust port of the Python
//! oracle's `parse_document`; the two must agree on the corpus.

/// Requirement kind subclasses; their coarse document type is "requirement".
const REQ_KIND_CLASSES: [&str; 3] = [
    "functional-requirement",
    "quality-requirement",
    "constraint",
];

// Fields consumed by the trace engine slice (frontmatter-triple edges).
#[allow(dead_code)]
pub struct Triple {
    pub predicate: String,
    pub object: String,
    pub line: usize,
}

pub struct Document {
    pub id: Option<String>,
    pub title: Option<String>,
    pub iri: Option<String>,
    pub lang: Option<String>,
    pub translation_of: Option<String>,
    pub classes: Vec<String>,
    pub triples: Vec<Triple>,
    pub has_frontmatter: bool,
    /// Raw frontmatter lines between the `---` markers (for the rewriter).
    pub frontmatter: Vec<String>,
    pub body: String,
    /// 1-based file line of the first body line (after the closing `---`).
    pub body_offset: usize,
    pub file: String,
}

impl Document {
    fn empty(file: &str) -> Self {
        Document {
            id: None,
            title: None,
            iri: None,
            lang: None,
            translation_of: None,
            classes: Vec::new(),
            triples: Vec::new(),
            has_frontmatter: false,
            frontmatter: Vec::new(),
            body: String::new(),
            body_offset: 1,
            file: file.to_string(),
        }
    }

    /// The 1-based file line of the `id:` frontmatter entry, or 1.
    pub fn id_line(&self) -> usize {
        self.frontmatter
            .iter()
            .position(|l| l.trim_start().starts_with("id:"))
            .map(|i| i + 2)
            .unwrap_or(1)
    }

    /// The coarse document type (matches the oracle's `document_type`):
    /// requirement subclasses collapse to "requirement"; otherwise the first
    /// declared class; otherwise "requirement" by ID pattern, else "document".
    pub fn kind(&self) -> String {
        if let Some(first) = self.classes.first() {
            if REQ_KIND_CLASSES.contains(&first.as_str()) {
                return "requirement".to_string();
            }
            return first.clone();
        }
        match &self.id {
            Some(id) if is_requirement_id(id) => "requirement".to_string(),
            _ => "document".to_string(),
        }
    }
}

/// `REQ-XX-YY-ZZ-NN` shape check (four two-digit groups).
pub fn is_requirement_id(id: &str) -> bool {
    let rest = match id.strip_prefix("REQ-") {
        Some(r) => r,
        None => return false,
    };
    let groups: Vec<&str> = rest.split('-').collect();
    groups.len() == 4
        && groups
            .iter()
            .all(|g| g.len() == 2 && g.chars().all(|c| c.is_ascii_digit()))
}

/// Python `\w`: alphanumeric (Unicode) or underscore.
fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// `key:` followed by `\s*` — the shared prefix of the oracle's scalar
/// regexes. Returns the rest of the line after the optional whitespace.
fn scalar_rest<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    Some(line.strip_prefix(key)?.strip_prefix(':')?.trim_start())
}

/// `id` per FRONTMATTER_ID_RE `^id:\s*["']?([\w][\w-]*)["']?\s*$`: one
/// optional quote on each side (independently, so mismatched pairs pass)
/// around exactly one `[\w][\w-]*` token.
fn id_value(line: &str) -> Option<String> {
    let rest = scalar_rest(line, "id")?.trim_end();
    let rest = rest.strip_prefix(['"', '\'']).unwrap_or(rest);
    let rest = rest.strip_suffix(['"', '\'']).unwrap_or(rest);
    let mut chars = rest.chars();
    if !chars.next().is_some_and(is_word_char) {
        return None;
    }
    chars
        .all(|c| is_word_char(c) || c == '-')
        .then(|| rest.to_string())
}

/// `iri` per FRONTMATTER_IRI_RE `^iri:\s*(\S+)\s*$`: the raw single token,
/// quotes and all.
fn iri_value(line: &str) -> Option<String> {
    single_token(scalar_rest(line, "iri")?)
}

/// `title` per FRONTMATTER_TITLE_RE `^title:\s*["']?(.+?)["']?\s*$`: strip
/// at most one quote per side (mismatched pairs allowed); empty is no title.
fn title_value(line: &str) -> Option<String> {
    let rest = scalar_rest(line, "title")?.trim_end();
    if rest.is_empty() {
        return None;
    }
    // The group needs at least one character, so a lone quote backtracks
    // into the group instead of being consumed as the opening quote.
    let mid = match rest.strip_prefix(['"', '\'']) {
        Some(m) if !m.is_empty() => m,
        _ => rest,
    };
    let title = match mid.strip_suffix(['"', '\'']) {
        Some(t) if !t.is_empty() => t,
        _ => mid,
    };
    Some(title.to_string())
}

/// The section key per TOP_KEY_RE `^([\w.-]+):`, or None. Only a matching
/// line changes the current section (the oracle keeps it otherwise).
fn top_key(line: &str) -> Option<String> {
    let key: String = line
        .chars()
        .take_while(|&c| is_word_char(c) || c == '.' || c == '-')
        .collect();
    (!key.is_empty() && line[key.len()..].starts_with(':')).then_some(key)
}

/// `- <ws> arqix:classes/<CLS>` per CLASS_ITEM_RE, the class token placed
/// directly after the prefix.
fn class_item(line: &str) -> Option<String> {
    let rest = line.strip_prefix('-')?;
    if !rest.starts_with(char::is_whitespace) {
        return None;
    }
    let rest = rest.trim_start().strip_prefix("arqix:classes/")?;
    if rest.starts_with(char::is_whitespace) {
        return None;
    }
    single_token(rest)
}

/// The single non-whitespace token of `s`, or None if there is not exactly
/// one (mirrors the oracle's trailing `(\S+)\s*$`).
fn single_token(s: &str) -> Option<String> {
    let mut it = s.split_whitespace();
    let token = it.next()?;
    if it.next().is_some() {
        return None;
    }
    Some(token.to_string())
}

/// `- <ws> predicate: <ws> arqix:properties/<PRED>` with the oracle's
/// whitespace tolerance (TRIPLE_PRED_RE). Returns the bare predicate name.
fn triple_predicate(line: &str) -> Option<String> {
    let rest = line.strip_prefix('-')?;
    if !rest.starts_with(char::is_whitespace) {
        return None; // `\s+` requires at least one space after the dash
    }
    let rest = rest
        .trim_start()
        .strip_prefix("predicate:")?
        .trim_start()
        .strip_prefix("arqix:properties/")?;
    if rest.starts_with(char::is_whitespace) {
        return None; // `(\S+)` sits directly after the prefix
    }
    single_token(rest)
}

/// `object: <ws> arqix:<OBJ>` (TRIPLE_OBJ_INLINE_RE). Returns the full IRI.
fn triple_object_inline(line: &str) -> Option<String> {
    let rest = line.strip_prefix("object:")?.trim_start();
    single_token(rest).filter(|token| token.starts_with("arqix:"))
}

/// `- <ws> arqix:<OBJ>` (TRIPLE_OBJ_ITEM_RE). Returns the full IRI.
fn triple_object_item(line: &str) -> Option<String> {
    let rest = line.strip_prefix('-')?;
    if !rest.starts_with(char::is_whitespace) {
        return None;
    }
    single_token(rest.trim_start()).filter(|token| token.starts_with("arqix:"))
}

/// Extract a scalar frontmatter value: `key: value`, quotes stripped. Used
/// for the `meta` fields the oracle does not model (lang, translation-of).
fn scalar(line: &str, key: &str) -> Option<String> {
    let rest = line.strip_prefix(key)?.strip_prefix(':')?;
    Some(unquote(rest.trim()))
}

fn unquote(value: &str) -> String {
    let bytes = value.as_bytes();
    if value.len() >= 2
        && ((bytes[0] == b'"' && bytes[value.len() - 1] == b'"')
            || (bytes[0] == b'\'' && bytes[value.len() - 1] == b'\''))
    {
        value[1..value.len() - 1].to_string()
    } else {
        value.to_string()
    }
}

/// Split like Python `str.splitlines` — the oracle reads every corpus file
/// with it, so the engine must break lines on the same boundary set (form
/// feed, NEL, and the Unicode line/paragraph separators included).
pub(crate) fn py_splitlines(text: &str) -> Vec<&str> {
    const BOUNDARIES: [char; 10] = [
        '\n', '\r', '\x0b', '\x0c', '\x1c', '\x1d', '\x1e', '\u{85}', '\u{2028}', '\u{2029}',
    ];
    let mut lines = Vec::new();
    let mut start = 0;
    let mut chars = text.char_indices().peekable();
    while let Some((i, c)) = chars.next() {
        if !BOUNDARIES.contains(&c) {
            continue;
        }
        lines.push(&text[start..i]);
        start = i + c.len_utf8();
        if c == '\r' && chars.peek().is_some_and(|&(_, next)| next == '\n') {
            chars.next();
            start += 1;
        }
    }
    if start < text.len() {
        lines.push(&text[start..]);
    }
    lines
}

pub fn parse(file: &str, text: &str) -> Document {
    let mut doc = Document::empty(file);
    let lines: Vec<&str> = py_splitlines(text);

    if lines.first().map(|l| l.trim()) != Some("---") {
        doc.body = text.to_string();
        return doc;
    }
    let end = match lines
        .iter()
        .enumerate()
        .skip(1)
        .find(|(_, l)| l.trim() == "---")
    {
        Some((i, _)) => i,
        None => {
            doc.body = text.to_string();
            return doc;
        }
    };

    doc.has_frontmatter = true;
    doc.frontmatter = lines[1..end].iter().map(|s| s.to_string()).collect();
    doc.body = lines[end + 1..].join("\n");
    doc.body_offset = end + 2; // file line after the closing "---"

    let mut section = String::new();
    let mut predicate: Option<String> = None;

    for (idx, line) in lines[1..end].iter().enumerate() {
        let file_line = idx + 2; // line 1 is the opening "---"
        let stripped = line.trim();

        // The oracle's top-level branch takes every non-indented line —
        // zero-indent list items included — and only a TOP_KEY_RE match
        // changes the section.
        if !line.is_empty() && !line.starts_with(char::is_whitespace) {
            if let Some(key) = top_key(line) {
                section = key;
                predicate = None;
            }
            if let Some(v) = id_value(stripped) {
                doc.id = Some(v);
            } else if let Some(v) = iri_value(stripped) {
                doc.iri = Some(v);
            } else if let Some(v) = title_value(stripped) {
                doc.title = Some(v);
            }
            continue;
        }

        match section.as_str() {
            "rdf" => {
                if let Some(cls) = class_item(stripped) {
                    doc.classes.push(cls);
                }
            }
            "triples" => {
                if let Some(pred) = triple_predicate(stripped) {
                    predicate = Some(pred);
                } else if let Some(pred) = predicate.clone() {
                    let object =
                        triple_object_inline(stripped).or_else(|| triple_object_item(stripped));
                    if let Some(obj) = object {
                        doc.triples.push(Triple {
                            predicate: pred,
                            object: obj,
                            line: file_line,
                        });
                    }
                }
            }
            "meta" => {
                if let Some(v) = scalar(stripped, "lang") {
                    doc.lang = Some(v);
                } else if let Some(v) = scalar(stripped, "translation-of") {
                    doc.translation_of = Some(v);
                }
            }
            _ => {}
        }
    }

    doc
}

#[cfg(test)]
mod tests {
    use super::*;

    const DOC: &str = "---\nid: REQ-01-01-08-01\ntitle: Example\niri: arqix:requirements/req-01-01-08-01\nrdf:\n  type:\n    - arqix:classes/functional-requirement\ntriples:\n  - predicate: arqix:properties/derived-from\n    object:\n      - arqix:user-stories/us-01-01-08\nmeta:\n  lang: en\n---\n## Example\n\nBody text.\n";

    // arqix:no-requirement
    #[test]
    fn parses_scalars_classes_and_triples() {
        let d = parse("r.md", DOC);
        assert_eq!(d.id.as_deref(), Some("REQ-01-01-08-01"));
        assert_eq!(d.title.as_deref(), Some("Example"));
        assert_eq!(d.iri.as_deref(), Some("arqix:requirements/req-01-01-08-01"));
        assert_eq!(d.lang.as_deref(), Some("en"));
        assert_eq!(d.classes, vec!["functional-requirement"]);
        assert_eq!(d.kind(), "requirement");
        assert_eq!(d.triples.len(), 1);
        assert_eq!(d.triples[0].predicate, "derived-from");
        assert_eq!(d.triples[0].object, "arqix:user-stories/us-01-01-08");
        assert!(d.body.contains("Body text."));
    }

    // arqix:no-requirement
    #[test]
    fn kind_falls_back_to_id_pattern_without_classes() {
        let d = parse("r.md", "---\nid: REQ-99-99-99-01\n---\nbody\n");
        assert_eq!(d.kind(), "requirement");
    }

    // arqix:no-requirement
    #[test]
    fn no_frontmatter_is_all_body() {
        let d = parse("x.md", "no frontmatter here\n");
        assert!(!d.has_frontmatter);
        assert_eq!(d.id, None);
        assert!(d.body.contains("no frontmatter"));
    }

    // arqix:no-requirement
    #[test]
    fn requirement_id_shape() {
        assert!(is_requirement_id("REQ-01-02-03-04"));
        assert!(!is_requirement_id("US-01-02-03"));
        assert!(!is_requirement_id("REQ-1-2-3"));
    }

    // arqix:no-requirement
    #[test]
    fn triples_tolerate_variable_whitespace_like_the_oracle() {
        // Extra spaces after the dash, after the colon, and an inline object
        // with no space are all accepted by the oracle's `\s+`/`\s*` regexes.
        let doc = "---\nid: REQ-01-01-08-01\ntriples:\n  -   predicate:  arqix:properties/derived-from\n    object:arqix:user-stories/us-01-01-08\n---\nbody\n";
        let d = parse("r.md", doc);
        assert_eq!(d.triples.len(), 1);
        assert_eq!(d.triples[0].predicate, "derived-from");
        assert_eq!(d.triples[0].object, "arqix:user-stories/us-01-01-08");
    }

    // arqix:no-requirement
    #[test]
    fn triples_reject_trailing_tokens_like_the_oracle() {
        // The oracle anchors on `\s*$`, so a stray trailing token is not a
        // predicate/object line.
        let doc = "---\nid: REQ-01-01-08-01\ntriples:\n  - predicate: arqix:properties/derived-from oops\n---\nbody\n";
        let d = parse("r.md", doc);
        assert!(d.triples.is_empty());
    }

    // arqix:no-requirement
    #[test]
    fn zero_indent_list_items_are_ignored_like_the_oracle() {
        // The oracle's top-level branch catches every non-indented line, so
        // a zero-indent list item is neither a class nor a triple item.
        let doc = "---\nid: REQ-99-99-99-01\nrdf:\n- arqix:classes/functional-requirement\ntriples:\n- predicate: arqix:properties/derived-from\n- arqix:user-stories/us-01-01-08\n---\nbody\n";
        let d = parse("r.md", doc);
        assert!(
            d.classes.is_empty(),
            "zero-indent class item: {:?}",
            d.classes
        );
        assert!(d.triples.is_empty());
    }

    // arqix:no-requirement
    #[test]
    fn section_changes_only_on_top_key_lines_like_the_oracle() {
        // A non-indented line that does not match TOP_KEY_RE `^([\w.-]+):`
        // leaves the current section untouched.
        let doc = "---\nid: REQ-99-99-99-01\nrdf:\n  - arqix:classes/adr\nnot a key line\n  - arqix:classes/second\n---\nbody\n";
        let d = parse("r.md", doc);
        assert_eq!(d.classes, vec!["adr", "second"]);
    }

    // arqix:no-requirement
    #[test]
    fn id_matches_the_oracle_word_shape() {
        // FRONTMATTER_ID_RE `^id:\s*["']?([\w][\w-]*)["']?\s*$`: optional,
        // independently unbalanced quotes around one `[\w][\w-]*` token.
        let quoted = parse("r.md", "---\nid: \"REQ-99-99-99-01'\n---\nbody\n");
        assert_eq!(quoted.id.as_deref(), Some("REQ-99-99-99-01"));
        let spaced = parse("r.md", "---\nid: hello world\n---\nbody\n");
        assert_eq!(spaced.id, None);
        let dashed = parse("r.md", "---\nid: -leading\n---\nbody\n");
        assert_eq!(dashed.id, None);
    }

    // arqix:no-requirement
    #[test]
    fn iri_keeps_quotes_and_takes_a_single_token_like_the_oracle() {
        // FRONTMATTER_IRI_RE `^iri:\s*(\S+)\s*$` keeps the raw token —
        // quotes included — and rejects a multi-token value.
        let quoted = parse("r.md", "---\nid: X\niri: \"arqix:x\"\n---\nbody\n");
        assert_eq!(quoted.iri.as_deref(), Some("\"arqix:x\""));
        let spaced = parse("r.md", "---\nid: X\niri: two tokens\n---\nbody\n");
        assert_eq!(spaced.iri, None);
    }

    // arqix:no-requirement
    #[test]
    fn title_strips_at_most_one_quote_per_side_like_the_oracle() {
        // FRONTMATTER_TITLE_RE `^title:\s*["']?(.+?)["']?\s*$`: one optional
        // quote each side, mismatched pairs allowed, empty value rejected.
        let inner = parse("r.md", "---\nid: X\ntitle: A \"B\"\n---\nbody\n");
        assert_eq!(inner.title.as_deref(), Some("A \"B"));
        let mismatched = parse("r.md", "---\nid: X\ntitle: 'Quoted\"\n---\nbody\n");
        assert_eq!(mismatched.title.as_deref(), Some("Quoted"));
        let empty = parse("r.md", "---\nid: X\ntitle:\n---\nbody\n");
        assert_eq!(empty.title, None);
    }

    // arqix:no-requirement
    #[test]
    fn class_items_follow_the_oracle_regex() {
        // CLASS_ITEM_RE `^-\s+arqix:classes/(\S+)\s*$`.
        let wide = parse(
            "r.md",
            "---\nid: X\nrdf:\n  -   arqix:classes/adr\n---\nbody\n",
        );
        assert_eq!(wide.classes, vec!["adr"]);
        let trailing = parse(
            "r.md",
            "---\nid: X\nrdf:\n  - arqix:classes/adr junk\n---\nbody\n",
        );
        assert!(trailing.classes.is_empty());
        let gap = parse(
            "r.md",
            "---\nid: X\nrdf:\n  - arqix:classes/ adr\n---\nbody\n",
        );
        assert!(gap.classes.is_empty());
    }

    // arqix:no-requirement
    #[test]
    fn lines_split_like_python_splitlines() {
        // The oracle reads documents with str.splitlines, which also breaks
        // on form feed, NEL, and the Unicode line/paragraph separators.
        assert_eq!(
            py_splitlines("a\x0cb\u{2028}c\r\nd\re"),
            vec!["a", "b", "c", "d", "e"]
        );
        assert_eq!(py_splitlines("x\n"), vec!["x"]);
        assert_eq!(py_splitlines(""), Vec::<&str>::new());
    }

    // arqix:no-requirement
    #[test]
    fn triple_predicate_token_must_follow_the_prefix_immediately() {
        // TRIPLE_PRED_RE puts `(\S+)` right after `arqix:properties/` — a
        // space between prefix and token is not a predicate line.
        let doc = "---\nid: X\ntriples:\n  - predicate: arqix:properties/ derived-from\n    object: arqix:user-stories/us-01-01-08\n---\nbody\n";
        let d = parse("r.md", doc);
        assert!(d.triples.is_empty());
    }
}
