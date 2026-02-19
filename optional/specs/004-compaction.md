# Spec 004: Compaction Protocol
**Parent PRD:** specs/000-PRD.md
**Date:** 2026-02-17

This file tells any agent (or session restart) exactly how to resume work on this sprint.

---

## On Every Session Start

```bash
cd /Users/nateb/cursor-onboarding-kit

# 1. Check epic status
bd show cursor-onboarding-kit-71q

# 2. See what's ready to work on (unblocked, not in_progress)
bd ready --json

# 3. See if any work was claimed but not finished
bd list --status in_progress --json

# 4. If in_progress work exists, continue it
#    If nothing in_progress, claim the next ready task:
bd update {next-task-id} --status in_progress --json
```

---

## Ralph Loop (per task)

```
1. CLAIM     bd update {id} --status in_progress
2. FETCH     curl -s "https://docs.cursor.com/en/{path}.md"
3. READ      Parse content, find key learnable concepts
4. EXTRACT   Pull verbatim quotes (≤3 sentences per block)
5. VERIFY    Confirm quote exists verbatim in fetched content
6. MKDIR     mkdir -p LAB-changelog-viewer/output/docs/{section}
7. WRITE     Write tip file per specs/003-protocol.md template
8. CLOSE     bd close {id} --reason "written: output/docs/{path}.md"
9. NEXT      bd ready --json → claim next task
```

---

## Bead IDs Quick Reference

| Section | Bead ID |
|---|---|
| ROOT EPIC | cursor-onboarding-kit-71q |
| get-started | cursor-onboarding-kit-1lm |
| agent core | cursor-onboarding-kit-26z |
| agent/chat | cursor-onboarding-kit-bs0 |
| background-agent | cursor-onboarding-kit-c1z |
| background-agent/api | cursor-onboarding-kit-cvo |
| cli core | cursor-onboarding-kit-ioe |
| cli/reference | cursor-onboarding-kit-ytk |
| cli/cookbook | cursor-onboarding-kit-3x9 |
| cli integrations | cursor-onboarding-kit-cqt |
| context/@-symbols | cursor-onboarding-kit-c6d |
| context system | cursor-onboarding-kit-wru |
| inline-edit | cursor-onboarding-kit-mir |
| tab | cursor-onboarding-kit-lvl |
| models | cursor-onboarding-kit-b60 |
| settings/api-keys | cursor-onboarding-kit-yvl |
| configuration | cursor-onboarding-kit-1rc |
| integrations | cursor-onboarding-kit-zqo |
| bugbot | cursor-onboarding-kit-d7v |
| account | cursor-onboarding-kit-1hz |
| account/teams | cursor-onboarding-kit-zap |
| guides/migration | cursor-onboarding-kit-2m9 |
| guides/languages | cursor-onboarding-kit-r25 |
| guides/advanced | cursor-onboarding-kit-iui |
| guides/tutorials | cursor-onboarding-kit-zuo |
| guides/working-with-context | cursor-onboarding-kit-2yw |
| tools | cursor-onboarding-kit-z6m |
| troubleshooting | cursor-onboarding-kit-vil |
| welcome | cursor-onboarding-kit-j70 |

---

## Files to Check

| File | Purpose |
|---|---|
| specs/000-PRD.md | Full problem statement and definition of done |
| specs/001-sections.md | Section breakdown with priorities |
| specs/002-tasks.md | Every doc page listed with its section bead |
| specs/003-protocol.md | Exact file format and quote rules |
| specs/004-compaction.md | This file — session restart checklist |

---

## Output Location

```
/Users/nateb/cursor-onboarding-kit/LAB-changelog-viewer/output/docs/
```

Directory structure mirrors the URL path after `/en/`.
