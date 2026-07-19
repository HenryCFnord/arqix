//! The shipped module catalog (ADR-0021): each process module embeds its
//! ontology documents at compile time, straight from the authoring corpus —
//! the documents stay the single source of truth, a consuming corpus needs
//! no files and no sync step. The reserved core is always effective; the
//! module layers follow `[process].modules` (an unconfigured corpus has
//! every shipped module).

/// The shipped module names, in catalog order.
pub(crate) const SHIPPED_MODULES: [&str; 2] = ["story-driven", "knowledge-base"];

// arqix:implements REQ-08-01-31-02
/// The embedded ontology documents of the reserved core and the effective
/// modules, as (virtual corpus path, text) pairs — parsed by the same
/// reader as corpus documents, so there is exactly one grammar.
pub(crate) fn embedded_ontology_docs(
    modules: &Option<Vec<String>>,
) -> Vec<(&'static str, &'static str)> {
    let effective = |name: &str| modules.as_ref().is_none_or(|m| m.iter().any(|s| s == name));
    let mut docs: Vec<(&'static str, &'static str)> = CORE.to_vec();
    let tables: [&[(&str, &str)]; 2] = [&STORY_DRIVEN, &KNOWLEDGE_BASE];
    for (name, table) in SHIPPED_MODULES.iter().zip(tables) {
        if effective(name) {
            docs.extend_from_slice(table);
        }
    }
    docs
}

/// The reserved-core IRIs: redefining one with different semantics is
/// ONT-009 (REQ-08-01-31-03).
pub(crate) fn core_iris() -> std::collections::HashSet<String> {
    CORE.iter()
        .filter_map(|(_, text)| {
            text.lines()
                .find_map(|line| line.strip_prefix("iri: "))
                .map(|iri| iri.trim().to_string())
        })
        .collect()
}

pub(crate) const CORE: [(&str, &str); 26] = [
    (
        "docs/ontology/classes/adr.md",
        include_str!("../docs/ontology/classes/adr.md"),
    ),
    (
        "docs/ontology/classes/artefact.md",
        include_str!("../docs/ontology/classes/artefact.md"),
    ),
    (
        "docs/ontology/classes/code-artefact.md",
        include_str!("../docs/ontology/classes/code-artefact.md"),
    ),
    (
        "docs/ontology/classes/config-artefact.md",
        include_str!("../docs/ontology/classes/config-artefact.md"),
    ),
    (
        "docs/ontology/classes/document-page.md",
        include_str!("../docs/ontology/classes/document-page.md"),
    ),
    (
        "docs/ontology/classes/documentation.md",
        include_str!("../docs/ontology/classes/documentation.md"),
    ),
    (
        "docs/ontology/classes/implementation-artefact.md",
        include_str!("../docs/ontology/classes/implementation-artefact.md"),
    ),
    (
        "docs/ontology/classes/integration-test.md",
        include_str!("../docs/ontology/classes/integration-test.md"),
    ),
    (
        "docs/ontology/classes/knowledge-artefact.md",
        include_str!("../docs/ontology/classes/knowledge-artefact.md"),
    ),
    (
        "docs/ontology/classes/ontology-definition.md",
        include_str!("../docs/ontology/classes/ontology-definition.md"),
    ),
    (
        "docs/ontology/classes/planning-artefact.md",
        include_str!("../docs/ontology/classes/planning-artefact.md"),
    ),
    (
        "docs/ontology/classes/report.md",
        include_str!("../docs/ontology/classes/report.md"),
    ),
    (
        "docs/ontology/classes/unit.md",
        include_str!("../docs/ontology/classes/unit.md"),
    ),
    (
        "docs/ontology/classes/unit-test.md",
        include_str!("../docs/ontology/classes/unit-test.md"),
    ),
    (
        "docs/ontology/classes/verification-artefact.md",
        include_str!("../docs/ontology/classes/verification-artefact.md"),
    ),
    (
        "docs/ontology/properties/documented-by.md",
        include_str!("../docs/ontology/properties/documented-by.md"),
    ),
    (
        "docs/ontology/properties/documents-artefact.md",
        include_str!("../docs/ontology/properties/documents-artefact.md"),
    ),
    (
        "docs/ontology/properties/has-translation.md",
        include_str!("../docs/ontology/properties/has-translation.md"),
    ),
    (
        "docs/ontology/properties/includes-unit.md",
        include_str!("../docs/ontology/properties/includes-unit.md"),
    ),
    (
        "docs/ontology/properties/is-included-in.md",
        include_str!("../docs/ontology/properties/is-included-in.md"),
    ),
    (
        "docs/ontology/properties/is-superseded-by.md",
        include_str!("../docs/ontology/properties/is-superseded-by.md"),
    ),
    (
        "docs/ontology/properties/references-artefact.md",
        include_str!("../docs/ontology/properties/references-artefact.md"),
    ),
    (
        "docs/ontology/properties/references-external-source.md",
        include_str!("../docs/ontology/properties/references-external-source.md"),
    ),
    (
        "docs/ontology/properties/supersedes.md",
        include_str!("../docs/ontology/properties/supersedes.md"),
    ),
    (
        "docs/ontology/properties/tests-code-artefact.md",
        include_str!("../docs/ontology/properties/tests-code-artefact.md"),
    ),
    (
        "docs/ontology/properties/translation-of.md",
        include_str!("../docs/ontology/properties/translation-of.md"),
    ),
];

