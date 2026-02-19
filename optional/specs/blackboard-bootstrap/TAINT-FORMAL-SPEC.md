# Formal Specification: Taint Analysis Engine

**Version**: 1.0
**Date**: 2026-02-18
**Status**: Draft for Review

---

## 1. Domain of Discourse

### 1.1 Basic Sets

```
Path      = { valid filesystem paths }
Content   = { sequences of bytes }
Operation = { Read | Write | Transform | Exec | Sink }
TaintLevel = { Clean | Low | Medium | High | Critical }
Command   = { curl | wget | rsync | scp | nc | ... }
```

### 1.2 State Variables

```
σ : State
σ.tainted : Set(Path × TaintLevel)    -- Currently tainted paths
σ.history : List(Operation)            -- Operation history for audit
σ.blocked : List(Command)              -- Blocked commands for logging
```

---

## 2. Core Definitions

### 2.1 Protected Source

A path `p` is a **protected source** if it matches any pattern in `policy/sources.yaml`:

```
is_protected(p: Path) : Boolean
  ∃ pattern ∈ load("policy/sources.yaml").sources:
    match(pattern.glob, p) = true
```

### 2.2 Exfiltration Sink

A command `c` is an **exfiltration sink** if it matches any entry in `policy/sinks.yaml`:

```
is_sink(c: Command) : Boolean
  ∃ entry ∈ load("policy/sinks.yaml").sinks:
    c.name = entry.command ∧ entry.block_if_tainted = true
```

### 2.3 Taint Propagation

Taint propagates through operations:

```
propagates_taint(op: Operation, σ: State) : Set(Path × TaintLevel)

  case op of
    Read(p):
      if is_protected(p) then
        {(p, level_of(p))}
      else
        ∅

    Write(p, source):
      if source ∈ σ.tainted then
        {(p, σ.taint_level(source))}
      else
        ∅

    Transform(input, output):
      if input ∈ σ.tainted then
        {(output, σ.taint_level(input))}
      else
        ∅

    Exec(cmd):
      ∅  -- Execution doesn't create new taint, but may be blocked

    Sink(cmd, data_source):
      ∅  -- Sinks consume taint, don't create it
```

### 2.4 Taint State Transition

```
transition(σ: State, op: Operation) : State × Result

  let new_taints = propagates_taint(op, σ)
  let σ' = σ with tainted = σ.tainted ∪ new_taints
  let σ' = σ' with history = σ.history ++ [op]

  case op of
    Sink(cmd, data_source):
      if is_sink(cmd) ∧ (data_source ∈ σ'.tainted ∨ conversation_tainted(σ')):
        (σ' with blocked = σ'.blocked ++ [cmd], Blocked(cmd))
      else:
        (σ', Allowed)

    otherwise:
      (σ', Allowed)
```

---

## 3. Security Invariants

### 3.1 Non-Exfiltration Theorem

**Theorem**: If the taint engine correctly implements this specification, then:

```
∀ σ, op:
  let (σ', result) = transition(σ, op)
  ∀ p ∈ σ'.tainted:
    ∀ sink_op ∈ σ'.history where is_sink(sink_op.cmd):
      sink_op.data_source ≠ p ∨ result = Blocked
```

**In English**: No tainted data can flow to an exfiltration sink without being blocked.

### 3.2 Taint Monotonicity

**Invariant**: Taint only increases, never decreases, during a conversation.

```
∀ σ, op:
  let (σ', _) = transition(σ, op)
  σ.tainted ⊆ σ'.tainted
```

**Implication**: Once a conversation is tainted, it stays tainted until explicit reset.

### 3.3 Complete Audit Trail

**Invariant**: Every operation is logged.

```
∀ σ, op:
  let (σ', _) = transition(σ, op)
  σ'.history = σ.history ++ [op]
```

---

## 4. Operations Specification

### 4.1 File Read Operation

```
read_file(σ: State, path: Path) : (State, Content)

  Precondition: path exists ∧ readable
  Postcondition:
    σ'.tainted = σ.tainted ∪ (if is_protected(path) then {path} else ∅)
    σ'.history = σ.history ++ [Read(path)]

  Implementation:
    1. Check if path matches any protected pattern
    2. If yes, add to σ.tainted with appropriate level
    3. Log operation to σ.history
    4. Return file contents
```

### 4.2 Shell Execution Operation

```
exec_shell(σ: State, cmd: Command, args: List(String)) : (State, Output)

  Precondition: cmd is allowed (not in immediate-deny list)
  Postcondition:
    If is_sink(cmd) ∧ σ.tainted ≠ ∅:
      σ'.blocked = σ.blocked ++ [cmd]
      Result = Blocked("Exfiltration blocked: conversation tainted")
    Else:
      σ'.history = σ.history ++ [Exec(cmd, args)]
      Result = Allowed(output)

  Implementation:
    1. Parse command and arguments
    2. Check if command is an exfiltration sink
    3. If sink and tainted, BLOCK and LOG
    4. If allowed, execute and capture output
    5. Log to history
```

### 4.3 Symlink Resolution

```
resolve_symlink(σ: State, path: Path) : (State, ResolvedPath)

  Precondition: path exists (possibly as symlink)
  Postcondition:
    resolved = follow_symlinks(path)  -- Recursive resolution
    If any component of resolved matches protected pattern:
      σ'.tainted = σ.tainted ∪ {(resolved, level)}

  Implementation:
    1. Follow symlink chain to final target
    2. Check if target is protected
    3. Propagate taint through symlink
    4. Return resolved path
```

### 4.4 Environment Variable Access

