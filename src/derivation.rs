//! Derivation check (ADR-0016): verifies that a committed C4 Mermaid view is
//! structurally derivable from the C4 model (`workspace.dsl`). Every diagram
//! element must map to a model element of matching kind — matched by display
//! name, not id, so the hand-abbreviated diagrams (`render` for
//! `renderToolchain`) validate unchanged — and every diagram relationship must
//! map to a model edge, direct or Structurizr-implied (a system-level edge
//! justifies an edge to a container of that system). In-process and
//! dependency-free; it checks, it does not generate (US-04-01-18).

use crate::diag::Diagnostic;
use std::collections::HashSet;

/// Parse a `<!-- derived from <path> (view: <View>) -->` marker into the model
/// path (relative to the document) and the named view.
pub(crate) fn derived_marker(line: &str) -> Option<(String, String)> {
    let inner = line
        .trim()
        .strip_prefix("<!--")?
        .strip_suffix("-->")?
        .trim();
    let rest = inner.strip_prefix("derived from")?;
    if !rest.starts_with(char::is_whitespace) {
        return None;
    }
    let (path, view) = rest.trim().split_once("(view:")?;
    let path = path.trim();
    let view = view.strip_suffix(')')?.trim();
    if path.is_empty() || view.is_empty() {
        return None;
    }
    Some((path.to_string(), view.to_string()))
}

/// The C4 element kinds the model and the diagrams share.
#[derive(Clone, Copy, PartialEq, Debug)]
enum Kind {
    Person,
    System,
    Container,
    Component,
}

impl Kind {
    fn label(self) -> &'static str {
        match self {
            Kind::Person => "person",
            Kind::System => "software system",
            Kind::Container => "container",
            Kind::Component => "component",
        }
    }
}

struct DslElement {
    id: String,
    kind: Kind,
    name: String,
    external: bool,
    parent: Option<String>,
}

struct DslModel {
    elements: Vec<DslElement>,
    edges: HashSet<(String, String)>,
}

impl DslModel {
    fn by_name(&self, name: &str) -> Option<&DslElement> {
        self.elements.iter().find(|e| e.name == name)
    }

    fn parent_of(&self, id: &str) -> Option<String> {
        self.elements
            .iter()
            .find(|e| e.id == id)
            .and_then(|e| e.parent.clone())
    }

    /// Whether the model connects `s` to `d` directly or by a Structurizr
    /// implied edge: a container view inherits a system-level edge for a
    /// container of that system, so an edge to/from a container's parent
    /// counts (ADR-0016).
    fn justified(&self, s: &str, d: &str) -> bool {
        let sp = self.parent_of(s);
        let dp = self.parent_of(d);
        let candidates = [
            (Some(s.to_string()), Some(d.to_string())),
            (Some(s.to_string()), dp.clone()),
            (sp.clone(), Some(d.to_string())),
            (sp, dp),
        ];
        candidates.iter().any(|(a, b)| match (a, b) {
            (Some(a), Some(b)) => self.edges.contains(&(a.clone(), b.clone())),
            _ => false,
        })
    }
}

struct DiaElement {
    id: String,
    kind: Kind,
    name: String,
    external: bool,
}

struct Diagram {
    elements: Vec<DiaElement>,
    rels: Vec<(String, String)>,
}

/// The `"..."` string literals in a line, in order (no escapes in the DSL or
/// the C4 Mermaid the model uses).
fn quoted_strings(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut chars = line.chars();
    while let Some(c) = chars.next() {
        if c == '"' {
            let mut s = String::new();
            for c in chars.by_ref() {
                if c == '"' {
                    break;
                }
                s.push(c);
            }
            out.push(s);
        }
    }
    out
}

fn parse_dsl(_text: &str) -> DslModel {
    DslModel {
        elements: Vec::new(),
        edges: HashSet::new(),
    }
}

fn parse_mermaid(_text: &str) -> Diagram {
    Diagram {
        elements: Vec::new(),
        rels: Vec::new(),
    }
}

