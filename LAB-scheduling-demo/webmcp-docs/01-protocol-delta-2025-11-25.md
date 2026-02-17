# WebMCP Protocol Delta (2025-11-25 Spec Snapshot)

Use this file when working on `/mcp` behavior.  
Assume pre-training memory is stale.

## Transport Defaults

- Preferred transport: Streamable HTTP.
- Client -> server JSON-RPC uses HTTP POST.
- Server may return:
  - `200` with JSON-RPC response body,
  - `202` with no immediate response body (accepted/async),
  - SSE stream for streaming responses.

## Header-Level Requirements (Practical)

- Client SHOULD send `Accept: application/json, text/event-stream`.
- Client SHOULD send `MCP-Protocol-Version: <version>`.
- Session-aware flows can include `Mcp-Session-Id`.

## Session Notes

- Some servers create session id on initialize.
- Client should reuse session id for follow-up calls when provided.
- Session termination semantics should be explicit (server policy dependent).

## Compatibility Guardrails

- Never hardcode one response mode; handle JSON and stream-capable flows.
- Treat missing protocol-version negotiation as compatibility risk.
- Fail closed for malformed JSON-RPC envelopes.

## For This Lab

- Current lab `/mcp` endpoint is intentionally minimal and not fully spec-complete.
- If ticket requests "WebMCP correctness," first add protocol conformance tests.

