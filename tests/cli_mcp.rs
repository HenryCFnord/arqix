//! Command contract: `mcp serve` — owned by the MCP Server (arc42
//! chapter 5): the required MCP subset over stdio, transport separated
//! from tool logic (ADR-0014). The scripted sessions below are the
//! executable protocol fixture the ADR makes responsible for spec
//! conformance.

mod common;

use common::scratch_copy;
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
    for tool in ["search", "read", "list", "trace"] {
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

/// The text payload of the tool-call response at `index`.
fn tool_text(responses: &[Value], index: usize) -> &str {
    responses[index]["result"]["content"][0]["text"]
        .as_str()
        .unwrap_or_else(|| panic!("tool results are text content: {}", responses[index]))
}

// arqix:verifies REQ-05-01-12-02
#[test]
fn mcp_search_supports_kind_and_path_filters() {
    let repo = scratch_copy("minimal", "mcp_search_supports_kind_and_path_filters");
    std::fs::create_dir_all(repo.join("docs/guide")).unwrap();
    std::fs::write(
        repo.join("docs/guide/unit-guide-note.md"),
        "---\nid: unit-guide-note\ntitle: Guide Note\niri: arqix:units/unit-guide-note\nrdf:\n  type:\n    - arqix:classes/unit\n---\n\n## Guide Note\n\nThis fixture note lives under the guide path.\n",
    )
    .unwrap();

    let responses = session(
        &repo,
        &[
            INITIALIZE,
            INITIALIZED,
            r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"search","arguments":{"query":"fixture"}}}"#,
            r#"{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"search","arguments":{"query":"fixture","kind":"unit"}}}"#,
            r#"{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"search","arguments":{"query":"fixture","path":"docs/guide"}}}"#,
            r#"{"jsonrpc":"2.0","id":5,"method":"tools/call","params":{"name":"search","arguments":{"query":"fixture","kind":"unit","path":"docs/REQ"}}}"#,
        ],
    );
    assert_eq!(responses.len(), 5, "responses: {responses:?}");

    // Unfiltered, the query hits both documents.
    let unfiltered = tool_text(&responses, 1);
    assert!(
        unfiltered.contains("unit-guide-note") && unfiltered.contains("REQ-99-99-99-01"),
        "the unfiltered search hits both documents: {unfiltered}"
    );
    // The kind filter drops the requirement hit and keeps the unit hit.
    let by_kind = tool_text(&responses, 2);
    assert!(
        by_kind.contains("unit-guide-note") && !by_kind.contains("REQ-99-99-99-01"),
        "kind=unit keeps only the unit document: {by_kind}"
    );
    // The path prefix filter drops everything outside the prefix.
    let by_path = tool_text(&responses, 3);
    assert!(
        by_path.contains("unit-guide-note") && !by_path.contains("REQ-99-99-99-01"),
        "path=docs/guide keeps only files under the prefix: {by_path}"
    );
    // Filters combine: a kind that never occurs under the path yields nothing.
    let combined = tool_text(&responses, 4);
    assert!(
        !combined.contains("unit-guide-note") && !combined.contains("REQ-99-99-99-01"),
        "contradictory filters yield no hits: {combined}"
    );
}

// arqix:verifies REQ-05-01-12-02
#[test]
fn mcp_list_supports_a_lifecycle_filter() {
    let repo = scratch_copy("minimal", "mcp_list_supports_a_lifecycle_filter");
    std::fs::write(
        repo.join("docs/unit-draft-note.md"),
        "---\nid: unit-draft-note\ntitle: Draft Note\niri: arqix:units/unit-draft-note\nmeta:\n  lifecycle-status: draft\n---\n\n## Draft Note\n",
    )
    .unwrap();
    std::fs::write(
        repo.join("docs/unit-plain-note.md"),
        "---\nid: unit-plain-note\ntitle: Plain Note\niri: arqix:units/unit-plain-note\n---\n\n## Plain Note\n",
    )
    .unwrap();

    let responses = session(
        &repo,
        &[
            INITIALIZE,
            INITIALIZED,
            r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"list","arguments":{}}}"#,
            r#"{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"list","arguments":{"lifecycle":"active"}}}"#,
            r#"{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"list","arguments":{"lifecycle":"draft"}}}"#,
        ],
    );
    assert_eq!(responses.len(), 4, "responses: {responses:?}");

    let unfiltered = tool_text(&responses, 1);
    assert!(
        unfiltered.contains("REQ-99-99-99-01")
            && unfiltered.contains("unit-draft-note")
            && unfiltered.contains("unit-plain-note"),
        "the unfiltered catalog lists all documents: {unfiltered}"
    );
    // The fixture requirement declares lifecycle-status: active; a document
    // without a lifecycle line never matches a lifecycle filter.
    let active = tool_text(&responses, 2);
    assert!(
        active.contains("REQ-99-99-99-01")
            && !active.contains("unit-draft-note")
            && !active.contains("unit-plain-note"),
        "lifecycle=active keeps only documents declaring it: {active}"
    );
    let draft = tool_text(&responses, 3);
    assert!(
        draft.contains("unit-draft-note")
            && !draft.contains("REQ-99-99-99-01")
            && !draft.contains("unit-plain-note"),
        "lifecycle=draft keeps only documents declaring it: {draft}"
    );
}

