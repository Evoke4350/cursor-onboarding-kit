# WebMCP // Hosting + CLI Patterns (Compressed)

`scope`: deployment/runtime patterns without vendor-specific assumptions.

## Route Pattern

- expose one deterministic MCP route
- validate JSON-RPC envelope before dispatch
- enforce auth check before tool execution
- return structured JSON-RPC errors

## Tool Contract Pattern

- keep `tools/list` stable and deterministic
- validate `tools/call` argument shape before execution
- keep at least one deterministic tool for health checks

## Client Wiring Pattern

- store endpoint + headers in local client config (`.cursor/mcp.json` style)
- keep tokens/keys in env, not in committed files

## Smoke Workflow

1. start service
2. call `tools/list`
3. call one deterministic tool via `tools/call`
4. verify unauthorized path with wrong/missing key

## Deployment Checklist

- fixed route path
- request body limit
- consistent error format
- log `method`, `id`, status, and auth failures

## Change Guardrail

- separate protocol changes from scheduling-logic changes
