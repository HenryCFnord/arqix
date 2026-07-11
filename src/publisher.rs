//! Publish & Render Orchestrator, site half: `publish site` renders the
//! corpus into a static site (REQ-04-01-03-01), per language root
//! (REQ-04-01-07-01), with a deterministic built-in Markdown renderer as the
//! default toolchain and an optional configured `site-command` orchestrated
//! after generation (REQ-04-01-03-03). Toolchain failures are system errors
//! (exit 2), never findings (REQ-04-01-07-02). Like the assembler, it only
//! writes generated artefacts — sources are never mutated (ADR-0004).

use crate::OutputFormat;
use crate::diag::SCHEMA_VERSION;
use serde_json::json;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

// arqix:implements REQ-04-01-03-01
// arqix:implements REQ-04-01-03-03
// arqix:implements REQ-04-01-07-01
// arqix:implements REQ-04-01-07-02
/// `arqix publish site [--lang <lang>]`
pub fn site(lang: Option<&str>, format: OutputFormat) -> ExitCode {
    let policy = crate::config::publish_policy(Path::new("."));
    let default_lang = crate::config::default_lang(Path::new("."));
    let lang = lang.unwrap_or(&default_lang);

    // The language's target: the default language owns the site root, every
    // other language its own subdirectory (REQ-04-01-07-01).
    let out_dir = if lang == default_lang {
        PathBuf::from(&policy.site_dir)
    } else {
        Path::new(&policy.site_dir).join(lang)
    };

    let mut pages: Vec<(String, String)> = Vec::new(); // (rel html path, title)
    let skip = crate::config::skip_dirs(Path::new("."));
    for root in crate::config::roots(Path::new(".")) {
        // The language's root: <root>/<lang> where the layout has one; the
        // bare root serves the default language otherwise.
        let lang_root = Path::new(&root).join(lang);
        let lang_root = if lang_root.is_dir() {
            lang_root
        } else if lang == default_lang {
            PathBuf::from(&root)
        } else {
            continue;
        };

        let mut files = Vec::new();
        collect_markdown(&lang_root, &skip, &mut files);
        for file in files {
            let text = match std::fs::read_to_string(&file) {
                Ok(text) => text,
                Err(err) => {
                    eprintln!("error: cannot read {}: {err}", file.display());
                    return ExitCode::from(2);
                }
            };
            let doc = crate::parser::parse(&file.to_string_lossy(), &text);
            let title = doc.title.clone().unwrap_or_else(|| stem(&file).to_string());
            let rel = file
                .strip_prefix(&lang_root)
                .unwrap_or(&file)
                .with_extension("html")
                .to_string_lossy()
                .replace('\\', "/");
            let html = page_html(&title, &render_markdown(&doc.body));
            if let Err(code) = write_page(&out_dir.join(&rel), &html) {
                return code;
            }
            pages.push((rel, title));
        }
    }

    pages.sort();
    if let Err(code) = write_page(&out_dir.join("index.html"), &index_html(&pages)) {
        return code;
    }

    // Orchestrate the configured toolchain after generation, inheriting
    // stdio so its own errors surface transparently.
    if let Some(command) = &policy.site_command {
        let mut parts = command.split_whitespace();
        let Some(program) = parts.next() else {
            eprintln!("error: [policies.publish] site-command is empty");
            return ExitCode::from(2);
        };
        let status = Command::new(program).args(parts).status();
        match status {
            Ok(status) if status.success() => {}
            Ok(status) => {
                eprintln!(
                    "error: site toolchain failed: '{command}' exited with {}",
                    status
                        .code()
                        .map_or("signal".to_string(), |c| c.to_string())
                );
                return ExitCode::from(2);
            }
            Err(err) => {
                eprintln!("error: site toolchain failed: cannot run '{command}': {err}");
                return ExitCode::from(2);
            }
        }
    }

    match format {
        OutputFormat::Json => {
            let result = json!({
                "schema_version": SCHEMA_VERSION,
                "lang": lang,
                "out_dir": out_dir.to_string_lossy(),
                "pages": pages.len() + 1,
            });
            println!(
                "{}",
                serde_json::to_string_pretty(&result).expect("valid JSON")
            );
        }
        OutputFormat::Text => println!(
            "published {} page(s) to {}",
            pages.len() + 1,
            out_dir.display()
        ),
    }
    ExitCode::SUCCESS
}

/// Collect Markdown sources under `dir`, path-sorted, honouring the skip
/// set; templates (`.tpl.md`) are never published.
fn collect_markdown(dir: &Path, skip: &[String], files: &mut Vec<PathBuf>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };
    let mut paths: Vec<_> = entries.flatten().map(|e| e.path()).collect();
    paths.sort();
    for path in paths {
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if path.is_dir() {
            if !path.symlink_metadata().is_ok_and(|m| m.is_symlink())
                && !skip.iter().any(|s| s == name)
            {
                collect_markdown(&path, skip, files);
            }
        } else if name.ends_with(".md") && !name.ends_with(".tpl.md") {
            files.push(path);
        }
    }
}

