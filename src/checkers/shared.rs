//! Helpers shared by the corpus checkers: the finding shape and the
//! Python-compatible formatting/IO the retired oracles pinned. The trace
//! engine keeps its own `py_repr` — its oracle lineage differs byte-wise.

use std::path::Path;

pub(crate) struct Finding {
    pub(crate) path: String,
    pub(crate) rule: &'static str,
    pub(crate) level: &'static str,
    pub(crate) message: String,
}

impl Finding {
    pub(crate) fn error(path: &str, rule: &'static str, message: String) -> Self {
        Finding {
            path: path.to_string(),
            rule,
            level: "error",
            message,
        }
    }

    pub(crate) fn warning(path: &str, rule: &'static str, message: String) -> Self {
        Finding {
            path: path.to_string(),
            rule,
            level: "warning",
            message,
        }
    }
}

pub(crate) fn py_repr(s: &str) -> String {
    let has_single = s.contains('\'');
    let has_double = s.contains('"');
    let quote = if has_single && !has_double { '"' } else { '\'' };
    let mut out = String::with_capacity(s.len() + 2);
    out.push(quote);
    for c in s.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '\t' => out.push_str("\\t"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            c if c == quote => {
                out.push('\\');
                out.push(c);
            }
            c if (c as u32) < 0x20 || (c as u32) == 0x7f => {
                out.push_str(&format!("\\x{:02x}", c as u32));
            }
            c => out.push(c),
        }
    }
    out.push(quote);
    out
}

pub(crate) fn repr_opt(value: Option<&str>) -> String {
    match value {
        Some(s) => py_repr(s),
        None => "None".to_string(),
    }
}

pub(crate) fn py_list_repr(items: &[String]) -> String {
    let inner: Vec<String> = items.iter().map(|s| py_repr(s)).collect();
    format!("[{}]", inner.join(", "))
}

pub(crate) fn read_universal(path: &str) -> String {
    let raw = std::fs::read_to_string(path).unwrap_or_default();
    raw.replace("\r\n", "\n").replace('\r', "\n")
}

pub(crate) fn basename(path: &str) -> String {
    path.rsplit('/').next().unwrap_or(path).to_string()
}

pub(crate) fn collect_md(dir: &Path, out: &mut Vec<String>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let p = entry.path();
        if p.is_dir() {
            collect_md(&p, out);
        } else if p.extension().and_then(|e| e.to_str()) == Some("md") {
            out.push(p.to_string_lossy().replace('\\', "/"));
        }
    }
}