/// Check one derived Mermaid view against the model, emitting a finding for
/// each diagram element the model does not define (REQ-04-01-18-01) and each
/// relationship the model does not justify (REQ-04-01-18-02).
pub(crate) fn check(
    _dsl_text: &str,
    _mermaid_text: &str,
    _view: &str,
    _file: &str,
    _line: usize,
) -> Vec<Diagnostic> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DSL: &str = r#"
workspace "t" "d" {
    model {
        agent = person "Coding Agent" "loops"
        sys = softwareSystem "The System" "core" {
            cli = container "CLI Binary" "all" "Rust"
        }
        ext = softwareSystem "External Thing" "x" "External"
        agent -> sys "uses"
        cli -> ext "calls"
    }
    views {
        systemContext sys "SystemContext" {
            include *
        }
    }
}
"#;

    // arqix:no-requirement
    #[test]
    fn derived_marker_parses_path_and_view() {
        assert_eq!(
            derived_marker("<!-- derived from ../model/workspace.dsl (view: SystemContext) -->"),
            Some((
                "../model/workspace.dsl".to_string(),
                "SystemContext".to_string()
            ))
        );
        assert_eq!(derived_marker("<!-- a normal comment -->"), None);
    }

    // arqix:no-requirement
    #[test]
    fn parse_dsl_extracts_elements_edges_and_parents() {
        let m = parse_dsl(DSL);
        let cli = m.by_name("CLI Binary").expect("container by name");
        assert_eq!(cli.kind, Kind::Container);
        assert_eq!(cli.parent.as_deref(), Some("sys"));
        let ext = m.by_name("External Thing").expect("external system");
        assert!(ext.external, "the External tag is read");
        assert_eq!(m.by_name("The System").unwrap().kind, Kind::System);
        assert!(m.edges.contains(&("agent".to_string(), "sys".to_string())));
        assert!(
            m.justified("agent", "cli"),
            "agent->sys justifies agent->cli"
        );
    }

    // arqix:no-requirement
    #[test]
    fn parse_mermaid_extracts_named_elements_and_rels() {
        let mm = "C4Container\n    Person(a, \"Coding Agent\", \"loops\")\n    Container(c, \"CLI Binary\", \"Rust\", \"all\")\n    Rel(a, c, \"uses\")\n";
        let d = parse_mermaid(mm);
        assert_eq!(d.elements.len(), 2);
        assert_eq!(d.elements[0].name, "Coding Agent");
        assert_eq!(d.rels, vec![("a".to_string(), "c".to_string())]);
    }

    // arqix:verifies REQ-04-01-18-01
    #[test]
    fn check_flags_an_element_absent_from_the_model() {
        let mm = "C4Context\n    System(x, \"Nonexistent System\", \"y\")\n";
        let diags = check(DSL, mm, "SystemContext", "u.md", 10);
        assert_eq!(diags.len(), 1, "the phantom element is flagged: {diags:?}");
        assert_eq!(diags[0].code, "LNT-DRV-001");
        assert!(diags[0].message.contains("Nonexistent System"));
    }

    // arqix:verifies REQ-04-01-18-01
    #[test]
    fn check_matches_by_name_not_id_and_accepts_shortened_ids() {
        // Diagram id `render` differs from DSL id `ext`, but the display name
        // matches, and the External tag lines up.
        let mm = "C4Context\n    System_Ext(render, \"External Thing\", \"x\")\n";
        let diags = check(DSL, mm, "SystemContext", "u.md", 10);
        assert!(
            diags.is_empty(),
            "name match tolerates shortened ids: {diags:?}"
        );
    }

    // arqix:verifies REQ-04-01-18-02
    #[test]
    fn check_flags_a_relationship_the_model_does_not_justify() {
        // cli -> agent has no direct or implied model edge.
        let mm = "C4Container\n    Person(a, \"Coding Agent\", \"loops\")\n    Container(c, \"CLI Binary\", \"Rust\", \"all\")\n    Rel(c, a, \"bogus\")\n";
        let diags = check(DSL, mm, "Containers", "u.md", 10);
        assert!(
            diags.iter().any(|d| d.code == "LNT-DRV-002"),
            "the unjustified relationship is flagged: {diags:?}"
        );
    }

    // arqix:verifies REQ-04-01-18-02
    #[test]
    fn check_accepts_a_valid_view_with_an_implied_edge() {
        // agent -> cli is justified by the model's agent -> sys (cli is a
        // container of sys), the Structurizr container-view pushdown.
        let mm = "C4Container\n    Person(a, \"Coding Agent\", \"loops\")\n    Container(c, \"CLI Binary\", \"Rust\", \"all\")\n    Rel(a, c, \"uses\")\n";
        let diags = check(DSL, mm, "Containers", "u.md", 10);
        assert!(diags.is_empty(), "a faithful view is clean: {diags:?}");
    }
}
