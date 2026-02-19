// A single Cursor changelog entry parsed from cursor.com/changelog
export type CursorRelease = {
  id: string;           // slug or version string, used as unique key
  tag_name: string;     // version label, e.g. "2.5"
  name: string;         // release title
  published_at: string; // ISO 8601 string
  body: string | null;  // full body text in markdown-ish format
  html_url: string;     // https://www.cursor.com/changelog/<slug>
  prerelease: boolean;
  draft: boolean;
};

export type CachedReleasesFile = {
  fetchedAt: number;
  ttlMs: number;
  releases: CursorRelease[];
};

// Thin index entry for list endpoint — avoids sending full body payloads
export type ReleaseIndexEntry = {
  tag_name: string;
  name: string;
  published_at: string;
  prerelease: boolean;
  bodyPreview: string;
};