// arqix:verifies REQ-05-01-12-02
#[test]
fn mcp_trace_answers_coverage_for_a_requirement_and_a_story() {
    let repo = scratch_copy(
        "minimal",
        "mcp_trace_answers_coverage_for_a_requirement_and_a_story",
    );
    std::fs::create_dir_all(repo.join("tests")).unwrap();
    // Assembled from pieces so the marker gate never reads this literal
    // itself as a marker line of this file.
    std::fs::write(
        repo.join("tests/probe.rs"),
        format!("// arqix:{} REQ-99-99-99-01\nfn probe() {{}}\n", "verifies"),
    )
    .unwrap();
    // A story with a requirement derived from it via a declared triple, so
    // the story scope resolves to that requirement (and nothing else).
    std::fs::write(
        repo.join("docs/US-42-01-01-story.md"),
        "---\nid: US-42-01-01\ntitle: Scoped Story\niri: arqix:user-stories/us-42-01-01\nrdf:\n  type:\n    - arqix:classes/user-story\n---\n\n## Scoped Story\n",
    )
    .unwrap();
    std::fs::write(
        repo.join("docs/REQ-42-01-01-01-scoped.md"),
        "---\nid: REQ-42-01-01-01\ntitle: Scoped Requirement\niri: arqix:requirements/req-42-01-01-01\nrdf:\n  type:\n    - arqix:classes/functional-requirement\ntriples:\n  - predicate: arqix:properties/derived-from\n    object:\n      - arqix:user-stories/us-42-01-01\n---\n\n## Requirement\n\nThe system SHALL be in scope.\n",
    )
    .unwrap();

    let responses = session(
        &repo,
        &[
            INITIALIZE,
            INITIALIZED,
            r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"trace","arguments":{"id":"REQ-99-99-99-01"}}}"#,
            r#"{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"trace","arguments":{"id":"US-42-01-01"}}}"#,
        ],
    );
    assert_eq!(responses.len(), 3, "responses: {responses:?}");

    // A requirement id answers with its coverage row: status and the
    // verifying marker location.
    let requirement = tool_text(&responses, 1);
    assert!(
        requirement.contains("verified") && requirement.contains("tests/probe.rs:1"),
        "the verified requirement carries its status and marker location: {requirement}"
    );
    // A story id answers with the rows of the requirements derived from it.
    let story = tool_text(&responses, 2);
    assert!(
        story.contains("REQ-42-01-01-01") && story.contains("uncovered"),
        "the story scope covers its derived requirement: {story}"
    );
    assert!(
        !story.contains("REQ-99-99-99-01"),
        "out-of-scope requirements stay out: {story}"
    );
}

// arqix:verifies REQ-05-01-12-02
#[test]
fn mcp_trace_reports_an_unknown_id_as_a_tool_error() {
    let responses = session(
        &common::fixture("minimal"),
        &[
            INITIALIZE,
            INITIALIZED,
            r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"trace","arguments":{"id":"REQ-00-00-99-99"}}}"#,
        ],
    );
    let result = &responses[1]["result"];
    assert_eq!(
        result["isError"], true,
        "an unknown id is a tool-level error result: {result}"
    );
    assert!(
        result["content"][0]["text"]
            .as_str()
            .is_some_and(|text| text.contains("REQ-00-00-99-99")),
        "the error names the missing id: {result}"
    );
}