pub(crate) const STORY_DRIVEN: [(&str, &str); 34] = [
    (
        "docs/ontology/classes/constraint.md",
        include_str!("../docs/ontology/classes/constraint.md"),
    ),
    (
        "docs/ontology/classes/functional-requirement.md",
        include_str!("../docs/ontology/classes/functional-requirement.md"),
    ),
    (
        "docs/ontology/classes/persona.md",
        include_str!("../docs/ontology/classes/persona.md"),
    ),
    (
        "docs/ontology/classes/quality-requirement.md",
        include_str!("../docs/ontology/classes/quality-requirement.md"),
    ),
    (
        "docs/ontology/classes/requirement.md",
        include_str!("../docs/ontology/classes/requirement.md"),
    ),
    (
        "docs/ontology/classes/user-story.md",
        include_str!("../docs/ontology/classes/user-story.md"),
    ),
    (
        "docs/ontology/classes/verification-method.md",
        include_str!("../docs/ontology/classes/verification-method.md"),
    ),
    (
        "docs/ontology/classes/workflow.md",
        include_str!("../docs/ontology/classes/workflow.md"),
    ),
    (
        "docs/ontology/properties/derived-from.md",
        include_str!("../docs/ontology/properties/derived-from.md"),
    ),
    (
        "docs/ontology/properties/guides-design-of.md",
        include_str!("../docs/ontology/properties/guides-design-of.md"),
    ),
    (
        "docs/ontology/properties/guides-implementation-of.md",
        include_str!("../docs/ontology/properties/guides-implementation-of.md"),
    ),
    (
        "docs/ontology/properties/guides-verification-of.md",
        include_str!("../docs/ontology/properties/guides-verification-of.md"),
    ),
    (
        "docs/ontology/properties/has-persona.md",
        include_str!("../docs/ontology/properties/has-persona.md"),
    ),
    (
        "docs/ontology/properties/has-primary-persona.md",
        include_str!("../docs/ontology/properties/has-primary-persona.md"),
    ),
    (
        "docs/ontology/properties/has-relevant-persona.md",
        include_str!("../docs/ontology/properties/has-relevant-persona.md"),
    ),
    (
        "docs/ontology/properties/has-requirement.md",
        include_str!("../docs/ontology/properties/has-requirement.md"),
    ),
    (
        "docs/ontology/properties/has-story.md",
        include_str!("../docs/ontology/properties/has-story.md"),
    ),
    (
        "docs/ontology/properties/has-verification-method.md",
        include_str!("../docs/ontology/properties/has-verification-method.md"),
    ),
    (
        "docs/ontology/properties/implements-requirement.md",
        include_str!("../docs/ontology/properties/implements-requirement.md"),
    ),
    (
        "docs/ontology/properties/is-part-of-workflow.md",
        include_str!("../docs/ontology/properties/is-part-of-workflow.md"),
    ),
    (
        "docs/ontology/properties/is-persona-for-story.md",
        include_str!("../docs/ontology/properties/is-persona-for-story.md"),
    ),
    (
        "docs/ontology/properties/is-primary-persona-for-workflow.md",
        include_str!("../docs/ontology/properties/is-primary-persona-for-workflow.md"),
    ),
    (
        "docs/ontology/properties/is-relevant-persona-for-workflow.md",
        include_str!("../docs/ontology/properties/is-relevant-persona-for-workflow.md"),
    ),
    (
        "docs/ontology/properties/is-requirement-for-story.md",
        include_str!("../docs/ontology/properties/is-requirement-for-story.md"),
    ),
    (
        "docs/ontology/properties/is-verified-by.md",
        include_str!("../docs/ontology/properties/is-verified-by.md"),
    ),
    (
        "docs/ontology/properties/realizes-user-story.md",
        include_str!("../docs/ontology/properties/realizes-user-story.md"),
    ),
    (
        "docs/ontology/properties/realizes-verification-method.md",
        include_str!("../docs/ontology/properties/realizes-verification-method.md"),
    ),
    (
        "docs/ontology/properties/supports-verification-of.md",
        include_str!("../docs/ontology/properties/supports-verification-of.md"),
    ),
    (
        "docs/ontology/properties/verifies-requirement.md",
        include_str!("../docs/ontology/properties/verifies-requirement.md"),
    ),
    (
        "docs/ontology/properties/verifies-user-story.md",
        include_str!("../docs/ontology/properties/verifies-user-story.md"),
    ),
    (
        "docs/ontology/individuals/analysis.md",
        include_str!("../docs/ontology/individuals/analysis.md"),
    ),
    (
        "docs/ontology/individuals/demonstration.md",
        include_str!("../docs/ontology/individuals/demonstration.md"),
    ),
    (
        "docs/ontology/individuals/inspection.md",
        include_str!("../docs/ontology/individuals/inspection.md"),
    ),
    (
        "docs/ontology/individuals/test-method.md",
        include_str!("../docs/ontology/individuals/test-method.md"),
    ),
];

pub(crate) const KNOWLEDGE_BASE: [(&str, &str); 3] = [
    (
        "docs/ontology/classes/claim.md",
        include_str!("../docs/ontology/classes/claim.md"),
    ),
    (
        "docs/ontology/classes/source.md",
        include_str!("../docs/ontology/classes/source.md"),
    ),
    (
        "docs/ontology/properties/supported-by.md",
        include_str!("../docs/ontology/properties/supported-by.md"),
    ),
];
