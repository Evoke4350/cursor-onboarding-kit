# Commit Protocol

**Purpose**: Strict commit discipline for autonomous Ralph loop execution.

---

## The Rule

**ONE TASK = ONE COMMIT = ONE PUSH**

No exceptions. No batching. No "I'll commit later."

---

## Pre-Commit Checklist

Before ANY commit:

```bash
# 1. Verify single task
bd list -t task -s in_progress | wc -l
# MUST be exactly 1

# 2. Check what changed
git status --short
git diff --stat

# 3. Verify changes relate to task
# If files unrelated to task are modified, STOP
```

---

## Commit Format

Every commit follows this format:

```
<type>(<scope>): <description>

[body if needed]

Task: <task-id>
Source: <source-file>:<lines>
Acceptance: <criteria-id>

Co-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>
```

### Types
- `feat`: New feature (tasks implementing new functionality)
- `fix`: Bug fix
- `docs`: Documentation
- `test`: Test addition/modification
- `refactor`: Code restructuring
- `security`: Security-related change

### Examples

```
feat(security): add taint sources schema

Create .workshop/policy/sources.yaml with protected paths.
Implements taint tracking for .secrets/*, *.env, credentials.*

Task: cursor-onboarding-kit-y6k
Source: ~/agno/libs/agno/agno/guardrails/pii.py:10-95
Acceptance: AC-S-01, AC-S-02

Co-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>
```

```
feat(cursor): prototype SQLite reader for Cursor conversations

Read ~/Library/Application Support/Cursor/User/globalStorage/state.vscdb
Parse cursorDiskKV table for composerData entries.

Task: cursor-onboarding-kit-34k
Source: ~/Dicklesworthstone/coding_agent_session_search/src/connectors/cursor.rs:156-239
Acceptance: AC-C-01, AC-C-02

Co-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>
```

---

## Post-Commit Actions

After EVERY commit:

```bash
# 1. Push immediately
git push

# 2. Mark task complete
bd update <task-id> --status closed

# 3. Update progress
echo "$(date -Iseconds): Completed <task-id>" >> specs/blackboard-bootstrap/progress.txt

# 4. Print DONE
echo "DONE"
```

---

## Commit Loop Protocol

```
┌─────────────────────────────────────────────────────────────┐
│                    RALPH LOOP ITERATION                     │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  1. ORIENT                                                  │
│     ├─ Read CURSOR-BOOTSTRAP.md                             │
│     ├─ Find next P0 task: bd list -t task -s open | grep P0 │
│     └─ Check dependencies: bd dep tree <task-id>            │
│                                                             │
│  2. CLAIM                                                   │
│     └─ bd update <task-id> --status in_progress             │
│                                                             │
│  3. EXECUTE                                                 │
│     ├─ Read source from CROSS-REFERENCE-MATRIX.md           │
│     ├─ Read acceptance criteria from ACCEPTANCE-CRITERIA.md │
│     ├─ Implement task                                       │
│     └─ Verify against acceptance criteria                   │
│                                                             │
│  4. COMMIT                                                  │
│     ├─ git add <specific files>                             │
│     ├─ git commit (with format above)                       │
│     └─ git push                                             │
│                                                             │
│  5. CLOSE                                                   │
│     ├─ bd update <task-id> --status closed                  │
│     ├─ Update progress.txt                                  │
│     └─ echo "DONE"                                          │
│                                                             │
│  6. LOOP                                                    │
│     └─ Return to step 1 (fresh context)                     │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## Forbidden Commit Patterns

### ❌ Batch Commits
```
# NEVER
git commit -m "implemented tasks S-01, S-02, S-03"
```

### ❌ WIP Commits
```
# NEVER
git commit -m "WIP on S-01"
```

### ❌ Unrelated Changes
```
# NEVER commit files unrelated to the task
git add -A  # If this picks up unrelated files, STOP
```

### ❌ No-Push Commits
```
# NEVER
git commit -m "task complete"
# without immediate push
```

---

## Recovery from Bad Commits

If you make a bad commit:

```bash
# BEFORE pushing:
git reset HEAD~1

# AFTER pushing:
# Create fix commit, don't rewrite history
git revert HEAD
```

---

## Verification

After each loop iteration:

```bash
# Verify commit exists
git log -1 --oneline

# Verify push succeeded
git status  # Should show "up to date with origin"

# Verify task closed
bd show <task-id> | grep status  # Should show "closed"
```

---

## Compaction + Commit Interaction

If compaction occurs mid-task:

1. **If uncommitted changes exist**: Stash with descriptive message
2. **After recovery**: Check stash, continue task
3. **Do NOT commit during compaction recovery** — finish the task first

---

**Remember: The git history IS the audit trail. Each commit is proof of work.**