```
access_env(σ: State, var: String) : (State, Value)

  Precondition: var exists in environment
  Postcondition:
    If var originated from protected file (tracked separately):
      σ'.tainted = σ.tainted ∪ {(env:var, level)}

  Implementation:
    1. Check if environment variable was set from protected source
    2. If yes, propagate taint
    3. Return value
```

---

## 5. Policy Schema

### 5.1 Sources Policy

```yaml
# .workshop/policy/sources.yaml
sources:
  - pattern: ".secrets/*"
    taint: critical
    description: "Secrets directory"

  - pattern: "*.env"
    taint: high
    description: "Environment files"

  - pattern: "*.pem"
    taint: critical
    description: "Certificate files"

  - pattern: "*.key"
    taint: critical
    description: "Private key files"

  - pattern: "credentials.*"
    taint: critical
    description: "Credential files"
```

### 5.2 Sinks Policy

```yaml
# .workshop/policy/sinks.yaml
sinks:
  - command: "curl"
    block_if_tainted: true
    reason: "HTTP client can exfiltrate data"

  - command: "wget"
    block_if_tainted: true
    reason: "HTTP client can exfiltrate data"

  - command: "rsync"
    block_if_tainted: true
    reason: "File sync can exfiltrate data"

  - command: "scp"
    block_if_tainted: true
    reason: "Secure copy can exfiltrate data"

  - command: "nc"
    block_if_tainted: true
    reason: "Netcat can exfiltrate data"

  - command: "nslookup"
    block_if_tainted: true
    reason: "DNS can be used for data exfiltration"
```

---

## 6. Conformance Tests

### 6.1 Must Pass

For 95% confidence, implementation MUST pass:

1. **Basic taint propagation**: Read protected file → mark tainted
2. **Basic sink blocking**: Tainted → sink command → BLOCKED
3. **Symlink tracking**: Read via symlink → mark tainted
4. **Encoding passthrough**: Base64/gzip/hex → still tainted
5. **Unicode normalization**: NFC/NFD → still tainted
6. **Partial read**: Read subset of protected file → still tainted
7. **Environment tracking**: Export from protected → env var tainted
8. **No false positives**: Read public file → NOT tainted
9. **Audit completeness**: Every operation logged
10. **Taint monotonicity**: Taint never decreases

### 6.2 Must Not Pass

For 95% confidence, implementation MUST FAIL these (indicating correct blocking):

1. **Direct exfil**: Read secret → curl secret → BLOCKED
2. **Encoded exfil**: Read secret → base64 → curl → BLOCKED
3. **Symlink exfil**: Create symlink to secret → read via symlink → curl → BLOCKED
4. **Env exfil**: Read secret → export → curl with env var → BLOCKED
5. **DNS exfil**: Read secret → encode in DNS query → BLOCKED

---

## 7. Failure Modes

### 7.1 False Negative (CATASTROPHIC)

**Definition**: Tainted data reaches sink without being blocked.

**Severity**: Critical — Secrets may leak.

**Detection**: Adversarial test suite must catch all false negatives.

**Response**: Immediate halt, audit log review, fix required before further operation.

### 7.2 False Positive (ANNOYING)

**Definition**: Clean data is incorrectly marked tainted, blocking legitimate work.

**Severity**: Medium — Reduces productivity but doesn't leak data.

**Detection**: User reports, test suite false-positive tests.

**Response**: Log, allow user override with explicit acknowledgment, fix in next version.

---

## 8. Implementation Requirements

### 8.1 Rust Type Safety

```rust
/// Taint level with explicit ordering
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaintLevel {
    Clean = 0,
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4,
}

/// Taint state is immutable for safety
pub struct TaintState {
    tainted: BTreeMap<PathBuf, TaintLevel>,
    history: Vec<Operation>,
    blocked: Vec<BlockedCommand>,
}

impl TaintState {
    /// Taint can only be added, never removed
    pub fn add_taint(&mut self, path: PathBuf, level: TaintLevel) {
        self.tainted
            .entry(path)
            .and_modify(|existing| *existing = (*existing).max(level))
            .or_insert(level);
    }

    /// Check if any tainted data exists
    pub fn is_tainted(&self) -> bool {
        !self.tainted.is_empty()
    }

    /// Check if command should be blocked
    pub fn should_block(&self, cmd: &str, policy: &SinkPolicy) -> bool {
        self.is_tainted() && policy.is_sink(cmd)
    }
}
```

### 8.2 Audit Logging

```rust
/// Every operation must be logged
pub struct AuditLog {
    entries: Vec<AuditEntry>,
}

pub struct AuditEntry {
    timestamp: DateTime<Utc>,
    operation: Operation,
    taint_before: TaintLevel,
    taint_after: TaintLevel,
    result: OperationResult,
}

impl AuditLog {
    pub fn log(&mut self, op: Operation, before: TaintLevel, after: TaintLevel, result: OperationResult) {
        self.entries.push(AuditEntry {
            timestamp: Utc::now(),
            operation: op,
            taint_before: before,
            taint_after: after,
            result,
        });
    }
}
```

---

## 9. Verification Checklist

Before claiming 95% confidence:

- [ ] All 10 "Must Pass" tests pass
- [ ] All 5 "Must Not Pass" tests correctly block
- [ ] All 15 adversarial tests in tests/adversarial/ handled correctly
- [ ] Audit log contains every operation
- [ ] Taint monotonicity verified with property tests
- [ ] Symlink resolution tested on macOS and Linux
- [ ] Unicode normalization tested (NFC, NFD, NFKC, NFKD)
- [ ] Race conditions tested with concurrent access
- [ ] Memory safety verified (no panics in production code)

---

**This specification is the contract. Implementation must conform to be trusted.**
