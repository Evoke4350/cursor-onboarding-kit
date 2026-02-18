# Setup Guide: Self-Configuring Workshop

**The core principle**: Describe what you want, the system guides you through setup.

---

## The Magic Phrase

> "Ask me clarifying questions until you know what I want to build and walk me through the setup step by step."

This is the only prompt you need. The system will:
1. Ask clarifying questions to understand your needs
2. Derive configuration from your answers
3. Walk you through each step
4. Generate your workshop automatically

---

## Why This Works

From arXiv:2510.08191 (Training-Free GRPO):

> "LLMs can achieve similar output distribution effects by learning experiential knowledge as a token prior."

The system doesn't need a human expert to configure it. It:
- **Derives** configuration from conversation (arscontexta pattern)
- **Stores** your preferences as markdown (experience library)
- **Injects** them as context (token prior)

You describe → System derives → Workshop generates

---

## Example Session

```
You: Ask me clarifying questions until you know what I want to build
     and walk me through the setup step by step.

Agent: I'll help you set up your workshop. Let me ask a few questions:

       1. What kind of work do you do? (software/research/writing/other)
       2. Do you want your notes to reference code files?
       3. Where should I create your workshop?

You: Software development, yes reference code, in ~/projects/my-app/workshop

Agent: Got it. Here's what I'll create:

       ~/projects/my-app/workshop/
       ├── bench/          # Your workspace overview
       ├── shavings/       # Code insights
       ├── sawdust/        # Session logs
       └── .workshop/      # Configuration

       Security: I'll set up taint analysis for .secrets/ and block
       exfil commands when tainted.

       Proceed? (yes/no)

You: yes

Agent: [Creates folders, generates identity.md, sets up hooks]

       ✓ Workshop created!
       ✓ Security configured
       ✓ 5 Cs commands available

       Next: Try `workshop cut src/main.rs` to extract your first insight.
```

---

## What Gets Derived

From your answers, the system derives:

| Question | Configuration |
|----------|--------------|
| Work type | Domain vocabulary (software→functions, research→claims) |
| Code refs? | `--with-code-ref` default behavior |
| Location | Workshop path |
| Security level | Policy pack (basic/paranoid/development) |
| Search mode | ripgrep only vs semantic search |

---

## The 6-Phase Process

From arscontexta:

1. **Detect** — What environment, what tools available
2. **Understand** — Ask questions, extract signals
3. **Derive** — Map signals to configuration dimensions
4. **Propose** — Show what will be generated
5. **Generate** — Create folders, templates, hooks
6. **Validate** — Smoke test, first-success guidance

You don't need to know any of this. Just describe what you want.

---

## No Human Expert Needed

Traditional setup:
- Read documentation
- Edit config files
- Debug misconfigurations
- Ask expert for help

This system:
- Describe what you want
- Answer questions
- Get working workshop

**The experience library configures itself.**
