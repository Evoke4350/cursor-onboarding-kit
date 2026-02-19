import {
  createServer,
  type IncomingMessage,
  type ServerResponse,
} from "node:http";
import { readFile, unlink } from "node:fs/promises";
import { fileURLToPath } from "node:url";
import { join } from "node:path";

import type { CursorRelease, ReleaseIndexEntry } from "../src/types.ts";
import {
  getReleases,
  loadCache,
  isCacheStale,
  buildBodyPreview,
  CACHE_FILE,
} from "../src/cursor.ts";

const AGENT_DEV_KEY = process.env.AGENT_DEV_KEY ?? "lab-agent-key";

const uiDir = fileURLToPath(new URL(".", import.meta.url));
const publicDir = join(uiDir, "public");

// ── types ──────────────────────────────────────────────────────────

type JsonRpcRequest = {
  jsonrpc?: string;
  id?: string | number | null;
  method?: string;
  params?: Record<string, unknown>;
};

// ── helpers ────────────────────────────────────────────────────────

function sendJson(
  res: ServerResponse<IncomingMessage>,
  status: number,
  payload: unknown,
): void {
  res.writeHead(status, { "content-type": "application/json; charset=utf-8" });
  res.end(JSON.stringify(payload, null, 2));
}

function sendJsonRpcResult(
  res: ServerResponse<IncomingMessage>,
  id: JsonRpcRequest["id"],
  result: unknown,
): void {
  sendJson(res, 200, { jsonrpc: "2.0", id: id ?? null, result });
}

function sendJsonRpcError(
  res: ServerResponse<IncomingMessage>,
  id: JsonRpcRequest["id"],
  code: number,
  message: string,
): void {
  sendJson(res, 200, { jsonrpc: "2.0", id: id ?? null, error: { code, message } });
}

async function readRequestBody(req: IncomingMessage): Promise<string> {
  return new Promise<string>((resolve, reject) => {
    let data = "";
    req.setEncoding("utf8");
    req.on("data", (chunk) => {
      data += chunk;
      if (data.length > 1024 * 1024) reject(new Error("Request body too large"));
    });
    req.on("end", () => resolve(data));
    req.on("error", reject);
  });
}

function isAgentAuthorized(req: IncomingMessage): boolean {
  const headerValue = req.headers["x-agent-key"];
  if (Array.isArray(headerValue)) return headerValue.includes(AGENT_DEV_KEY);
  return headerValue === AGENT_DEV_KEY;
}

function toIndexEntry(r: CursorRelease): ReleaseIndexEntry {
  return {
    tag_name: r.tag_name,
    name: r.name ?? r.tag_name,
    published_at: r.published_at,
    prerelease: r.prerelease,
    bodyPreview: buildBodyPreview(r.body),
  };
}

function formatReleaseAsText(release: CursorRelease): string {
  return `# ${release.tag_name}\nDate: ${release.published_at.slice(0, 10)}\n\n${release.body ?? "(no release notes)"}`;
}

function extractSnippet(body: string | null, query: string, contextChars = 120): string {
  const text = body ?? "";
  const idx = text.toLowerCase().indexOf(query.toLowerCase());
  if (idx === -1) return "(not found in body)";
  const start = Math.max(0, idx - contextChars / 2);
  const end = Math.min(text.length, idx + contextChars / 2);
  return (start > 0 ? "…" : "") + text.slice(start, end) + (end < text.length ? "…" : "");
}

// ── static routes ──────────────────────────────────────────────────

const staticRoutes: Record<string, { file: string; contentType: string }> = {
  "/": { file: "index.html", contentType: "text/html; charset=utf-8" },
  "/app.js": { file: "app.js", contentType: "text/javascript; charset=utf-8" },
  "/styles.css": { file: "styles.css", contentType: "text/css; charset=utf-8" },
};

// ── server ─────────────────────────────────────────────────────────

