# Compaction Protocol

**Purpose**: State preservation across context resets during autonomous Ralph loop execution.

---

## Compaction Trigger

When context approaches limit (system warning), execute this protocol BEFORE compaction occurs.

---

## State Preservation Checklist

### 1. Current Task State
```bash
# What task am I on?
bd list -t task -s in_progress

# Save to sawdust/state/current-task.md
cat > workshop/sawdust/state/current-task.md << 'EOF'
task_id: <id>
title: <title>
phase: <orient|execute|prove|persist>
started: <timestamp>
blocking: <any blockers>
EOF
```

### 2. Uncommitted Changes
```bash
# Stash anything not committed
git stash push -m "compaction checkpoint $(date +%s)"
git status > workshop/sawdust/state/git-status.txt
```

### 3. Taint State (CRITICAL)
```bash
# If operating on sensitive files, document taint state
echo "TAINT_STATE: <clean|tainted>" > workshop/sawdust/state/taint-state.txt
echo "TAINT_SOURCES: <files read>" >> workshop/sawdust/state/taint-state.txt
echo "TAINT_TIME: $(date -Iseconds)" >> workshop/sawdust/state/taint-state.txt
```

### 4. Session Summary
```bash
cat >> workshop/sawdust/sessions/compaction-$(date +%Y%m%d-%H%M%S).md << 'EOF'
# Compaction Checkpoint

## Task in Progress
<task-id>: <description>

## Work Done
- <bullet list of actual work>

## Files Modified
- <list of files>

## Tests Run
- <which tests, what results>

## Next Step
<exact next action to take>

## Taint State
Clean / Tainted (if tainted, list sources)

## Git State
<last commit hash>
<uncommitted changes summary>
EOF
```

---

## Post-Compaction Recovery

### Immediate Actions (within first 30 seconds)

```bash
# 1. Check for in-progress task
cat workshop/sawdust/state/current-task.md

# 2. Check taint state
cat workshop/sawdust/state/taint-state.txt

# 3. Check git state
git status
git stash list

# 4. Resume task
bd update <task-id> --status in_progress
```

### Resume Protocol

1. **Read recovery files** — current-task.md, taint-state.txt, latest session file
2. **Restore taint awareness** — If state shows "tainted", re-read what sources were accessed
3. **Continue from exact point** — Do not restart the task, continue where you left off
4. **No new tasks** — Do not start a new task until current one is complete

---

## Taint State Inheritance

**CRITICAL**: Taint state persists across compaction.

If `taint-state.txt` shows `TAINT_STATE: tainted`:
- Do NOT execute any sink commands (curl, wget, rsync, scp)
- The previous session read sensitive files
- This conversation is contaminated until explicitly reset

Reset taint state ONLY when:
- Starting a completely new task unrelated to previous sensitive data
- Human explicitly requests reset
- New session with no inheritance

---

## Forbidden Actions During Compaction

1. **Never commit during compaction** — Too easy to lose track of what's being committed
2. **Never start new tasks** — Finish what's in progress
3. **Never skip verification** — If Showboat verify was pending, run it
4. **Never ignore taint state** — Security invariant must be preserved

---

## Compaction Log

Each compaction appends to `workshop/sawdust/sessions/compaction-log.md`:

```
[2026-02-18T01:45:00] Compaction occurred
- Task: C-02 (in_progress)
- Taint: clean
- Git: clean
- Recovery: successful
```

---

## Verification

After recovery, verify state integrity:

```bash
# Check bead state matches file state
bd list -t task -s in_progress | wc -l  # Should be 0 or 1

# Check git is clean or properly stashed
git status --porcelain | wc -l  # Know the count

# Check taint state file exists
test -f workshop/sawdust/state/taint-state.txt && echo "OK"
```

---

**Remember: Compaction is an interruption, not a reset. Preserve state. Resume exactly.**
