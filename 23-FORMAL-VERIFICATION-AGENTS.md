# Formal Verification and Property-Based Testing with AI Agents

**References:**
- [AI and the Future of Formal Verification](https://martin.kleppmann.com/2025/12/08/ai-formal-verification.html) (Martin Kleppmann, December 2025)
- [Property-Based Testing](https://red.anthropic.com/2026/property-based-testing/) (Anthropic Red, 2026)
- [Property-Based Testing in Practice](https://kiro.dev/blog/property-based-testing/) (Kiro)
- [fast-check](https://github.com/dubzzz/fast-check) (Property-based testing for JavaScript/TypeScript)
- [Hypothesis](https://hypothesis.works/) (Property-based testing for Python)

---

## The Core Thesis

Martin Kleppmann's argument is simple but profound:

> **LLMs are bad at formal verification, but they're excellent at writing specifications that humans can verify.**

The economics of formal verification have always been brutal:
- seL4: ~23 lines of proof per line of code
- CompCert: ~10+ years of effort for a verified C compiler
- Most teams can't afford this

But LLMs change the equation in a different way. They can:

1. **Generate specifications** - Natural language or semi-formal descriptions of intended behavior
2. **Suggest invariants** - Properties that should always hold
3. **Write property tests** - Executable checks that verify behavior
4. **Translate between levels** - Informal → formal, code → spec, spec → test

---

## Property-Based Testing: The Practical Middle Ground

Formal verification proves correctness. Property-based testing (PBT) *finds incorrectness*.

### How PBT Works

```javascript
// Example: fast-check property test
fc.assert(
  fc.property(
    fc.string(),                    // Any string as input
    (s) => {
      const encoded = base64Encode(s);
      const decoded = base64Decode(encoded);
      return decoded === s;         // Round-trip invariant
    }
  )
)
```

The framework generates hundreds or thousands of random inputs automatically. You define *properties* (invariants), not specific test cases.

### Common Property Shapes

| Property Type | Description | Example |
|---------------|-------------|---------|
| **Round-trip** | Encode → decode returns original | JSON.parse(JSON.stringify(x)) ≡ x |
| **Idempotence** | f(f(x)) ≡ f(x) | `Math.abs(Math.abs(x))` |
| **Commutativity** | f(a, b) ≡ f(b, a) | `a + b` |
| **Associativity** | f(f(a, b), c) ≡ f(a, f(b, c)) | `(a + b) + c` |
| **Identity** | f(x, identity) ≡ x | `x + 0` |
| **Invariance** | Property P holds before and after | List length after sort equals before |
| **No exceptions** | Never crashes on valid input | Parser handles any input |

---

## Agent Patterns for Verification

### Pattern 1: Specification Generation

```
Agent Task: Given this code, generate a specification

Input: Source code
Output: Natural-language specification of behavior

Use when: Code exists but documentation is missing
```

The agent reads code and produces a human-reviewable spec. The spec becomes the contract.

### Pattern 2: Invariant Discovery

```
Agent Task: Identify invariants that should hold for this system

Input: Code + specification
Output: List of properties that should always be true

Use when: You need to understand what to test
```

### Pattern 3: Property Test Generation

```
Agent Task: Generate property-based tests from this specification

Input: Specification
Output: Executable property tests (fast-check, Hypothesis, etc.)

Use when: You have a spec but no tests
```

### Pattern 4: Specification Consistency Check

```
Agent Task: Does this code match this specification?

Input: Code + specification
Output: Analysis of discrepancies

Use when: Verifying implementation against contract
```

---

## The "Vericoding" Workflow

Traditional development: Write code → Write tests → Hope it works

**Vericoding**: Write spec → Generate properties → Generate tests → Write code → Verify

```
┌─────────────┐
│  Intention  │  (What should this do?)
└──────┬──────┘
       ↓
┌─────────────┐
│  Spec       │  (Formal or semi-formal description)
└──────┬──────┘
       ↓
┌─────────────┐
│  Properties │  (Invariants that should hold)
└──────┬──────┘
       ↓
┌─────────────┐
│  Tests      │  (PBT or example-based)
└──────┬──────┘
       ↓
┌─────────────┐
│  Code       │  (Implementation)
└──────┬──────┘
       ↓
┌─────────────┐
│  Verify     │  (Run tests, check properties)
└─────────────┘
```

Agents excel at the translation steps. They're not proving theorems—they're generating structured content from structured content.

---

## On-the-Fly Property Testing with Agents

Instead of pre-generating all test cases, agents can:

1. **Observe running behavior** - Watch the system in action
2. **Hypothesize properties** - "This function seems to always return positive values"
3. **Generate tests** - Create property tests on the fly
4. **Validate or refute** - Run the tests, see what breaks

This is similar to how Antithesis's Bombadil works: continuous exploration of state space.

---

## The Tyche Approach

The Tyche paper (referenced in Anthropic's PBT work) introduces the concept of:

> **Agents as test oracles** - An agent can judge whether output is "reasonable" without knowing the exact expected value.

```python
# Traditional test: exact match
assert compute(input) == expected_output

# Agent oracle test: semantic match
result = compute(input)
assert agent_judge(input, result) == "reasonable"
```

This is powerful when:
- Output is non-deterministic (LLM responses, search results)
- Multiple valid outputs exist
- Exact values are hard to encode

---

## Tools and Frameworks

### Property-Based Testing

| Language | Framework | Link |
|----------|-----------|------|
| JavaScript/TypeScript | fast-check | https://github.com/dubzzz/fast-check |
| Python | Hypothesis | https://hypothesis.works/ |
| Rust | proptest | https://github.com/AltSysrq/proptest |
| Go | gopter | https://github.com/leanovate/gopter |
| Java | jqwik | https://jqwik.net/ |
| Elixir | StreamData | https://github.com/whatyouhide/stream_data |

### Formal Verification

| Tool | Use Case | Learning Curve |
|------|----------|----------------|
| TLA+ | Distributed systems | High |
| Dafny | General verification | Medium-High |
| Coq | Proof assistant | Very High |
| Prusti | Rust verification | Medium |
| SAW | Cryptographic code | High |

---

## Mapping to Cursor Onboarding Kit

| Concept | Kit Equivalent |
|---------|----------------|
| Specification generation | Issue templates in `40-TEMPLATES/` |
| Invariant discovery | Guardrails in `AGENTS.md` |
| Property testing | Test patterns in labs |
| Agent as oracle | Review workflows with Codex |

---

## Practical Recommendations

1. **Start with PBT, not formal verification** - The ROI is much higher
2. **Use agents for spec generation, not proof** - They're better at writing than proving
3. **Human review of specs is mandatory** - Agents can produce plausible-but-wrong specifications
4. **Properties > Examples** - One well-chosen property is worth 100 example tests
5. **Round-trip tests are your friend** - They catch an enormous class of bugs

---

## The Ceiling

Current AI agents cannot:
- Reliably prove mathematical theorems
- Verify concurrent systems without human guidance
- Replace formal methods for safety-critical systems

But they *can*:
- Generate specifications that humans can verify
- Write property tests that catch real bugs
- Explore state spaces faster than manual testing
- Act as semantic oracles for non-deterministic output

The sweet spot is **human + agent + property testing**. Not agent alone. Not formal verification alone. The combination.