const server = createServer(async (req, res) => {
  try {
    const url = new URL(req.url ?? "/", "http://localhost");

    // ── MCP JSON-RPC ────────────────────────────────────────────────
    if (url.pathname === "/mcp") {
      if (!isAgentAuthorized(req)) {
        sendJsonRpcError(res, null, -32001, "Unauthorized agent key");
        return;
      }
      if (req.method !== "POST") {
        sendJsonRpcError(res, null, -32600, "Use POST for JSON-RPC");
        return;
      }

      const rawBody = await readRequestBody(req);
      let rpc: JsonRpcRequest;
      try {
        rpc = JSON.parse(rawBody || "{}") as JsonRpcRequest;
      } catch {
        sendJsonRpcError(res, null, -32700, "Parse error");
        return;
      }

      if (rpc.jsonrpc !== "2.0" || typeof rpc.method !== "string") {
        sendJsonRpcError(res, rpc.id, -32600, "Invalid Request");
        return;
      }

      if (rpc.method === "tools/list") {
        sendJsonRpcResult(res, rpc.id, {
          tools: [
            {
              name: "list_releases",
              description:
                "List all known Cursor releases with metadata (no full body text).",
              inputSchema: {
                type: "object",
                properties: {
                  includePrerelease: {
                    type: "boolean",
                    description: "Include prerelease versions (default: false).",
                  },
                },
              },
            },
            {
              name: "get_changelog",
              description:
                "Fetch the full release notes for a specific Cursor version.",
              inputSchema: {
                type: "object",
                properties: {
                  version: {
                    type: "string",
                    description: "Version tag, e.g. 'v0.48.0'. The 'v' prefix is required.",
                  },
                },
                required: ["version"],
              },
            },
            {
              name: "search_changelog",
              description:
                "Search across all Cursor release notes for a keyword or phrase.",
              inputSchema: {
                type: "object",
                properties: {
                  query: {
                    type: "string",
                    description: "Search term to match against version tags, names, and body text.",
                  },
                  limit: {
                    type: "number",
                    description: "Max results to return (default: 10, max: 50).",
                  },
                },
                required: ["query"],
              },
            },
          ],
        });
        return;
      }

      if (rpc.method === "tools/call") {
        const params = rpc.params ?? {};
        const name = params.name;
        const args = params.arguments as Record<string, unknown> | undefined;

        if (name === "list_releases") {
          const includePrerelease = Boolean(args?.includePrerelease ?? false);
          const releases = await getReleases();
          const filtered = releases.filter((r) => includePrerelease || !r.prerelease);
          const index = filtered.map((r) => ({
            tag_name: r.tag_name,
            name: r.name ?? r.tag_name,
            published_at: r.published_at,
            prerelease: r.prerelease,
          }));
          sendJsonRpcResult(res, rpc.id, {
            content: [{ type: "text", text: JSON.stringify(index, null, 2) }],
            structuredContent: { releases: index },
          });
          return;
        }

        if (name === "get_changelog") {
          const version = typeof args?.version === "string" ? args.version : null;
          if (!version) {
            sendJsonRpcError(res, rpc.id, -32602, "Missing required argument: version");
            return;
          }
          const releases = await getReleases();
          const release = releases.find((r) => r.tag_name === version);
          if (!release) {
            sendJsonRpcError(res, rpc.id, -32602, `Version not found: ${version}`);
            return;
          }
          sendJsonRpcResult(res, rpc.id, {
            content: [{ type: "text", text: formatReleaseAsText(release) }],
            structuredContent: release,
          });
          return;
        }

        if (name === "search_changelog") {
          const query = typeof args?.query === "string" ? args.query : null;
          if (!query) {
            sendJsonRpcError(res, rpc.id, -32602, "Missing required argument: query");
            return;
          }
          const limit = Math.min(Number(args?.limit ?? 10), 50);
          const releases = await getReleases();
          const q = query.toLowerCase();
          const matches = releases
            .filter(
              (r) =>
                r.tag_name.toLowerCase().includes(q) ||
                (r.name ?? "").toLowerCase().includes(q) ||
                (r.body ?? "").toLowerCase().includes(q),
            )
            .slice(0, limit)
            .map((r) => ({
              tag_name: r.tag_name,
              name: r.name ?? r.tag_name,
              published_at: r.published_at,
              snippet: extractSnippet(r.body, query),
            }));
          sendJsonRpcResult(res, rpc.id, {
            content: [{ type: "text", text: JSON.stringify(matches, null, 2) }],
            structuredContent: { matches },
          });
          return;
        }

        sendJsonRpcError(res, rpc.id, -32602, `Unknown tool: ${String(name)}`);
        return;
      }

      sendJsonRpcError(res, rpc.id, -32601, `Method not found: ${rpc.method}`);
      return;
    }

    // ── API routes ─────────────────────────────────────────────────

    if (url.pathname === "/api/releases") {
      const releases = await getReleases();
      const cache = await loadCache();
      const stale = cache ? isCacheStale(cache) : true;
      const cachedAt = cache ? new Date(cache.fetchedAt).toISOString() : null;
      const index = releases
        .filter((r) => !r.draft)
        .map(toIndexEntry);
      sendJson(res, 200, { releases: index, cachedAt, stale, total: index.length });
      return;
    }

    if (url.pathname === "/api/release") {
      const tag = url.searchParams.get("tag");
      if (!tag) {
        sendJson(res, 400, { error: "Missing ?tag= parameter" });
        return;
      }
      const releases = await getReleases();
      const release = releases.find((r) => r.tag_name === tag);
      if (!release) {
        sendJson(res, 404, { error: `Release not found: ${tag}` });
        return;
      }
      sendJson(res, 200, release);
      return;
    }

    if (url.pathname === "/api/cache-status") {
      const cache = await loadCache();
      if (!cache) {
        sendJson(res, 200, { exists: false, stale: true, count: 0 });
        return;
      }
      sendJson(res, 200, {
        exists: true,
        fetchedAt: new Date(cache.fetchedAt).toISOString(),
        ttlMs: cache.ttlMs,
        expiresAt: new Date(cache.fetchedAt + cache.ttlMs).toISOString(),
        stale: isCacheStale(cache),
        count: cache.releases.length,
      });
      return;
    }

    if (url.pathname === "/api/cache-bust" && req.method === "POST") {
      // Planted lab bug: throws ENOENT if .cache/ dir doesn't exist.
      // Students fix: wrap in try/catch or check existence first.
      await unlink(CACHE_FILE);
      sendJson(res, 200, { ok: true, message: "Cache cleared" });
      return;
    }

    // ── Static files ───────────────────────────────────────────────

    const route = staticRoutes[url.pathname];
    if (!route) {
      res.writeHead(404, { "content-type": "text/plain; charset=utf-8" });
      res.end("Not found");
      return;
    }

    const filePath = join(publicDir, route.file);
    const body = await readFile(filePath);
    res.writeHead(200, { "content-type": route.contentType });
    res.end(body);
  } catch (error) {
    // Keep runtime diagnostics visible in `docker logs` for workshop debugging.
    // eslint-disable-next-line no-console
    console.error("Changelog viewer server error:", error);
    res.writeHead(500, { "content-type": "text/plain; charset=utf-8" });
    res.end(`Server error: ${(error as Error).message}`);
  }
});

const port = Number(process.env.PORT ?? 4200);
server.listen(port, () => {
  // Keep startup output explicit for workshop copy/paste.
  // eslint-disable-next-line no-console
  console.log(`Changelog viewer running at http://localhost:${port}`);
  // eslint-disable-next-line no-console
  console.log(`Static files: ${publicDir}`);
});
