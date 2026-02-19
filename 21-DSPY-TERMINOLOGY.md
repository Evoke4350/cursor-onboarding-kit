# DSPy Terminology for Prompt Engineering

**Reference:** [DSPy Documentation](https://dspy.ai/learn/programming/overview/)

DSPy provides formal terminology for what we're already doing with prompts. Understanding these concepts helps you design more portable, maintainable prompt systems.

---

## The Four Components of Any Prompt

DSPy decomposes prompts into four distinct concerns. Conventional prompts couple these together — DSPy separates them.

### 1. Signature (Input/Output Types)

**What it is:** The declarative specification of what goes in and what comes out.

```
Input: code_diff, review_criteria
Output: findings, severity_scores, suggested_fixes
```

**In our docs:** See the "Semantic Prompt Address" in `optional/specs/semantic/PROMPT-SEMANTIC-MAP.md`. Each semantic ID like `UR-L2-technical-active-direct` implicitly defines a signature.

**Why it matters:** When you change LLMs, signatures stay the same. Only the adapter changes.

### 2. Adapter (Formatting and Parsing)

**What it is:** How inputs are formatted for the model and how outputs are parsed back.

```python
# Adapter logic (conceptual)
def format_input(signature, inputs):
    return f"Review this code for {inputs['review_criteria']}:\n{inputs['code_diff']}"

def parse_output(signature, raw_output):
    # Extract structured findings from model response
    return {"findings": parse_findings(raw_output)}
```

**In our docs:** See `20-PROMPT-PATTERNS.md` for formatting patterns. The "Prompt Casting" section shows adapter thinking.

**Why it matters:** Different models respond to different formatting. Adapters make prompts portable.

### 3. Module Logic (Strategies)

**What it is:** The reasoning strategies embedded in the prompt — "think step by step", "use these tools", "follow this workflow".

```
# Module logic examples
- Chain of Thought: "Think through this step by step..."
- Tool Use: "Use the Bash tool to verify..."
- Workflow: "First reproduce, then identify root cause, then fix..."
```

**In our docs:** The 5 Cs (cut, carve, chamfer, check, init) in `AGENTS.md` and the workflow patterns throughout.

**Why it matters:** Module logic is reusable across different signatures.

### 4. Manual Optimization (Trial and Error)

**What it is:** The iterative process of finding the right way to ask each model.

```
# Optimization loop
1. Write initial prompt
2. Test on examples
3. Observe failures
4. Adjust phrasing/formatting
5. Repeat
```

**In our docs:** See "Reinforcement Review" in `20-PROMPT-PATTERNS.md` and the prompt tips throughout.

**Why it matters:** DSPy automates this with teleprompters. Until then, we do it manually.

---

## Mapping to Existing Concepts

| DSPy Term | Cursor Onboarding Kit Equivalent | Location |
|-----------|----------------------------------|----------|
| Signature | Semantic Prompt Address | `optional/specs/semantic/PROMPT-SEMANTIC-MAP.md` |
| Adapter | Prompt Patterns, Prompt Casting | `20-PROMPT-PATTERNS.md` |
| Module Logic | 5 Cs, Workflows, State Machine | `AGENTS.md`, `99-EPILOGUE-STATE-MACHINE.md` |
| Manual Optimization | Prompt Tips, Reinforcement Review | `20-PROMPT-PATTERNS.md`, `04-TIPS-AND-TRICKS-SCORECARD.md` |

---

## Practical Application

### When Writing a New Prompt

1. **Define the signature first:** What inputs? What outputs?
2. **Choose adapter format:** How will the model understand inputs? How will you parse outputs?
3. **Select module logic:** Which strategy fits? (Chain of thought, tool use, constrained output)
4. **Iterate:** Test, observe, adjust

### Example: Code Review Prompt

```yaml
# Signature
inputs: [code_diff, review_criteria]
outputs: [findings, severity, fixes]

# Adapter
format: |
  Review this diff for {review_criteria}:
  {code_diff}

  Return JSON: {"findings": [...], "severity": "high|medium|low"}

# Module Logic
strategy: chain_of_thought
prompt: |
  First, identify all changed areas.
  Then, check each against {review_criteria}.
  Finally, summarize findings by severity.

# Manual Optimization
iterations:
  - v1: Direct ask → too vague
  - v2: Added structure → better
  - v3: Added examples → best
```

---

## Why This Matters for Portability

Conventional prompts couple architecture with implementation:

```
❌ Coupled: "As an expert reviewer, think step by step about this Python code..."
```

DSPy-style prompts separate concerns:

```
✅ Separated:
  Signature: review(code, language) → findings
  Adapter: format_for_model(code, language)
  Module: chain_of_thought()
```

When you switch models, you only change the adapter. Signature and module logic stay the same.

---

## Integration with Semantic Mapping

The AQAL semantic map in `PROMPT-SEMANTIC-MAP.md` provides a universal addressing scheme. DSPy provides the implementation components.

```
Semantic Address: UR-L2-technical-active-direct
                   ↓
DSPy Components:
  - Signature: What UR-L2 expects (input: code, output: changes)
  - Adapter: How to format for current model
  - Module: Which strategy (direct, guided, exploratory)
  - Optimization: Iterative refinement
```

This combination makes prompts:
- **Discoverable** (semantic address)
- **Portable** (DSPy separation)
- **Optimizable** (explicit iteration)

---

## Further Reading

- [DSPy Documentation](https://dspy.ai/learn/programming/overview/)
- [DSPy GitHub](https://github.com/stanfordnlp/dspy)
- Our semantic mapping: `optional/specs/semantic/PROMPT-SEMANTIC-MAP.md`
- Our prompt patterns: `20-PROMPT-PATTERNS.md`
