// Cursor Changelog Viewer — vanilla JS, no framework, no bundler
// Mirrors the boot() pattern from LAB-scheduling-demo/ui/public/app.js

const logStack = document.getElementById("log-stack");
const versionJump = document.getElementById("version-jump");
const statusBar = document.getElementById("status-bar");
const searchInput = document.getElementById("search");
const btnTop = document.getElementById("btn-top");

// ── Markdown renderer ─────────────────────────────────────────────
// Pure regex transforms — no external lib, no innerHTML injection of raw text.
// This is also the XSS guard: we only emit a controlled subset of HTML.
// Lines starting with "+ " → green (add), "~ " → yellow (change).
// Note: Cursor's official release notes use standard "- " bullets;
// the + and ~ classifiers are a lab exercise hook — see README.
function escapeHtml(str) {
  return str
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;");
}

function renderInline(text) {
  return escapeHtml(text)
    .replace(/`([^`]+)`/g, '<code class="md-code">$1</code>')
    .replace(/\*\*([^*]+)\*\*/g, '<strong class="md-strong">$1</strong>')
    .replace(/\[([^\]]+)\]\(([^)]+)\)/g, '<a class="md-link" href="$2" target="_blank" rel="noopener">$1</a>');
}

function renderMarkdown(rawBody) {
  const lines = (rawBody ?? "").split("\n");
  const parts = [];

  for (const rawLine of lines) {
    const line = rawLine.trimEnd();

    if (/^#{1,2}\s+/.test(line)) {
      parts.push(`<span class="md-h2">${renderInline(line.replace(/^#+\s+/, ""))}</span>`);
      continue;
    }
    if (/^###\s+/.test(line)) {
      parts.push(`<span class="md-h3">${renderInline(line.replace(/^###\s+/, ""))}</span>`);
      continue;
    }
    if (/^\+\s+/.test(line)) {
      parts.push(`<span class="md-add">${renderInline(line.slice(2))}</span>`);
      continue;
    }
    if (/^~\s+/.test(line)) {
      parts.push(`<span class="md-change">${renderInline(line.slice(2))}</span>`);
      continue;
    }
    // Heuristic classifier: lines with "fix" keywords get a distinct color
    if (/^[-*]\s+/i.test(line)) {
      const content = line.replace(/^[-*]\s+/, "");
      const cls = /\bfix(es|ed)?\b|\bbug\b/i.test(content) ? "md-fix" : "md-bullet";
      parts.push(`<span class="${cls}">${renderInline(content)}</span>`);
      continue;
    }
    if (line.trim() === "") {
      parts.push("<br>");
      continue;
    }
    parts.push(`<span class="md-line">${renderInline(line)}</span>`);
  }

  return parts.join("\n");
}

// ── Entry rendering ───────────────────────────────────────────────
function formatDate(isoString) {
  return isoString.slice(0, 10);
}

function renderLogEntry(entry) {
  const el = document.createElement("div");
  el.className = "log-entry" + (entry.prerelease ? " pre" : "");
  el.id = entry.tag_name;
  el.dataset.tag = entry.tag_name;
  el.dataset.searchText = (entry.tag_name + " " + entry.name + " " + entry.bodyPreview).toLowerCase();

  const dot = document.createElement("div");
  dot.className = "dot";
  dot.textContent = "●";

  const summary = document.createElement("div");
  summary.className = "log-summary";
  summary.setAttribute("role", "button");
  summary.setAttribute("tabindex", "0");
  summary.setAttribute("aria-expanded", "false");

  const preBadge = entry.prerelease
    ? `<span class="badge-pre">pre</span>`
    : "";

  summary.innerHTML = `
    <span class="tag">${escapeHtml(entry.tag_name)}</span>
    <span class="date">${formatDate(entry.published_at)}</span>
    <span class="preview">${escapeHtml(entry.bodyPreview || "(no notes)")}</span>
    ${preBadge}
    <span class="chevron">▶</span>
  `;

  const body = document.createElement("div");
  body.className = "log-body";
  body.hidden = true;

  el.appendChild(dot);
  el.appendChild(summary);
  el.appendChild(body);

  summary.addEventListener("click", () => void expandEntry(el));
  summary.addEventListener("keydown", (e) => {
    if (e.key === "Enter" || e.key === " ") {
      e.preventDefault();
      void expandEntry(el);
    }
  });

  return el;
}

