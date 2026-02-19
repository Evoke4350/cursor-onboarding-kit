# LAB-changelog-viewer

A git-log-style viewer for Cursor release notes. Fetches from the GitHub Releases API, caches locally, and renders in a dark terminal aesthetic you'd actually want to look at.

Part of the [Agentic Blackboard](../README.md) onboarding kit.

## Quick start

```bash
cd LAB-changelog-viewer
npm install
npm run dev
# → http://localhost:4200
```

Set `GITHUB_TOKEN` to avoid the 60 req/hr unauthenticated rate limit:

```bash
GITHUB_TOKEN=ghp_xxx npm run dev
```

## Dump to markdown

Writes all releases as individual `.md` files with YAML frontmatter — useful for LLM analysis sessions:

```bash
npm run dump
# → output/v0.48.0.md, output/v0.47.3.md, ...
```

Each file looks like:

```
---
tag: v0.48.0
name: "Bug fixes and stability improvements"
published_at: 2025-01-15T10:00:00Z
prerelease: false
url: https://github.com/getcursor/cursor/releases/tag/v0.48.0
---

# v0.48.0 — Bug fixes and stability improvements

<release body>
```

Point an agent at `output/` and ask it to find trends, compare releases, or summarize changes in a given area.

## Tests

```bash
npm test
```

Tests cover `isCacheStale` and `buildBodyPreview` — pure functions only. `fetchReleasesFromAPI` requires network and is tested manually.

## API

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/releases` | Index of all releases (no body) |
| GET | `/api/release?tag=2.5` | Full release including markdown body |
| GET | `/api/cache-status` | Cache age, TTL, staleness |
| POST | `/api/cache-bust` | Clear the cache (dev only) |
| POST | `/mcp` | MCP JSON-RPC endpoint (see below) |

## MCP agent integration

The server exposes a JSON-RPC 2.0 MCP endpoint at `/mcp`.

Auth header: `x-agent-key: lab-agent-key` (override with `AGENT_DEV_KEY` env).

### Smoke test

```bash
# List tools
curl -s -X POST http://localhost:4200/mcp \
  -H "x-agent-key: lab-agent-key" \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}' | jq .

# Get a specific release
curl -s -X POST http://localhost:4200/mcp \
  -H "x-agent-key: lab-agent-key" \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"get_changelog","arguments":{"version":"2.5"}}}' | jq .

# Search the changelog
curl -s -X POST http://localhost:4200/mcp \
  -H "x-agent-key: lab-agent-key" \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"search_changelog","arguments":{"query":"tab completion","limit":5}}}' | jq .
```

### Available tools

| Tool | Args | Description |
|------|------|-------------|
| `list_releases` | `includePrerelease?` | All releases, no body text |
| `get_changelog` | `version` (required) | Full body for one release |
| `search_changelog` | `query`, `limit?` | Search tags, names, and bodies |

## Env vars

| Var | Default | Notes |
|-----|---------|-------|
| `PORT` | `4200` | Server port |
| `AGENT_DEV_KEY` | `lab-agent-key` | MCP auth key |
| `CACHE_TTL_MS` | `3600000` | Cache TTL in ms (1 hour) |
| `FETCH_PAGE_LIMIT` | `40` | Pages to fetch from cursor.com/changelog (10 entries/page). `40` = full history (~400 entries, Mar 2023–present). `4` = back to ~Aug 2025 (28 entries). |
| `OUTPUT_DIR` | `./output` | Dump script output directory |

## First run

The first `npm run dev` will fetch all 40 pages from cursor.com (~400 entries, takes ~30s). After that it's served from `.cache/releases.json` until the TTL expires.

To only fetch recent history (faster startup):

```bash
FETCH_PAGE_LIMIT=4 npm run dev  # ~28 entries, back to Aug 2025
```

## Docker

```bash
docker build -t changelog-viewer .
docker run -p 4200:4200 changelog-viewer
```

## UI notes

The `+` and `~` line classifiers in the viewer are a hook for lab exercises. Cursor's actual release notes use standard `- ` bullets — the colored diff aesthetic only activates if notes happen to use those prefixes.

Exercise: modify `renderMarkdown()` in `app.js` to heuristically classify lines by keyword (e.g., lines containing "Added" → green, "Changed" → yellow, "Fixed" → blue).

## Cache

Cache lives at `.cache/releases.json` (gitignored). TTL defaults to 1 hour.

**Planted lab bug:** `POST /api/cache-bust` throws `ENOENT` if `.cache/` doesn't exist yet (e.g., on first run before any fetch). Students fix by wrapping the `unlink` call in a try/catch.
