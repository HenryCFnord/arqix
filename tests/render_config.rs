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
        header.contains("newunicodechar") && header.contains("IfFontExistsTF"),
        "header.tex maps the ballot glyphs behind a font-existence guard"
    );
    assert!(
        header.contains("amssymb") && header.contains("boxtimes"),
        "header.tex keeps the amssymb no-tofu fallback"
    );
}

// arqix:no-requirement
#[test]
fn pandoc_defaults_wire_the_c4_png_filter() {
    let defaults = repo_file("docs/pandoc/defaults.yaml");
    assert!(
        defaults.contains("filters:") && defaults.contains("docs/pandoc/svg-to-png.lua"),
        "defaults.yaml must wire in the SVG->PNG filter: {defaults}"
    );
    let filter = repo_file("docs/pandoc/svg-to-png.lua");
    assert!(
        filter.contains("model/generated") && filter.contains("png"),
        "the filter rewrites the C4 view embeds to their PNG variant"
    );
    let render = repo_file("scripts/render_views.sh");
    assert!(
        render.contains("structurizr/$fmt") && render.contains("\"svg\" \"png\""),
        "render_views.sh renders both the svg and png variant: {render}"
    );
}
