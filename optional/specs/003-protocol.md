# Spec 003: Strict Editorial Protocol
**Parent PRD:** specs/000-PRD.md
**Date:** 2026-02-17

This is the canonical protocol for writing each doc tip file. Same protocol as the changelog tips. No shortcuts.

---

## Fetch

```bash
curl -s "https://docs.cursor.com/en/{path}.md"
```

The `.md` suffix on docs.cursor.com returns raw markdown. No HTML parsing needed. If a page returns 404, note it in the tip file as NOT-FOUND.

---

## File Header (mandatory fields)

```markdown
# Doc Tip: {Page Title}

**Source:** {Page title} — docs.cursor.com
**Date scraped:** 2026-02-17
**URL:** https://docs.cursor.com/en/{path}
**Verification status:** VERBATIM | PARAPHRASE | NOT-FOUND
```

---

## Verification Status Rules

- **VERBATIM** — every quote in the file exists word-for-word in the fetched page content
- **PARAPHRASE** — at least one statement is paraphrased because the source language is too dense to quote directly (flag which statement)
- **NOT-FOUND** — the concept was thought of during analysis but does not appear in the page (flag clearly as "not found in source, thought of it")

---

## Quote Rules (same as changelog tips)

1. Start where the thought begins, continue until fully expressed — do not truncate mid-thought
2. Include all reasoning, not just conclusions
3. Keep hedges and qualifiers ("may", "can", "typically") — they signal uncertainty
4. Include emotional or emphasis language when present
5. Do not combine statements from different sections of the page into one quote block
6. If a quote would exceed 3 sentences, break into separate quote blocks with bridging narrative
7. Confirm each quote exists verbatim before marking VERBATIM

---

## Output File Template

```markdown
# Doc Tip: {Page Title}

**Source:** {Page title} — docs.cursor.com
**Date scraped:** 2026-02-17
**URL:** https://docs.cursor.com/en/{path}
**Verification status:** VERBATIM

---

## What this teaches

{Reasoning about why this is useful or interesting. Include your uncertainty. Do not strip hedges.
Write as if explaining to a smart engineer who hasn't used Cursor. If something surprised you, say so.}

---

## Quote from docs (verbatim)

> "{Exact text}"

[If needed, second quote block:]

> "{Exact text from same page, different section}"

---

## Demo talking point

{Specific, actionable. What to show, in what order, what moment lands.}

---

## Audience note

{Who this hits hardest and how to frame it for them.}
```

---

## Multi-concept pages

Some pages contain multiple distinct, learnable things. In that case, write multiple sections in the same file, each with its own quote block, "what this teaches", and demo talking point. Do NOT create multiple files per page — one file per page, multiple concept sections within it.

Example page: `agent/modes` — covers Agent mode, Ask mode, Manual mode separately. Three sections in one file.

---

## Pure reference pages

Some pages (e.g. `cli/reference/parameters`, `background-agent/api/list-agents`) are pure reference — they're tables and parameter lists with no prose to quote. For these:

- Write a single "what this teaches" section explaining the shape of the reference
- Quote the most important 1-2 lines (e.g. a key parameter or default value)
- Mark as PARAPHRASE if the page is primarily tabular
- Demo talking point: "Point to this page during setup — don't try to memorize it"

---

## Compaction / Session Restart Protocol

```bash
cd /Users/nateb/cursor-onboarding-kit
bd ready --json                          # See unblocked tasks
bd list --status in_progress --json      # Find any claimed but unfinished work
bd show {bead-id}                        # Check specific bead details
```

Pick up the next ready task. Check the spec files here if the protocol is unclear.
