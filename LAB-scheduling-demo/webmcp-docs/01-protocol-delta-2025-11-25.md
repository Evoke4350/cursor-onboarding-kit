# WebMCP // Protocol Contract (Compressed)

`scope`: `/mcp` behavior and client/server envelope expectations.

## Envelope

- JSON-RPC: `2.0`
- request fields: `jsonrpc`, `id`, `method`, `params`
- lab methods: `tools/list`, `tools/call`

## Transport

- HTTP `POST /mcp`
- request content type: `application/json`
- response modes to handle:
  - `200` JSON-RPC payload
  - `202` accepted/no immediate body
  - stream-capable response paths

## Header Expectations

- required (lab): `x-agent-key`
- recommended: `Accept: application/json, text/event-stream`
- recommended: `MCP-Protocol-Version`
- optional: `Mcp-Session-Id` for session continuity

## Error Model

- `-32700`: parse error
- `-32600`: invalid request
- `-32601`: method not found
- `-32602`: invalid params / unknown tool
- `-32001`: unauthorized (lab-specific)

## Change Guardrail

- do not assume full spec conformance from this lab runtime
- add protocol conformance tests before strict compatibility changes
