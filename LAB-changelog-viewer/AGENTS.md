# AGENTS.md — LAB-changelog-viewer

Agent instructions for working in this lab directory.

## What this is

A self-contained Cursor changelog viewer built on the same pattern as `LAB-scheduling-demo`. The server fetches Cursor releases from the GitHub API, caches them locally, and serves a git-log-style web UI.

## Server

Start: `npm run dev` → `http://localhost:4200`

MCP endpoint: `POST /mcp` with header `x-agent-key: lab-agent-key`

## Key files

| File | Purpose |
|------|---------|
| `src/github.ts` | GitHub API fetch, cache read/write |
| `src/types.ts` | TypeScript types |
| `ui/server.ts` | HTTP server, API routes, MCP dispatch |
| `ui/public/app.js` | Frontend: fetch, render, filter |
| `scripts/dump-markdown.ts` | CLI: write releases to `output/*.md` |

## Triage workflow

Standard loop: discuss → scope → fix → verify → package.

1. Read the symptom (UI bug, API error, test failure)
2. Identify the relevant file (github.ts, server.ts, or app.js)
3. Read the function — don't guess
4. Fix the minimal thing
5. Verify: run test, curl endpoint, or reload browser

## Known planted bugs

Two intentional bugs for lab exercises:

1. **`/api/cache-bust` ENOENT** — `unlink()` throws if `.cache/` dir doesn't exist. Fix: try/catch around `unlink`.
2. **Docker missing GITHUB_TOKEN** — Dockerfile omits the token; students hit rate limits. Fix: pass `-e GITHUB_TOKEN=...` at runtime.

## MCP tools available

- `list_releases` — all release metadata, no body
- `get_changelog(version)` — full body for a version (e.g., `"v0.48.0"`)
- `search_changelog(query, limit?)` — full-text search across all releases

## Tests

```bash
npm test   # node:test + node:assert, pure-function tests only
```

## Useful curl commands

```bash
# Check cache status
curl -s http://localhost:4200/api/cache-status | jq .

# Get a specific release
curl -s "http://localhost:4200/api/release?tag=v0.48.0" | jq .body

# Search via MCP
curl -s -X POST http://localhost:4200/mcp \
  -H "x-agent-key: lab-agent-key" \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"search_changelog","arguments":{"query":"composer"}}}' | jq .
```
