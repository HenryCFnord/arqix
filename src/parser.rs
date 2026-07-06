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

/// Extract a scalar frontmatter value: `key: value`, quotes stripped.
fn scalar(line: &str, key: &str) -> Option<String> {
    let rest = line.strip_prefix(key)?.strip_prefix(':')?;
    Some(unquote(rest.trim()))
}

fn after<'a>(line: &'a str, prefix: &str) -> Option<&'a str> {
    line.strip_prefix(prefix).map(str::trim)
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

pub fn parse(file: &str, text: &str) -> Document {
    let mut doc = Document::empty(file);
    let lines: Vec<&str> = text.lines().collect();

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
        let indented = line.starts_with(char::is_whitespace);

        if !line.is_empty() && !indented && !stripped.starts_with('-') {
            section = line.split(':').next().unwrap_or("").to_string();
            predicate = None;
            if let Some(v) = scalar(stripped, "id") {
                doc.id = Some(v);
            } else if let Some(v) = scalar(stripped, "iri") {
                doc.iri = Some(v);
            } else if let Some(v) = scalar(stripped, "title") {
                doc.title = Some(v);
            }
            continue;
        }

        match section.as_str() {
            "rdf" => {
                if let Some(cls) = after(stripped, "- arqix:classes/") {
                    doc.classes.push(cls.to_string());
                }
            }
            "triples" => {
                if let Some(pred) = after(stripped, "- predicate: arqix:properties/") {
                    predicate = Some(pred.to_string());
                } else if let Some(pred) = predicate.clone() {
                    let object = after(stripped, "object: ")
                        .or_else(|| after(stripped, "- "))
                        .filter(|obj| obj.starts_with("arqix:"));
                    if let Some(obj) = object {
                        doc.triples.push(Triple {
                            predicate: pred,
                            object: obj.to_string(),
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

    #[test]
    fn kind_falls_back_to_id_pattern_without_classes() {
        let d = parse("r.md", "---\nid: REQ-99-99-99-01\n---\nbody\n");
        assert_eq!(d.kind(), "requirement");
    }

    #[test]
    fn no_frontmatter_is_all_body() {
        let d = parse("x.md", "no frontmatter here\n");
        assert!(!d.has_frontmatter);
        assert_eq!(d.id, None);
        assert!(d.body.contains("no frontmatter"));
    }

    #[test]
    fn requirement_id_shape() {
        assert!(is_requirement_id("REQ-01-02-03-04"));
        assert!(!is_requirement_id("US-01-02-03"));
        assert!(!is_requirement_id("REQ-1-2-3"));
    }
}
