/**
 * cursor.ts — fetch and parse the Cursor changelog from cursor.com/changelog
 *
 * The page is a Next.js app that embeds all article content in the initial HTML.
 * Each <article> element contains one changelog entry with version label, date,
 * title, and full body text.
 *
 * Cache: .cache/releases.json — same shape as before, TTL-based.
 */
import { readFile, writeFile, mkdir } from "node:fs/promises";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";
import type { CursorRelease, CachedReleasesFile } from "./types.ts";

const CURSOR_CHANGELOG_URL = "https://www.cursor.com/changelog";
const DEFAULT_TTL_MS = 60 * 60 * 1000; // 1 hour

const labRoot = dirname(dirname(fileURLToPath(import.meta.url)));
const CACHE_DIR = join(labRoot, ".cache");
const CACHE_FILE = join(CACHE_DIR, "releases.json");

// ── HTML parsing helpers ───────────────────────────────────────────

function stripTags(html: string): string {
  return html.replace(/<[^>]+>/g, " ").replace(/\s+/g, " ").trim();
}

function decodeEntities(text: string): string {
  return text
    .replace(/&#x27;/g, "'")
    .replace(/&amp;/g, "&")
    .replace(/&lt;/g, "<")
    .replace(/&gt;/g, ">")
    .replace(/&quot;/g, '"')
    .replace(/&#39;/g, "'")
    .replace(/&nbsp;/g, " ");
}

// Convert article HTML body into a markdown-ish representation
function extractMarkdownBody(articleHtml: string): string {
  let html = articleHtml;

  // Section headings (marked with # in the source text via .label class or anchors)
  html = html.replace(/<h1[^>]*>(.*?)<\/h1>/gis, (_m, c) => `\n# ${stripTags(c).trim()}\n`);
  html = html.replace(/<h2[^>]*>(.*?)<\/h2>/gis, (_m, c) => `\n## ${stripTags(c).trim()}\n`);
  html = html.replace(/<h3[^>]*>(.*?)<\/h3>/gis, (_m, c) => `\n### ${stripTags(c).trim()}\n`);

  // Inline anchors (section headings with # prefix from the source)
  // These appear as plain text starting with # already via h1/h2 above

  // List items
  html = html.replace(/<li[^>]*>(.*?)<\/li>/gis, (_m, c) => `- ${stripTags(c).trim()}\n`);

  // Paragraphs
  html = html.replace(/<p[^>]*>(.*?)<\/p>/gis, (_m, c) => `${stripTags(c).trim()}\n\n`);

  // Remaining tags
  const text = decodeEntities(stripTags(html));

  // Collapse excessive newlines
  return text
    .replace(/\n{3,}/g, "\n\n")
    .trim();
}

// ── Parse the changelog page ───────────────────────────────────────

export function parseChangelogPage(html: string): CursorRelease[] {
  const releases: CursorRelease[] = [];

  // Extract all <article> elements
  const articlePattern = /<article[^>]*>([\s\S]*?)<\/article>/gi;
  let match: RegExpExecArray | null;

  while ((match = articlePattern.exec(html)) !== null) {
    const art = match[1];

    // Slug from first /changelog/ href in the article
    const slugMatch = art.match(/href="\/changelog\/([^"#]+)"/);
    const slug = slugMatch ? slugMatch[1] : "";

    // Version label (the span.label inside the article header)
    const labelMatch = art.match(/<span class="label">([^<]+)<\/span>/);
    const version = labelMatch ? labelMatch[1].trim() : slug || "unknown";

    // Date from <time> element text content
    const timeMatch = art.match(/<time[^>]*>([^<]+)<\/time>/);
    const dateStr = timeMatch ? timeMatch[1].trim() : "";

    // Convert "Feb 17, 2026" → ISO date string
    const published_at = parseDateString(dateStr);

    // Title from first <h1> in the article
    const h1Match = art.match(/<h1[^>]*>([\s\S]*?)<\/h1>/i);
    const name = h1Match ? stripTags(h1Match[1]).trim() : version;

    // Body text (everything after the header section)
    // The article is structured as: [header div] [body content]
    // We take the full article text minus the version/date/title header noise
    const body = extractMarkdownBody(art);

    if (!slug && !version) continue;

    releases.push({
      id: slug || version,
      tag_name: version,
      name,
      published_at,
      body,
      html_url: `https://www.cursor.com/changelog/${slug}`,
      prerelease: false,
      draft: false,
    });
  }

  return releases;
}

function parseDateString(dateStr: string): string {
  // Input: "Feb 17, 2026" or "Jan 8, 2026"
  // Output: ISO 8601 "2026-02-17T00:00:00.000Z"
  try {
    const d = new Date(dateStr);
    if (!isNaN(d.getTime())) return d.toISOString();
  } catch {
    // fall through
  }
  return new Date().toISOString();
}

// ── Cache ──────────────────────────────────────────────────────────

export async function loadCache(): Promise<CachedReleasesFile | null> {
  try {
    const raw = await readFile(CACHE_FILE, "utf8");
    return JSON.parse(raw) as CachedReleasesFile;
  } catch {
    return null;
  }
}

export function isCacheStale(cache: CachedReleasesFile): boolean {
  return Date.now() - cache.fetchedAt >= cache.ttlMs;
}

export async function saveCache(releases: CursorRelease[]): Promise<void> {
  await mkdir(CACHE_DIR, { recursive: true });
  const file: CachedReleasesFile = {
    fetchedAt: Date.now(),
    ttlMs: Number(process.env.CACHE_TTL_MS ?? DEFAULT_TTL_MS),
    releases,
  };
  await writeFile(CACHE_FILE, JSON.stringify(file, null, 2), "utf8");
}

// Max pages to fetch. Each page has ~10 entries. 40 pages = full history back to ~2023.
// Override with FETCH_PAGE_LIMIT env var. Default: 40 (full history).
const DEFAULT_PAGE_LIMIT = 40;

export async function fetchReleasesFromSite(
  pageLimit = Number(process.env.FETCH_PAGE_LIMIT ?? DEFAULT_PAGE_LIMIT),
): Promise<CursorRelease[]> {
  const headers: Record<string, string> = {
    "User-Agent":
      "Mozilla/5.0 (compatible; cursor-onboarding-kit/lab-changelog-viewer)",
    Accept: "text/html,application/xhtml+xml",
  };

  const allReleases: CursorRelease[] = [];

  for (let page = 1; page <= pageLimit; page++) {
    // Page 1 is /changelog, subsequent pages are /changelog/page/N
    const url =
      page === 1
        ? CURSOR_CHANGELOG_URL
        : `${CURSOR_CHANGELOG_URL}/page/${page}`;

    const response = await fetch(url, { headers });
    if (!response.ok) {
      throw new Error(
        `cursor.com/changelog page ${page} returned ${response.status} ${response.statusText}`,
      );
    }

    const html = await response.text();
    const releases = parseChangelogPage(html);

    if (releases.length === 0) {
      // Empty page means we've gone past the last page
      break;
    }

    allReleases.push(...releases);

    // Progress logging so long fetches aren't silent
    // eslint-disable-next-line no-console
    console.log(
      `  fetched page ${page}: ${releases.length} entries (oldest: ${releases.at(-1)?.published_at.slice(0, 10)})`,
    );
  }

  if (allReleases.length === 0) {
    throw new Error(
      "Parsed 0 releases from cursor.com/changelog — the page structure may have changed.",
    );
  }

  return allReleases;
}

export async function getReleases(): Promise<CursorRelease[]> {
  const cache = await loadCache();
  if (cache && !isCacheStale(cache)) {
    return cache.releases as CursorRelease[];
  }
  const releases = await fetchReleasesFromSite();
  await saveCache(releases);
  return releases;
}

export function buildBodyPreview(body: string | null): string {
  const text = (body ?? "")
    .replace(/^[#*\->\s]+/gm, " ")
    .replace(/\s+/g, " ")
    .trim();
  if (text.length <= 80) return text;
  return text.slice(0, 80) + "…";
}

// ── Re-export cache file path for server.ts ─────────────────────────
export { CACHE_FILE };
