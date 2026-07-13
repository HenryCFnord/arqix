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

/// Parse the small Structurizr-DSL subset the model uses: element
/// declarations (`id = kind "Name" …`), relationships (`src -> dst …`), and
/// brace nesting so a container knows its enclosing system. Anything else
/// (views, styles, the workspace header) only balances the brace stack.
fn parse_dsl(text: &str) -> DslModel {
    let mut elements = Vec::new();
    let mut edges = HashSet::new();
    let mut scope: Vec<Option<String>> = Vec::new();
    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with("//") {
            continue;
        }
        if let Some(element) = parse_dsl_element(line, &scope) {
            let id = element.id.clone();
            elements.push(element);
            if line.ends_with('{') {
                scope.push(Some(id));
            }
        } else if let Some((src, dst)) = parse_dsl_relationship(line) {
            edges.insert((src, dst));
        } else if line == "}" {
            scope.pop();
        } else if line.ends_with('{') {
            scope.push(None);
        }
    }
    DslModel { elements, edges }
}

/// `<id> = <kind> "Name" [more] [{`: the enclosing element (nearest `Some` on
/// the brace stack) is the parent; a softwareSystem tagged `"External"` is
/// external.
fn parse_dsl_element(line: &str, scope: &[Option<String>]) -> Option<DslElement> {
    let (id, rest) = line.split_once('=')?;
    let id = id.trim();
    if id.is_empty() || !id.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }
    let (keyword, after) = rest.trim().split_once(char::is_whitespace)?;
    let kind = match keyword {
        "person" => Kind::Person,
        "softwareSystem" => Kind::System,
        "container" => Kind::Container,
        "component" => Kind::Component,
        _ => return None,
    };
    let strings = quoted_strings(after);
    let name = strings.first()?.clone();
    let external = kind == Kind::System && strings.iter().any(|s| s == "External");
    let parent = scope.iter().rev().find_map(|s| s.clone());
    Some(DslElement {
        id: id.to_string(),
        kind,
        name,
        external,
        parent,
    })
}

/// `<src> -> <dst> ["label"]`: a model relationship by element id.
fn parse_dsl_relationship(line: &str) -> Option<(String, String)> {
    let (src, rest) = line.split_once("->")?;
    let src = src.trim();
    if src.is_empty() || !src.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }
    let dst: String = rest
        .trim()
        .chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect();
    (!dst.is_empty()).then(|| (src.to_string(), dst))
}

/// Parse a C4 Mermaid block into its elements and relationships.
fn parse_mermaid(text: &str) -> Diagram {
    let mut elements = Vec::new();
    let mut rels = Vec::new();
    for raw in text.lines() {
        let line = raw.trim();
        if let Some(element) = parse_c4_element(line) {
            elements.push(element);
        } else if let Some(rel) = parse_c4_rel(line) {
            rels.push(rel);
        }
    }
    Diagram { elements, rels }
}

/// `Person(id, "Name", …)` / `System[_Ext](…)` / `Container(…)` /
/// `Component(…)`: the diagram id is the first argument, the name the first
/// quoted string. `System_Boundary` and layout directives are not elements.
fn parse_c4_element(line: &str) -> Option<DiaElement> {
    let (func, args) = line.split_once('(')?;
    let (kind, external) = match func.trim() {
        "Person" => (Kind::Person, false),
        "Person_Ext" => (Kind::Person, true),
        "System" | "SystemDb" => (Kind::System, false),
        "System_Ext" | "SystemDb_Ext" => (Kind::System, true),
        "Container" | "ContainerDb" => (Kind::Container, false),
        "Component" | "ComponentDb" => (Kind::Component, false),
        _ => return None,
    };
    let id: String = args
        .chars()
        .take_while(|c| *c != ',')
        .collect::<String>()
        .trim()
        .to_string();
    let name = quoted_strings(args).into_iter().next()?;
    (!id.is_empty()).then_some(DiaElement {
        id,
        kind,
        name,
        external,
    })
}

/// `Rel(src, dst, "label")`: a diagram relationship by diagram id.
fn parse_c4_rel(line: &str) -> Option<(String, String)> {
    let args = ["Rel(", "Rel_Back(", "BiRel("]
        .iter()
        .find_map(|prefix| line.strip_prefix(prefix))?;
    let mut parts = args.splitn(3, ',');
    let src = parts.next()?.trim().to_string();
    let dst = parts
        .next()?
        .trim()
        .trim_end_matches(')')
        .trim()
        .to_string();
    (!src.is_empty() && !dst.is_empty()).then_some((src, dst))
}

/// Check one derived Mermaid view against the model, emitting a finding for
/// each diagram element the model does not define (REQ-04-01-18-01) and each
/// relationship the model does not justify (REQ-04-01-18-02). Elements are
/// matched by display name so the diagrams' shortened ids validate; free-text
/// labels are not compared (ADR-0016).
pub(crate) fn check(
    dsl_text: &str,
    mermaid_text: &str,
    view: &str,
    file: &str,
    line: usize,
) -> Vec<Diagnostic> {
    let dsl = parse_dsl(dsl_text);
    let diagram = parse_mermaid(mermaid_text);
    let mut diags = Vec::new();
    let id_name: std::collections::HashMap<&str, &str> = diagram
        .elements
        .iter()
        .map(|e| (e.id.as_str(), e.name.as_str()))
        .collect();

    for de in &diagram.elements {
        match dsl.by_name(&de.name) {
            None => diags.push(
                Diagnostic::error(
                    "LNT-DRV-001",
                    format!(
                        "view {view}: element \"{}\" is not defined in the model",
                        de.name
                    ),
                )
                .at_line(file, line),
            ),
            Some(me) if me.kind != de.kind => diags.push(
                Diagnostic::error(
                    "LNT-DRV-001",
                    format!(
                        "view {view}: element \"{}\" is a {} in the diagram but a {} in the model",
                        de.name,
                        de.kind.label(),
                        me.kind.label()
                    ),
                )
                .at_line(file, line),
            ),
            Some(me) if de.kind == Kind::System && de.external != me.external => {
                let (in_diagram, in_model) = if de.external {
                    ("external", "internal")
                } else {
                    ("internal", "external")
                };
                diags.push(
                    Diagnostic::error(
                        "LNT-DRV-001",
                        format!(
                            "view {view}: element \"{}\" is {in_diagram} in the diagram but {in_model} in the model",
                            de.name
                        ),
                    )
                    .at_line(file, line),
                );
            }
            Some(_) => {}
        }
    }

    for (src_id, dst_id) in &diagram.rels {
        let (Some(src_name), Some(dst_name)) =
            (id_name.get(src_id.as_str()), id_name.get(dst_id.as_str()))
        else {
            continue; // an endpoint not declared in the diagram is not our finding
        };
        let (Some(src), Some(dst)) = (dsl.by_name(src_name), dsl.by_name(dst_name)) else {
            continue; // an unresolved element is already an LNT-DRV-001
        };
        if !dsl.justified(&src.id, &dst.id) {
            diags.push(
                Diagnostic::error(
                    "LNT-DRV-002",
                    format!(
                        "view {view}: relationship \"{src_name}\" -> \"{dst_name}\" is not justified by the model"
                    ),
                )
                .at_line(file, line),
            );
        }
    }
    diags
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