fn write_page(path: &Path, html: &str) -> Result<(), ExitCode> {
    if let Some(parent) = path.parent()
        && let Err(err) = std::fs::create_dir_all(parent)
    {
        eprintln!("error: cannot create {}: {err}", parent.display());
        return Err(ExitCode::from(2));
    }
    if let Err(err) = std::fs::write(path, html) {
        eprintln!("error: cannot write {}: {err}", path.display());
        return Err(ExitCode::from(2));
    }
    Ok(())
}

fn stem(file: &Path) -> &str {
    file.file_stem().and_then(|s| s.to_str()).unwrap_or("page")
}

/// The shared page shell: self-contained, no external assets, light/dark
/// via `color-scheme` — the site must work offline and unstyled-proof.
fn page_html(title: &str, body: &str) -> String {
    format!(
        "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n\
         <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n\
         <title>{title}</title>\n<style>{CSS}</style>\n</head>\n<body>\n<main>\n\
         {body}</main>\n<footer><a href=\"index.html\">index</a> · generated by arqix</footer>\n\
         </body>\n</html>\n",
        title = escape(title),
    )
}

fn index_html(pages: &[(String, String)]) -> String {
    let mut items = String::new();
    for (href, title) in pages {
        items.push_str(&format!(
            "<li><a href=\"{href}\">{}</a></li>\n",
            escape(title)
        ));
    }
    page_html(
        "Documentation",
        &format!("<h1>Documentation</h1>\n<ul>\n{items}</ul>\n"),
    )
}

const CSS: &str = "
:root { color-scheme: light dark; }
body { font-family: ui-sans-serif, system-ui, sans-serif; line-height: 1.6;
       max-width: 46rem; margin: 0 auto; padding: 2rem 1.5rem; }
pre { overflow-x: auto; padding: 0.75rem; background: rgba(127,127,127,0.12); }
code { font-family: ui-monospace, monospace; }
table { border-collapse: collapse; }
td, th { border: 1px solid rgba(127,127,127,0.4); padding: 0.25rem 0.5rem; }
footer { margin-top: 3rem; font-size: 0.8rem; opacity: 0.7; }
";

/// Deterministic minimal Markdown-to-HTML for the corpus subset: ATX
/// headings, paragraphs, flat lists, fenced code, tables, and the inline
/// span set (code, bold, italic, links). Directives and HTML comments are
/// dropped; relative `.md` links are rewritten to `.html`.
fn render_markdown(body: &str) -> String {
    let mut out = String::new();
    let mut paragraph: Vec<String> = Vec::new();
    let mut list: Vec<String> = Vec::new();
    let mut table: Vec<String> = Vec::new();
    let mut code: Option<Vec<String>> = None;

    let flush_paragraph = |out: &mut String, paragraph: &mut Vec<String>| {
        if !paragraph.is_empty() {
            out.push_str(&format!("<p>{}</p>\n", inline(&paragraph.join(" "))));
            paragraph.clear();
        }
    };
    let flush_list = |out: &mut String, list: &mut Vec<String>| {
        if !list.is_empty() {
            out.push_str("<ul>\n");
            for item in list.iter() {
                out.push_str(&format!("<li>{}</li>\n", inline(item)));
            }
            out.push_str("</ul>\n");
            list.clear();
        }
    };
    let flush_table = |out: &mut String, table: &mut Vec<String>| {
        if !table.is_empty() {
            out.push_str("<table>\n");
            for (idx, row) in table.iter().enumerate() {
                let cells: Vec<&str> = row.trim_matches('|').split('|').collect();
                if cells.iter().all(|c| {
                    let c = c.trim();
                    !c.is_empty() && c.chars().all(|ch| ch == '-' || ch == ':')
                }) {
                    continue; // the header separator row
                }
                let tag = if idx == 0 { "th" } else { "td" };
                out.push_str("<tr>");
                for cell in cells {
                    out.push_str(&format!("<{tag}>{}</{tag}>", inline(cell.trim())));
                }
                out.push_str("</tr>\n");
            }
            out.push_str("</table>\n");
            table.clear();
        }
    };

    for line in body.lines() {
        if let Some(block) = &mut code {
            if line.trim_start().starts_with("```") {
                out.push_str(&format!(
                    "<pre><code>{}</code></pre>\n",
                    escape(&block.join("\n"))
                ));
                code = None;
            } else {
                block.push(line.to_string());
            }
            continue;
        }
        let trimmed = line.trim();
        if trimmed.starts_with("```") {
            flush_paragraph(&mut out, &mut paragraph);
            flush_list(&mut out, &mut list);
            flush_table(&mut out, &mut table);
            code = Some(Vec::new());
        } else if trimmed.starts_with("<!--") && trimmed.ends_with("-->") {
            // Directives and comments never reach the published page.
        } else if let Some(rest) = heading(trimmed) {
            flush_paragraph(&mut out, &mut paragraph);
            flush_list(&mut out, &mut list);
            flush_table(&mut out, &mut table);
            let (level, text) = rest;
            out.push_str(&format!("<h{level}>{}</h{level}>\n", inline(text)));
        } else if let Some(item) = trimmed.strip_prefix("- ") {
            flush_paragraph(&mut out, &mut paragraph);
            flush_table(&mut out, &mut table);
            list.push(item.to_string());
        } else if trimmed.starts_with('|') {
            flush_paragraph(&mut out, &mut paragraph);
            flush_list(&mut out, &mut list);
            table.push(trimmed.to_string());
        } else if trimmed.is_empty() {
            flush_paragraph(&mut out, &mut paragraph);
            flush_list(&mut out, &mut list);
            flush_table(&mut out, &mut table);
        } else {
            flush_list(&mut out, &mut list);
            flush_table(&mut out, &mut table);
            paragraph.push(trimmed.to_string());
        }
    }
    flush_paragraph(&mut out, &mut paragraph);
    flush_list(&mut out, &mut list);
    flush_table(&mut out, &mut table);
    if let Some(block) = code {
        // An unterminated fence still publishes its content, escaped.
        out.push_str(&format!(
            "<pre><code>{}</code></pre>\n",
            escape(&block.join("\n"))
        ));
    }
    out
}

