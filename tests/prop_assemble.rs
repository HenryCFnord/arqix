//! Property rule 3 (nothing vanishes): every statement of every included
//! unit appears in the assembled page — headings possibly re-leveled,
//! fenced lines verbatim, unit frontmatter omitted (REQ-02-01-12-05).

mod common;

use common::run_arqix_in;
use proptest::prelude::*;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

static CASE: AtomicUsize = AtomicUsize::new(0);

fn unit_line() -> impl Strategy<Value = String> {
    prop_oneof![
        "[A-Za-z][A-Za-z0-9 äöü.,]{0,24}",
        prop::sample::select(vec![
            "## Section",
            "### Deeper",
            "- a list item",
            "Prose that must survive.",
        ])
        .prop_map(str::to_string),
    ]
}

fn fenced_block() -> impl Strategy<Value = Vec<String>> {
    prop::collection::vec("[#a-z ]{1,16}", 1..4).prop_map(|inner| {
        let mut block = vec!["```text".to_string()];
        block.extend(inner);
        block.push("```".to_string());
        block
    })
}

fn unit_body() -> impl Strategy<Value = (Vec<String>, Vec<String>)> {
    (
        prop::collection::vec(unit_line(), 1..6),
        prop::option::of(fenced_block()),
    )
        .prop_map(|(mut lines, fence)| {
            lines.insert(0, "## Unit".to_string());
            let fence = fence.unwrap_or_default();
            (lines, fence)
        })
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(32))]

    // arqix:verifies REQ-02-01-12-05
    #[test]
    fn every_unit_line_reaches_the_assembled_page(
        bodies in prop::collection::vec(unit_body(), 1..4)
    ) {
        let case = CASE.fetch_add(1, Ordering::SeqCst);
        let repo = std::env::temp_dir().join(format!(
            "arqix-prop-assemble-{}-{case}",
            std::process::id()
        ));
        let units_dir = repo.join("docs/units");
        std::fs::create_dir_all(&units_dir).unwrap();
        let mut root = String::from("---\nid: page-1\ntitle: Page\n---\n\n# Page\n\n");
        for (i, (lines, fence)) in bodies.iter().enumerate() {
            let mut body = lines.clone();
            body.extend(fence.iter().cloned());
            let unit_text = format!(
                "---\nid: unit-{i}\ntitle: Unit {i}\n---\n\n{}\n",
                body.join("\n")
            );
            std::fs::write(units_dir.join(format!("u{i}.md")), unit_text).unwrap();
            root.push_str(&format!("<!-- arqix:include units/u{i}.md -->\n\n"));
        }
        std::fs::write(repo.join("docs/page.md"), &root).unwrap();

        let out = run_arqix_in(&repo, &["assemble", "build"]);
        prop_assert_eq!(out.status.code(), Some(0), "assemble failed: {}",
            String::from_utf8_lossy(&out.stderr));
        let expanded = std::fs::read_to_string(repo.join(PathBuf::from("pages/page.md")))
            .expect("assembled page written");

        for (lines, fence) in &bodies {
            for line in lines {
                let survives = if let Some(rest) = line.strip_prefix('#') {
                    // A heading may be re-leveled: its text survives under
                    // some depth.
                    let text = rest.trim_start_matches('#');
                    expanded
                        .lines()
                        .any(|l| l.starts_with('#') && l.trim_start_matches('#') == text)
                } else {
                    expanded.lines().any(|l| l == line)
                };
                prop_assert!(survives, "line {:?} vanished from:\n{expanded}", line);
            }
            for line in fence {
                // Fenced lines survive verbatim: the shift never reaches
                // inside a code fence, whatever the line looks like.
                prop_assert!(
                    expanded.lines().any(|l| l == line),
                    "fenced line {:?} not verbatim in:\n{expanded}",
                    line
                );
            }
        }
        prop_assert!(!expanded.contains("id: unit-"), "frontmatter leaked:\n{expanded}");
        std::fs::remove_dir_all(&repo).ok();
    }
}
