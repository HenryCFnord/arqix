---
id: ADR-0014
title: MCP Transport
slug: mcp-transport
iri: arqix:adrs/adr-0014
rdf:
  type:
    - arqix:classes/adr
triples:
  - predicate: arqix:properties/guides-implementation-of
    object:
      - arqix:requirements/req-05-01-12-01
      - arqix:requirements/req-05-01-12-02
      - arqix:requirements/req-05-01-12-03
properties:
  decision-status: accepted
external-references:
  - type: specification
    label: Model Context Protocol specification
    uri: https://modelcontextprotocol.io/specification
  - type: implementation
    label: Official MCP Rust SDK (rmcp)
    uri: https://github.com/modelcontextprotocol/rust-sdk
meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-12
  updated: 2026-07-12
  lang: en
  translation-of:
  generated: false
---

## MCP Transport

### Context

`arqix mcp serve` exposes the corpus to agents over the Model Context Protocol (US-05-01-12): stdio transport, at least the tools `search`, `read`, and `list`, transport separated from tool logic (REQ-05-01-12-01..03).
The protocol subset this requires is small: JSON-RPC 2.0 messages, one per line on stdin/stdout, with the methods `initialize`, `tools/list`, and `tools/call`, plus tolerance for client notifications.
The implementation question is whether to take the official Rust SDK (`rmcp`) as a dependency or to implement the subset directly.
The decision repeats a standing tension: arqix keeps its dependency tree deliberately small (six crates, each individually justified — the same bar the `regex` crate had to pass in ADR-0012's slice), and the binary is synchronous and deterministic throughout, while `rmcp` is async and brings the tokio runtime and its ecosystem as transitive dependencies.

### Decision

**`mcp serve` implements the required MCP subset directly over stdio; no SDK dependency.**

- The transport is a blocking loop: read a line from stdin, parse with `serde_json` (already in the tree), dispatch, write one line to stdout.
- The tool logic is the existing Document Store surface (`doc search/read/list`) exposed as data-producing functions; the transport layer only translates between JSON-RPC messages and those calls (REQ-05-01-12-03).
- Spec conformance for the subset is owned by the integration tests: a scripted client session over stdio (initialize handshake, tool discovery, tool execution, notification tolerance, unknown-method errors) is the executable protocol fixture.
- The decision is explicitly sized to the current requirement.
  The revisit trigger is a requirement the subset cannot carry: an HTTP or SSE transport, MCP resources or prompts, or server-initiated messages.
  Because transport and tool logic are separated by requirement, adopting the SDK later replaces only the loop, not the tools.

### Alternatives Considered

- **Official `rmcp` SDK:** rejected for now — maintained spec conformance and future transports are real benefits, but the price is the tokio runtime and dozens of transitive crates inside an otherwise synchronous six-crate binary: longer builds, a larger supply-chain surface on a published crate, and an async island whose only job would be to call synchronous store functions.
- **A lighter third-party MCP crate:** rejected — the ecosystem below the official SDK is young and volatile; pinning arqix to an unofficial protocol implementation trades the maintenance burden for a less accountable one.

### Consequences

- The dependency tree stays at six crates; `mcp serve` compiles and behaves like every other subcommand, synchronous and deterministic.
- arqix owns spec conformance for the implemented subset: protocol-version negotiation and message shapes must be tracked against the MCP specification by hand, and the scripted-session tests are the guard that changes stay conformant.
- Clients that require capabilities beyond the subset (resources, prompts, non-stdio transports) are out of scope until a requirement brings them in — at which point this decision is revisited with the SDK as the default candidate.