async function expandEntry(entryEl) {
  const isExpanded = entryEl.classList.contains("expanded");
  const body = entryEl.querySelector(".log-body");
  const summary = entryEl.querySelector(".log-summary");

  if (isExpanded) {
    entryEl.classList.remove("expanded");
    body.hidden = true;
    summary.setAttribute("aria-expanded", "false");
    return;
  }

  entryEl.classList.add("expanded");
  body.hidden = false;
  summary.setAttribute("aria-expanded", "true");

  if (body.dataset.loaded) return;

  body.innerHTML = '<span class="loading">loading…</span>';
  body.dataset.loaded = "pending";

  try {
    const tag = entryEl.dataset.tag;
    const res = await fetch(`/api/release?tag=${encodeURIComponent(tag)}`);
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    const release = await res.json();
    body.innerHTML = renderMarkdown(release.body);
    body.dataset.loaded = "done";
  } catch (err) {
    body.innerHTML = `<span class="md-line" style="color:var(--red)">failed to load: ${escapeHtml(err.message)}</span>`;
    body.dataset.loaded = "error";
  }
}

// ── Version jump sidebar ──────────────────────────────────────────
function renderVersionJump(releases) {
  versionJump.innerHTML = "";
  for (const entry of releases) {
    const a = document.createElement("a");
    a.href = "#" + entry.tag_name;
    a.textContent = entry.tag_name;
    if (entry.prerelease) a.className = "pre";
    versionJump.appendChild(a);
  }
}

// ── Log stack ─────────────────────────────────────────────────────
function renderLogStack(releases) {
  logStack.innerHTML = "";
  if (releases.length === 0) {
    logStack.innerHTML = '<div class="log-placeholder">no releases found.</div>';
    return;
  }
  for (const entry of releases) {
    logStack.appendChild(renderLogEntry(entry));
  }
}

// ── Search / filter ───────────────────────────────────────────────
let filterTimer = null;

function applyFilter(query) {
  const q = query.toLowerCase().trim();
  const entries = logStack.querySelectorAll(".log-entry");
  let visible = 0;
  for (const entry of entries) {
    const matches = !q || entry.dataset.searchText.includes(q);
    entry.classList.toggle("hidden", !matches);
    if (matches) visible++;
  }
  // Also filter version jump
  const jumpLinks = versionJump.querySelectorAll("a");
  for (const link of jumpLinks) {
    const tag = link.getAttribute("href").slice(1);
    const entry = logStack.querySelector(`[data-tag="${CSS.escape(tag)}"]`);
    link.style.display = entry?.classList.contains("hidden") ? "none" : "";
  }
  updateStatusCount(visible);
}

function updateStatusCount(count) {
  const totalEl = document.getElementById("status-total");
  if (totalEl) totalEl.textContent = `${count} release${count === 1 ? "" : "s"}`;
}

// ── Status bar ────────────────────────────────────────────────────
function renderStatusBar(data) {
  const staleWarning = data.stale ? " · <span style='color:var(--yellow)'>cache stale — refresh to update</span>" : "";
  const cachedStr = data.cachedAt
    ? `cached ${data.cachedAt.slice(0, 16).replace("T", " ")} UTC`
    : "live";
  statusBar.innerHTML = `<span id="status-total">${data.total} releases</span> · ${cachedStr}${staleWarning}`;
  if (data.stale) statusBar.classList.add("stale");
}

// ── Boot ──────────────────────────────────────────────────────────
async function boot() {
  logStack.innerHTML = '<div class="log-placeholder">fetching releases…</div>';

  const res = await fetch("/api/releases");
  if (!res.ok) throw new Error(`/api/releases returned HTTP ${res.status}`);
  const data = await res.json();

  renderLogStack(data.releases);
  renderVersionJump(data.releases);
  renderStatusBar(data);

  searchInput.addEventListener("input", () => {
    clearTimeout(filterTimer);
    filterTimer = setTimeout(() => applyFilter(searchInput.value), 150);
  });

  btnTop.addEventListener("click", () => {
    window.scrollTo({ top: 0, behavior: "smooth" });
  });
}

void boot().catch((err) => {
  logStack.innerHTML = `<div class="log-placeholder" style="color:var(--red)">failed to load: ${escapeHtml(err.message)}</div>`;
});
