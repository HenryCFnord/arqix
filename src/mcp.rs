//! MCP Server: the required MCP subset over stdio (US-05-01-12) — a
//! blocking JSON-RPC 2.0 loop, one message per line, implemented directly
//! per ADR-0014 (no SDK dependency). Transport handling (`serve`,
//! `handle`) stays separate from the tool logic (`tool_result`), which
//! answers through the same Document Store functions the CLI uses, so
//! both surfaces answer identically (REQ-05-01-12-03).

use serde_json::{Value, json};
use std::io::{BufRead, Write};
use std::process::ExitCode;

/// The protocol version offered when the client's request carries none;
/// a requested version is echoed — the served subset (initialize,
/// tools/list, tools/call) is identical across published versions.
const PROTOCOL_VERSION: &str = "2025-06-18";

// arqix:implements REQ-05-01-12-01
/// `arqix mcp serve` — read one JSON-RPC message per stdin line, write one
/// response per stdout line, exit cleanly on end-of-input.
pub fn serve() -> ExitCode {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    for line in stdin.lock().lines() {
        let Ok(line) = line else { break };
        if line.trim().is_empty() {
            continue;
        }
        if let Some(response) = handle_line(&line) {
            let mut out = stdout.lock();
            if writeln!(out, "{response}")
                .and_then(|()| out.flush())
                .is_err()
            {
                break;
            }
        }
    }
    ExitCode::SUCCESS
}

/// One transport step: parse errors are the JSON-RPC parse error, valid
/// messages go to the dispatcher.
fn handle_line(line: &str) -> Option<Value> {
    match serde_json::from_str::<Value>(line) {
        Ok(request) => handle(&request),
        Err(_) => Some(json!({
            "jsonrpc": "2.0",
            "id": Value::Null,
            "error": { "code": -32700, "message": "parse error" },
        })),
    }
}

/// Dispatch one JSON-RPC message. A message without an id is a
/// notification and gets no response.
fn handle(request: &Value) -> Option<Value> {
    let id = request.get("id").filter(|id| !id.is_null())?.clone();
    let method = request["method"].as_str().unwrap_or("");
    let params = &request["params"];

    let body = match method {
        "initialize" => Ok(json!({
            "protocolVersion": params["protocolVersion"]
                .as_str()
                .unwrap_or(PROTOCOL_VERSION),
            "capabilities": { "tools": {} },
            "serverInfo": {
                "name": "arqix",
                "version": env!("CARGO_PKG_VERSION"),
            },
        })),
        "ping" => Ok(json!({})),
        // arqix:implements REQ-05-01-12-02
        "tools/list" => Ok(json!({ "tools": tool_catalog() })),
        "tools/call" => tool_result(params["name"].as_str().unwrap_or(""), &params["arguments"]),
        _ => Err((-32601, format!("method not found: {method}"))),
    };

    Some(match body {
        Ok(result) => json!({ "jsonrpc": "2.0", "id": id, "result": result }),
        Err((code, message)) => json!({
            "jsonrpc": "2.0",
            "id": id,
            "error": { "code": code, "message": message },
        }),
    })
}

/// The declared tools: names, descriptions, and input schemas
/// (REQ-05-01-12-02).
fn tool_catalog() -> Value {
    json!([
        {
            "name": "search",
            "description": "Full-text search over the documentation corpus; returns matching documents with file and line.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "query": { "type": "string", "description": "Text to search for" },
                    "kind": { "type": "string", "description": "Optional filter: only hits in documents of this catalog kind, e.g. requirement" },
                    "path": { "type": "string", "description": "Optional filter: only hits in files whose repository-relative path starts with this prefix, e.g. docs/en/architecture" },
                },
                "required": ["query"],
            },
        },
        {
            "name": "read",
            "description": "Read one document by its declared id; returns frontmatter identity and the body.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "id": { "type": "string", "description": "Document id, e.g. REQ-01-01-03-01" },
                },
                "required": ["id"],
            },
        },
        {
            "name": "list",
            "description": "The document catalog: id, title, kind, file, and language for every document, optionally filtered by kind and lifecycle.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "kind": { "type": "string", "description": "Optional kind filter, e.g. requirement" },
                    "lifecycle": { "type": "string", "description": "Optional filter: only documents whose declared lifecycle-status equals this value, e.g. active, draft, done, retired; documents without a lifecycle line never match" },
                },
                "required": [],
            },
        },
        {
            "name": "trace",
            "description": "Coverage from the trace graph for one id: a requirement id answers with its coverage row (status verified/planned/uncovered plus the verifying, planned, and implementing marker locations), a story id with the rows of the requirements derived from it.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "id": { "type": "string", "description": "Requirement or story id, e.g. REQ-05-01-12-02 or US-05-01-12" },
                },
                "required": ["id"],
            },
        },
    ])
}

