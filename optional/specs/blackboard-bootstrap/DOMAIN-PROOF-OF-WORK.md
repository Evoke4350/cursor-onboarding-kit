# Domain 8: Proof of Work (Showboat)

**Bead**: cursor-onboarding-kit-clz
**Source**: https://github.com/simonw/showboat, https://github.com/simonw/chartroom
**Key Insight**: AI agents do work, but can't prove it. Showboat solves this.

---

## Problem

When an AI agent:
- Debugs an issue
- Runs a test
- Generates a report
- Creates a chart

You can see the output in the chat, but:
- It's not reproducible
- It's not verifiable
- It's lost when the session ends
- You can't prove to others what happened

---

## Solution: Showboat

Showboat creates **executable documents** that:
1. Mix commentary + code + captured output
2. Are readable documentation AND reproducible proof
3. Can be verified by re-running all code blocks
4. Stream remotely in real-time

### Example Showboat Document

````markdown
# Debugging the Auth Flow

*2026-02-18T01:08:00Z*

First, let's check the current auth state.

```bash
cat ~/.config/app/auth.json
```

```output
{"token": "abc123", "expires": "2026-03-01"}
```

Now let's test the token.

```bash
curl -H "Authorization: Bearer abc123" https://api.example.com/me
```

```output
{"user": "alice", "role": "admin"}
```

The token is valid and we're authenticated as alice.
````

### Verification

```bash
showboat verify debug-auth.md
# Re-runs both bash blocks, compares outputs
# Exits 0 if matches, 1 if changed
```

---

## Integration with Blackboard

### Shavings as Showboat Documents

| Before | After |
|--------|-------|
| Static markdown | Executable markdown |
| Can't verify | `showboat verify` |
| No code output | Captured output |
| Manual charts | Chartroom auto-alt |

### Command Mapping

| Blackboard Command | Showboat Equivalent |
|--------------------|---------------------|
| `/cut` (extract insight) | `showboat note` + `showboat exec` |
| `/check` (validate) | `showboat verify *.md` |
| `/carve` (find connections) | `showboat extract` + combine |
| Session capture | `SHOWBOAT_REMOTE_URL` → sawdust/ |

### Architecture Update

```
~/workshop/
├── bench/
│   └── index.md          # MOC (can be Showboat doc)
├── shavings/
│   ├── debug-auth.md     # Showboat document
│   ├── analyze-perf.md   # Showboat document
│   └── ...
├── sawdust/
│   ├── sessions/         # Remote streaming target
│   │   └── 2026-02-18T01-08-00/
│   └── queue/
└── .workshop/
    ├── tools/
    │   └── proof/        # Showboat tools
    │       ├── cut.md
    │       ├── check.md
    │       └── chart.md
    └── config.yaml       # SHOWBOAT_REMOTE_URL
```

---

## Tasks

| ID | Task | Priority |
|----|------|----------|
| W-01 | Integrate showboat CLI | P0 |
| W-02 | Store shavings as showboat docs | P0 |
| W-03 | /check runs showboat verify | P0 |
| W-04 | Remote streaming to sawdust | P1 |
| W-05 | Chartroom integration | P1 |

---

## UVX vs Rust Decision

### Recommendation: Hybrid

| Layer | Technology | Reason |
|-------|------------|--------|
| Showboat integration | Python/uvx | Native compatibility |
| Chartroom integration | Python/uvx | Native compatibility |
| Search (ripgrep) | Rust | Performance |
| SQLite reader | Rust | Performance |
| CLI framework | Rust | Performance, single binary |

### Installation

```bash
# Core workshop CLI (Rust)
cargo install workshop

# Showboat/Chartroom (Python via uvx)
uvx showboat --help
uvx chartroom --help
```

### Wrapper Pattern

```rust
// Rust CLI calls out to uvx tools
fn run_showboat_verify(shaving: &Path) -> Result<bool> {
    let output = Command::new("uvx")
        .args(["showboat", "verify", shaving.to_str().unwrap()])
        .output()?;
    Ok(output.status.success())
}
```

---

## Real-World Example

### Before (Static Shaving)

```markdown
---
title: the evidence suggests that morning exercise reduces anxiety
---

I analyzed the health data and found that participants who exercised
in the morning had 23% lower anxiety scores. The data showed...

[No code, no proof, just claims]
```

### After (Showboat Shaving)

```markdown
---
title: the evidence suggests that morning exercise reduces anxiety
created: 2026-02-18T01:08:00Z
---

*2026-02-18T01:08:00Z*

I analyzed the health data to test the morning exercise hypothesis.

```bash
cd ~/data/health && python3 analyze.py --group morning
```

```output
Group: morning_exercisers
n: 234
anxiety_mean: 12.3
anxiety_std: 3.2
```

```bash
python3 analyze.py --group evening
```

```output
Group: evening_exercisers
n: 198
anxiety_mean: 15.9
anxiety_std: 4.1
```

```bash
chartroom bar --json analysis.json -x group -y anxiety_mean -f markdown
```

![Anxiety by exercise time](chart.png)

The morning group has 23% lower anxiety (12.3 vs 15.9, p < 0.01).
```

### Verification

```bash
workshop check shavings/morning-exercise-anxiety.md
# Re-runs all 4 code blocks
# Regenerates chart
# Exits 0 if everything matches
```

---

## Remote Streaming

When `SHOWBOAT_REMOTE_URL` is set, every `note`, `exec`, `image` command POSTs to the URL.

**Blackboard as receiver:**

```bash
export SHOWBOAT_REMOTE_URL=http://localhost:8080/workshop/stream
```

Server receives:
- UUID (ties commands together)
- Command type (init, note, exec, image, pop)
- Content (markdown, code, output)

This enables:
- Real-time dashboard of agent work
- Session replay
- Collaborative viewing
- Audit trail

---

## Acceptance Criteria

| ID | Criterion | Verify |
|----|-----------|--------|
| W-01-1 | `uvx showboat` works | `uvx showboat --help` |
| W-01-2 | `uvx chartroom` works | `uvx chartroom --help` |
| W-02-1 | New shaving is Showboat format | `showboat verify shaving.md` |
| W-03-1 | `/check` runs verify on all shavings | Integration test |
| W-03-2 | Exit 1 if any output changed | Integration test |
| W-04-1 | POST to URL on each command | Mock server test |
| W-05-1 | Charts have auto-generated alt text | Visual inspection |
