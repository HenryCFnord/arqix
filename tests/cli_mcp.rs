//! Command contract: `mcp serve` — owned by the MCP Server (arc42
//! chapter 5): the required MCP subset over stdio, transport separated
//! from tool logic (ADR-0014). The scripted sessions below are the
//! executable protocol fixture the ADR makes responsible for spec
//! conformance.

mod common;

use serde_json::Value;
use std::io::Write;
use std::process::{Command, Stdio};

/// Drive one scripted MCP session: write `requests` (one JSON-RPC message
/// per line) to the server's stdin, close it, and parse every stdout line
/// as JSON. The server must exit 0 on end-of-input.
fn session(dir: &std::path::Path, requests: &[&str]) -> Vec<Value> {
    let mut child = Command::new(env!("CARGO_BIN_EXE_arqix"))
        .current_dir(dir)
        .args(["mcp", "serve"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to start mcp serve");

    let mut stdin = child.stdin.take().expect("stdin piped");
    for request in requests {
        stdin
            .write_all(format!("{request}\n").as_bytes())
            .expect("failed to write request");
    }
    drop(stdin);

    let output = child.wait_with_output().expect("mcp serve did not exit");
    assert_eq!(
        output.status.code(),
        Some(0),
        "the server exits cleanly on end-of-input, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|line| serde_json::from_str(line).expect("every response line is JSON"))
        .collect()
}

const INITIALIZE: &str = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"fixture-client","version":"0.0.0"}}}"#;
const INITIALIZED: &str = r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#;

// arqix:verifies REQ-05-01-12-01
#[test]
fn mcp_serve_speaks_jsonrpc_over_stdio() {
    let responses = session(
        &common::fixture("minimal"),
        &[
            INITIALIZE,
            INITIALIZED,
            r#"{"jsonrpc":"2.0","id":2,"method":"no/such/method"}"#,
        ],
    );

    // The notification gets no response: exactly two response lines.
    assert_eq!(
        responses.len(),
        2,
        "one response per request, none per notification: {responses:?}"
    );

    let init = &responses[0];
    assert_eq!(init["jsonrpc"], "2.0");
    assert_eq!(init["id"], 1);
    assert!(
        init["result"]["protocolVersion"].is_string(),
        "initialize negotiates a protocol version: {init}"
    );
    assert_eq!(
        init["result"]["serverInfo"]["name"], "arqix",
        "the server identifies itself: {init}"
    );

    let unknown = &responses[1];
    assert_eq!(unknown["id"], 2);
    assert_eq!(
        unknown["error"]["code"], -32601,
        "an unknown method is the JSON-RPC method-not-found error: {unknown}"
    );
}

// arqix:verifies REQ-05-01-12-02
#[test]
fn mcp_serve_exposes_search_read_and_list_tools() {
    let responses = session(
        &common::fixture("minimal"),
        &[
            INITIALIZE,
            INITIALIZED,
            r#"{"jsonrpc":"2.0","id":2,"method":"tools/list"}"#,
            r#"{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"list","arguments":{}}}"#,
            r#"{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"read","arguments":{"id":"REQ-99-99-99-01"}}}"#,
            r#"{"jsonrpc":"2.0","id":5,"method":"tools/call","params":{"name":"search","arguments":{"query":"fixture"}}}"#,
        ],
    );
    assert_eq!(responses.len(), 5, "responses: {responses:?}");

    let names: Vec<&str> = responses[1]["result"]["tools"]
        .as_array()
        .expect("tools/list returns a tool array")
        .iter()
        .filter_map(|t| t["name"].as_str())
        .collect();
    for tool in ["search", "read", "list"] {
        assert!(names.contains(&tool), "tool {tool} is declared: {names:?}");
    }
    assert!(
        responses[1]["result"]["tools"]
            .as_array()
            .unwrap()
            .iter()
            .all(|t| t["inputSchema"]["type"] == "object"),
        "every tool declares an input schema"
    );

    // Every call answers with MCP text content carrying the same JSON the
    // CLI surface produces.
    for (response, expected) in [
        (&responses[2], "REQ-99-99-99-01"),
        (&responses[3], "fixture"),
        (&responses[4], "REQ-99-99-99-01"),
    ] {
        let text = response["result"]["content"][0]["text"]
            .as_str()
            .unwrap_or_else(|| panic!("tool results are text content: {response}"));
        assert!(
            text.contains(expected),
            "expected {expected} in tool result: {text}"
        );
    }

    // An unknown tool is an invalid-params error, not a crash.
    let responses = session(
        &common::fixture("minimal"),
        &[
            INITIALIZE,
            r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"no-such-tool","arguments":{}}}"#,
        ],
    );
    assert_eq!(responses[1]["error"]["code"], -32602, "{responses:?}");
}