fn heading(line: &str) -> Option<(usize, &str)> {
    let level = line.chars().take_while(|c| *c == '#').count();
    if (1..=6).contains(&level)
        && let Some(text) = line[level..].strip_prefix(' ')
    {
        return Some((level, text));
    }
    None
}

/// Inline spans over escaped text: code first (its content is literal),
/// then links, bold, italic. Relative `.md` targets become `.html`.
fn inline(text: &str) -> String {
    let escaped = escape(text);
    let mut out = String::new();
    let mut rest = escaped.as_str();
    while let Some(start) = rest.find('`') {
        let (before, after) = rest.split_at(start);
        out.push_str(&spans(before));
        match after[1..].find('`') {
            Some(end) => {
                out.push_str(&format!("<code>{}</code>", &after[1..end + 1]));
                rest = &after[end + 2..];
            }
            None => {
                out.push_str(&spans(after));
                return out;
            }
        }
    }
    out.push_str(&spans(rest));
    out
}

/// Links, bold, italic — applied outside code spans only.
fn spans(text: &str) -> String {
    let mut out = String::new();
    let mut rest = text;
    // [text](target)
    while let Some(start) = rest.find('[') {
        let Some(mid) = rest[start..].find("](") else {
            break;
        };
        let Some(end) = rest[start + mid..].find(')') else {
            break;
        };
        let label = &rest[start + 1..start + mid];
        let target = &rest[start + mid + 2..start + mid + end];
        let target = if target.ends_with(".md") && !target.contains("://") {
            format!("{}html", &target[..target.len() - 2])
        } else {
            target.to_string()
        };
        out.push_str(&emphasis(&rest[..start]));
        out.push_str(&format!("<a href=\"{target}\">{}</a>", emphasis(label)));
        rest = &rest[start + mid + end + 1..];
    }
    out.push_str(&emphasis(rest));
    out
}

fn emphasis(text: &str) -> String {
    let mut out = text.to_string();
    for (marker, tag) in [("**", "strong"), ("*", "em")] {
        while let Some(start) = out.find(marker) {
            let Some(len) = out[start + marker.len()..].find(marker) else {
                break;
            };
            let inner = out[start + marker.len()..start + marker.len() + len].to_string();
            out.replace_range(
                start..start + 2 * marker.len() + len,
                &format!("<{tag}>{inner}</{tag}>"),
            );
        }
    }
    out
}

fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

#[cfg(test)]
mod tests {
    use super::render_markdown;

    // arqix:no-requirement
    #[test]
    fn the_renderer_covers_the_corpus_subset() {
        let html = render_markdown(
            "## Title\n\nOne `code` and [a link](other.md).\n\n- item **bold**\n\n| H |\n| --- |\n| c |\n\n```rust\nlet x = 1 < 2;\n```\n<!-- arqix:include x.md -->\n",
        );
        assert!(html.contains("<h2>Title</h2>"));
        assert!(html.contains("<code>code</code>"));
        assert!(html.contains("<a href=\"other.html\">a link</a>"));
        assert!(html.contains("<li>item <strong>bold</strong></li>"));
        assert!(html.contains("<th>H</th>") && html.contains("<td>c</td>"));
        assert!(html.contains("let x = 1 &lt; 2;"));
        assert!(!html.contains("arqix:include"), "directives never publish");
    }
}
