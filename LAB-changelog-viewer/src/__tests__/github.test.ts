import { strict as assert } from "node:assert";
import { test, describe } from "node:test";
import { isCacheStale, buildBodyPreview, parseChangelogPage } from "../cursor.ts";
import type { CachedReleasesFile } from "../types.ts";

// Note: fetchReleasesFromSite and getReleases require network and are not unit tested here.

function mkCache(overrides: Partial<CachedReleasesFile> = {}): CachedReleasesFile {
  return {
    fetchedAt: Date.now(),
    ttlMs: 3_600_000,
    releases: [],
    ...overrides,
  };
}

// ── isCacheStale ───────────────────────────────────────────────────

describe("isCacheStale", () => {
  test("returns false when fetchedAt is recent", () => {
    const cache = mkCache({ fetchedAt: Date.now() - 100 });
    assert.equal(isCacheStale(cache), false);
  });

  test("returns true when fetchedAt is older than ttlMs", () => {
    const cache = mkCache({
      fetchedAt: Date.now() - 3_700_000,
      ttlMs: 3_600_000,
    });
    assert.equal(isCacheStale(cache), true);
  });

  test("returns true when ttlMs is 0", () => {
    const cache = mkCache({ fetchedAt: Date.now(), ttlMs: 0 });
    assert.equal(isCacheStale(cache), true);
  });

  test("returns false when cache was just written", () => {
    const cache = mkCache({ fetchedAt: Date.now() });
    assert.equal(isCacheStale(cache), false);
  });
});

// ── buildBodyPreview ───────────────────────────────────────────────

describe("buildBodyPreview", () => {
  test("handles null body without throwing", () => {
    assert.doesNotThrow(() => buildBodyPreview(null));
    assert.equal(buildBodyPreview(null), "");
  });

  test("handles empty string without throwing", () => {
    assert.equal(buildBodyPreview(""), "");
  });

  test("strips leading ## from heading lines", () => {
    const result = buildBodyPreview("## What's New\nSome text here");
    assert.ok(!result.includes("##"), `Expected ## stripped, got: ${result}`);
  });

  test("strips leading - from bullet lines", () => {
    const result = buildBodyPreview("- Fixed a bug\n- Another fix");
    assert.ok(!result.startsWith("-"), `Expected - stripped, got: ${result}`);
  });

  test("truncates at 80 chars with ellipsis", () => {
    const long = "a".repeat(100);
    const result = buildBodyPreview(long);
    assert.ok(result.endsWith("…"), `Expected ellipsis, got: ${result}`);
    assert.ok(result.length <= 82, `Expected max ~81 chars, got: ${result.length}`);
  });

  test("returns short text unchanged (no truncation)", () => {
    const short = "Bug fixes and stability improvements";
    const result = buildBodyPreview(short);
    assert.equal(result, short);
  });

  test("collapses multiple whitespace characters", () => {
    const result = buildBodyPreview("line one\n\nline two\n\nline three");
    assert.ok(!result.includes("\n"), `Expected no newlines, got: ${result}`);
  });
});

// ── parseChangelogPage ─────────────────────────────────────────────

describe("parseChangelogPage", () => {
  const MINIMAL_HTML = `
    <article>
      <a href="/changelog/2-5"><span class="label">2.5</span></a>
      <time dateTime="2026-02-17T00:00:00.000Z">Feb 17, 2026</time>
      <h1>Plugins and Async Subagents</h1>
      <p>This release introduces plugins.</p>
      <ul><li>New plugin marketplace</li><li>Async subagents</li></ul>
    </article>
    <article>
      <a href="/changelog/2-4"><span class="label">2.4</span></a>
      <time dateTime="2026-01-22T00:00:00.000Z">Jan 22, 2026</time>
      <h1>Subagents and Skills</h1>
      <p>Subagent improvements.</p>
    </article>
  `;

  test("parses correct number of releases", () => {
    const releases = parseChangelogPage(MINIMAL_HTML);
    assert.equal(releases.length, 2);
  });

  test("extracts version label as tag_name", () => {
    const releases = parseChangelogPage(MINIMAL_HTML);
    assert.equal(releases[0].tag_name, "2.5");
  });

  test("extracts title as name", () => {
    const releases = parseChangelogPage(MINIMAL_HTML);
    assert.equal(releases[0].name, "Plugins and Async Subagents");
  });

  test("extracts date as ISO string", () => {
    const releases = parseChangelogPage(MINIMAL_HTML);
    assert.ok(
      releases[0].published_at.startsWith("2026-02-17"),
      `Expected 2026-02-17, got: ${releases[0].published_at}`,
    );
  });

  test("builds correct html_url from slug", () => {
    const releases = parseChangelogPage(MINIMAL_HTML);
    assert.equal(releases[0].html_url, "https://www.cursor.com/changelog/2-5");
  });

  test("includes body text", () => {
    const releases = parseChangelogPage(MINIMAL_HTML);
    assert.ok(
      (releases[0].body ?? "").includes("plugin"),
      `Expected 'plugin' in body, got: ${releases[0].body?.slice(0, 100)}`,
    );
  });

  test("returns empty array for empty HTML", () => {
    const releases = parseChangelogPage("<html><body></body></html>");
    assert.equal(releases.length, 0);
  });
});
