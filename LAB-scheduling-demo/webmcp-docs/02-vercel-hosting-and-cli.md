# Vercel MCP Hosting + CLI (Compressed)

## Next.js Route Pattern

- Use `createMcpHandler((server) => { ... })`.
- Export handler from route file (ex: `app/api/mcp/route.ts`).
- Keep tool schemas strict and small.

## Local Client Wiring

- Cursor-style local config pattern:
  - `.cursor/mcp.json` with server URL + optional headers/env interpolation.
- Keep auth material in env; do not commit secrets.

## Deploy + Inspect Loop

```bash
vercel
vercel mcp inspect https://<deployment-url>/api/mcp
vercel mcp list-tools https://<deployment-url>/api/mcp
```

## Reliability Heuristics

- Validate `tools/list` and `tools/call` on every deployment.
- Add a smoke check for auth failure path (wrong token/key).
- Keep at least one deterministic tool for contract health checks.

## Lab Mapping

- Lab endpoint is `/mcp` (not `/api/mcp`) for simplicity.
- If you migrate to Next route layout later, preserve JSON-RPC payload compatibility.

