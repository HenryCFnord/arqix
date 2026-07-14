//! Config-wiring guards for the PDF render defaults (docs/pandoc). These assert
//! the repository's own Pandoc configuration stays wired together; the LaTeX /
//! render correctness itself is confirmed by a Docker render, not here.

use std::path::PathBuf;

fn repo_file(rel: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

// arqix:no-requirement
#[test]
fn pandoc_defaults_wire_the_checkbox_glyph_header() {
    let defaults = repo_file("docs/pandoc/defaults.yaml");
    assert!(
        defaults.contains("include-in-header:") && defaults.contains("docs/pandoc/header.tex"),
        "defaults.yaml must wire in the checkbox-glyph header: {defaults}"
    );
    let header = repo_file("docs/pandoc/header.tex");
    assert!(
        header.contains("IfFontExistsTF"),
        "header.tex maps the ballot glyphs behind a font-existence guard: {header}"
    );
    assert!(
        header.contains("\\fbox") && header.contains("catcode"),
        "header.tex keeps a base-LaTeX no-tofu fallback and remaps the ballot code points: {header}"
    );
}