// arqix:implements REQ-05-01-12-03
/// Tool logic, transport-free: every tool answers with the same JSON the
/// CLI surface produces, wrapped as MCP text content. A missing required
/// argument or an unknown tool is the JSON-RPC invalid-params error; a
/// miss inside a tool (no such document) is a tool-level error result.
fn tool_result(name: &str, arguments: &Value) -> Result<Value, (i64, String)> {
    let data = match name {
        "search" => {
            let query = arguments["query"]
                .as_str()
                .ok_or((-32602, "search requires a string 'query'".to_string()))?;
            crate::store::search_json(
                query,
                arguments["kind"].as_str(),
                arguments["path"].as_str(),
            )
        }
        "read" => {
            let id = arguments["id"]
                .as_str()
                .ok_or((-32602, "read requires a string 'id'".to_string()))?;
            match crate::store::read_json(id) {
                Some(doc) => doc,
                None => {
                    return Ok(json!({
                        "content": [{ "type": "text", "text": format!("no document has id {id}") }],
                        "isError": true,
                    }));
                }
            }
        }
        "list" => {
            crate::store::catalog_json(arguments["kind"].as_str(), arguments["lifecycle"].as_str())
        }
        "trace" => {
            let id = arguments["id"]
                .as_str()
                .ok_or((-32602, "trace requires a string 'id'".to_string()))?;
            match crate::trace::trace_json(id) {
                Some(rows) => rows,
                None => {
                    return Ok(json!({
                        "content": [{ "type": "text", "text": format!("no requirement or story has id {id}") }],
                        "isError": true,
                    }));
                }
            }
        }
        _ => return Err((-32602, format!("unknown tool: {name}"))),
    };
    Ok(json!({
        "content": [{
            "type": "text",
            "text": serde_json::to_string_pretty(&data).expect("valid JSON"),
        }],
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    // arqix:verifies REQ-05-01-12-03
    #[test]
    fn tool_logic_answers_without_any_transport() {
        // The separation proof: the tool layer is callable as plain
        // functions, and its payload is exactly the store's data — no
        // stdio, no JSON-RPC envelope involved.
        let result = tool_result("list", &json!({})).expect("list succeeds");
        let payload = result["content"][0]["text"].as_str().expect("text content");
        let expected = serde_json::to_string_pretty(&crate::store::catalog_json(None, None))
            .expect("valid JSON");
        assert_eq!(payload, expected);
    }

    // arqix:no-requirement — JSON-RPC plumbing details below the requirement level
    #[test]
    fn notifications_and_null_ids_get_no_response() {
        assert_eq!(
            handle(&json!({ "jsonrpc": "2.0", "method": "notifications/initialized" })),
            None
        );
        assert_eq!(
            handle(&json!({ "jsonrpc": "2.0", "id": null, "method": "ping" })),
            None
        );
    }

    // arqix:no-requirement — JSON-RPC plumbing details below the requirement level
    #[test]
    fn a_parse_error_is_reported_with_a_null_id() {
        let response = handle_line("not json").expect("parse errors are answered");
        assert_eq!(response["error"]["code"], -32700);
        assert!(response["id"].is_null());
    }
}
