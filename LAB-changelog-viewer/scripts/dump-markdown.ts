/**
 * dump-markdown.ts
 *
 * Writes all Cursor releases as individual .md files to the output/ directory.
 * Respects the .cache/releases.json TTL — won't re-fetch if cache is fresh.
 *
 * Usage:
 *   npm run dump
 *   OUTPUT_DIR=/tmp/cursor-releases npm run dump
 */
import { mkdir, writeFile } from "node:fs/promises";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";
import { getReleases } from "../src/cursor.ts";
import type { CursorRelease } from "../src/types.ts";

const labRoot = dirname(dirname(fileURLToPath(import.meta.url)));
const outputDir = process.env.OUTPUT_DIR ?? join(labRoot, "output");

function sanitizeTag(tag: string): string {
  return tag.replace(/\//g, "-");
}

function buildMarkdownFile(release: CursorRelease): string {
  const body = release.body ?? "(no release notes)";
  const displayName = release.name?.trim() || release.tag_name;
  const lines = [
    "---",
    `tag: ${release.tag_name}`,
    `name: "${displayName.replace(/"/g, '\\"')}"`,
    `published_at: ${release.published_at}`,
    `prerelease: ${release.prerelease}`,
    `url: ${release.html_url}`,
    "---",
    "",
    `# ${release.tag_name} — ${displayName}`,
    "",
    body,
  ];
  return lines.join("\n");
}

async function main(): Promise<void> {
  console.log("fetching releases…");
  const releases = await getReleases();

  const publishable = releases
    .filter((r) => !r.draft)
    .sort(
      (a, b) =>
        new Date(b.published_at).getTime() - new Date(a.published_at).getTime(),
    );

  await mkdir(outputDir, { recursive: true });

  for (const release of publishable) {
    const filename = sanitizeTag(release.tag_name) + ".md";
    const filePath = join(outputDir, filename);
    const content = buildMarkdownFile(release);
    await writeFile(filePath, content, "utf8");
    console.log(`  wrote ${join("output", filename)}`);
  }

  console.log(`\ndone: wrote ${publishable.length} files to ${outputDir}`);
}

main().catch((err: unknown) => {
  console.error("dump-markdown error:", (err as Error).message);
  process.exit(1);
});
