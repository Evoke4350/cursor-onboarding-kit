# Cursor Agent Surfaces (CLI, Web, Bugbot, Enterprise)

This module documents advanced Cursor surfaces that are easy to miss in day-to-day editor use.

Scope:

- Cursor Agent CLI
- Cursor Web/Cloud Agent workflows
- Bugbot (PR review agent)
- Enterprise controls (service accounts, GitHub integration patterns)
- Update tracking and verification loop

---

## 1) Cursor Agent CLI (Terminal-First)

What it is:

- `agent` in terminal for interactive and non-interactive workflows
- supports Agent/Plan/Ask modes
- supports handoff to cloud agents (`&` prefix)

Why teams use it:

- headless workflows in shell scripts and CI-like automation
- easy integration with existing terminal-heavy engineering habits
- mode parity with editor workflows for planning vs execution

Docs:

- https://cursor.com/docs/cli/overview
- https://cursor.com/docs/cli/reference/parameters
- https://cursor.com/docs/cli/reference/permissions

### Shell Environment and External CLI Tools

Cursor terminal and agent shell workflows can use tools available in your shell environment (`PATH`, exported vars, profiles).

What this enables:

- use local CLIs directly (`psql`, `aws`, `react-native`, `expo`, custom binaries)
- compose Cursor with personal/team toolchains instead of replacing them
- expose additional tools by installing them and ensuring shell/profile loading is correct

Practical note:

- if expected tools are missing in terminal sessions, verify shell profile loading and integrated shell settings first

References:

- https://docs.cursor.com/configuration/shell
- https://cursor.com/docs/agent/terminal

---

## 2) Cursor Web/Cloud Agents

What it is:

- agent runs that continue outside local editor sessions
- accessible on web/mobile (`cursor.com/agents`)
- programmatic launch and follow-ups through API

Key capabilities to know:

- launch an agent against repo/ref/PR
- follow up on existing run
- stop/delete runs
- list accessible repositories (rate-limited endpoint)
- optional PR auto-creation behavior

Docs:

- https://cursor.com/docs/cloud-agent
- https://cursor.com/docs/cloud-agent/api/endpoints
- https://cursor.com/agents

---

## 3) Web Agent Advanced Usage

Useful advanced patterns:

- queue long-running refactors from web while local IDE stays free
- launch against PR/ref, then do follow-up prompts on the same run
- split runs by task shape (explore, implement, review) to reduce context pollution
- keep cloud-run output tied to PR comments for traceability

Good fit:

- large repo scans
- repetitive doc or migration work
- overnight non-blocking tasks

Agent Browser note:

- tools like Vercel `agent-browser` can extend automation workflows (web + iOS simulator/device automation)
- useful for repeatable UI/debug automation when paired with clear constraints and verification checks

Reference:

- https://github.com/vercel-labs/agent-browser

---

## 4) GitHub Integration (Web + Agent Workflows)

What it enables:

- cloud agents working from PR/issue comments
- repo cloning and PR creation from agent runs
- optional team-level IP allowlist setup for restricted orgs

Operational note:

- for enterprise automation, team-level GitHub app install matters more than individual setup

Docs:

- https://cursor.com/docs/integrations/github

---

## 5) Bugbot (Cursor Bot for PR Review)

What it is:

- automated PR review agent that comments on likely bugs and issues
- supports repo-level enablement and review triggers

How it fits this kit:

- run Bugbot as backpressure layer before merge
- route findings into the same evidence/risk/rollback workflow used in this kit

Docs:

- https://cursor.com/docs/bugbot
- https://cursor.com/bugbot

---

## 6) Bugbot Advanced Usage

Practical advanced patterns:

- treat Bugbot findings as triage queues, not auto-fix mandates
- dismiss only with explicit rationale in PR thread
- use "Fix in Web" for quick patch loops and "Fix in Cursor" for deeper local validation
- rerun Bugbot after risky edits or rebases

Reviewer workflow:

1. classify finding (bug, style, false positive, out-of-scope)
2. choose fix path (web or local)
3. require tests for behavior changes
4. close thread with rationale and result

---

## 7) Who Can Use What (Reality)

- most surfaces are broadly available, but access depends on plan, org policy, and repo integration state
- enterprise admins can restrict integrations, permissions, MCP, or network behavior
- service accounts/API automation are typically enterprise-governed
- if something is missing, check workspace policy and GitHub integration first

---

## 8) Enterprise Controls and API Keys

What is relevant:

- service accounts for non-human automation
- API key lifecycle (creation/rotation/revocation)
- centralized admin visibility and usage governance
- GitHub team-level integration requirements for service account repo access

Docs:

- https://cursor.com/docs/account/enterprise/service-accounts

Practical interpretation:

- if you want "open-router-like" internal automation patterns, start with official Cloud Agents API + service accounts
- treat this as governed platform automation, not ad hoc personal scripting

How API keys work in practice:

- model-provider keys (OpenAI/Anthropic/etc) are configured in Cursor model settings for BYOK model usage
- Cursor API keys are used for Cursor API surfaces (for example Cloud Agent API)
- enterprise service accounts provide non-human keys for governed automation
- repo automation depends on team GitHub integration and repo authorization

Docs:

- https://cursor.com/docs/settings/api-keys
- https://cursor.com/docs/api
- https://cursor.com/docs/account/enterprise/service-accounts

Open-router-style proxy pattern:

- yes, teams can run proxy layers to route Cursor-compatible traffic to alternate backends
- this is usually community tooling, not official Cursor product behavior
- treat proxy mode as custom infrastructure with security ownership (auth, logging, key handling, abuse controls)

Community examples (unofficial):

- https://github.com/pezzos/cursor-openrouter-proxy
- https://github.com/danilofalcao/cursor-deepseek

---

## 9) Secrets and Vault Hygiene

Short answer:

- Cursor supports API key configuration and enterprise service-account keys
- Cursor does not position itself as your full secrets vault platform
- for team-grade secret hygiene, pair Cursor with an external secrets manager

Recommended pattern:

1. store secrets in a dedicated vault
2. inject secrets just-in-time at runtime (hooks/CI/session), not in repo files
3. keep AI-visible context free of raw credentials
4. rotate keys and enforce short-lived access where possible

Good ecosystem options:

- 1Password hooks integration (just-in-time secrets)
- Akeyless extension/integration
- Infisical (open-source secrets platform)
- HashiCorp Vault OSS (mature vault baseline)
- SOPS + age (GitOps/file-encryption workflow)

References:

- https://cursor.com/docs/settings/api-keys
- https://cursor.com/docs/api
- https://cursor.com/docs/account/enterprise/service-accounts
- https://cursor.com/docs/agent/hooks
- https://1password.com/blog/bringing-secure-just-in-time-secrets-to-cursor-with-1password
- https://developer.1password.com/docs/cursor-hooks
- https://docs.akeyless.io/docs/cursor-akeyless-secrets-manager
- https://github.com/Infisical/infisical
- https://github.com/hashicorp/vault
- https://github.com/getsops/sops

---

## 10) Doc Sync (Fast)

Primary docs:

- Changelog page: https://www.cursor.com/changelog
- Alternate changelog host: https://changelog.cursor.sh/

Quick check:

- run `scripts/check-cursor-docs.sh` to fetch key pages with `curl` and validate expected markers

---

## 11) YOLO / Auto-Run Mode (Speed vs Safety)

What it is:

- a high-autonomy mode where the agent can execute more actions with fewer prompts
- useful for tightly scoped, low-risk, or disposable environments

Important caution:

- this can dramatically increase speed and blast radius at the same time
- use only with explicit boundaries and rollback confidence

Practical guidance:

- prefer sandboxed or constrained permissions first
- for sensitive repos, default to approval mode and explicit allowlists
- if using broad allow rules, keep scope narrow and monitor command output continuously

Docs:

- https://cursor.com/docs/cli/reference/permissions
- https://cursor.com/docs/agent/security
- https://cursor.com/docs/agent/terminal

Note:

- shortcut behavior and approval UX can vary by version and surface (editor vs CLI)
- treat "approve all" habits as temporary accelerators, not baseline policy
- keep personal high-autonomy aliases local-only (do not publish as team defaults)

Troubleshooting bonus:

- use `40-TEMPLATES/INSTRUCTION-STARTER-PACK/YOLO-TROUBLESHOOT-BONUS.sh` to inspect local CLI approval/sandbox posture before changing policies

---

## 12) Beta / Experimental Features

Use beta features when:

- your team can absorb occasional instability
- you can isolate experiments from production-critical work
- you have a rollback path

Use stable path when:

- onboarding new engineers
- running regulated/compliance-heavy workflows
- operating under strict enterprise controls

Suggested practice:

- test beta capabilities in one pilot repo first
- document observed behavior and version notes
- promote only proven patterns to team defaults

---

## 13) Enterprise Lock-Down Reality

In enterprise environments, some controls are intentionally restricted:

- MCP availability can be policy-governed
- approval/permissions may be centrally constrained
- service-account access depends on org-level integration setup
- security tooling may be mandatory in your workflow

Treat this as design input, not friction:

- optimize prompts and boundaries within policy
- encode required checks into your standard operating loop
- avoid "works on my machine" assumptions for automation

---

## 14) Sandboxes (Execution Safety Layer)

Use sandboxes to reduce blast radius while preserving agent speed.

Recommended approach:

1. local sandbox and permission boundaries first
2. disposable branch and clean rollback path
3. external sandbox service only when team workflow needs it

Optional service example:

- Daytona can run isolated sandboxes for automation-heavy workflows.
- `nono` is an OSS kernel-enforced capability sandbox for agent/CLI workloads.

Docs:

- https://daytona.io/docs/en
- https://www.daytona.io/docs/en/sandboxes/
- https://github.com/always-further/nono

Team guidance:

- treat external sandbox services as infrastructure decisions
- run a small pilot before broad rollout
- document cost, latency, and credential handling trade-offs

---

## 15) WASM and Kernel-Level Sandboxes (What Is "Standard"?)

Short answer:

- there is no single universal "agent sandbox standard" yet
- teams typically combine multiple layers depending on risk

Current common patterns:

- WASM runtime isolation (for tool/plugin execution) using engines such as Wasmtime
- kernel-level isolation (for arbitrary code) using primitives such as gVisor, Kata, or Firecracker
- managed sandbox platforms for faster adoption when infra ownership is not desired

Practical recommendation:

- use WASM-style sandboxing for bounded tools/components
- use kernel-level isolation for untrusted general code execution
- treat managed services as an ops trade-off (speed vs control)

References:

- https://developer.nvidia.com/blog/sandboxing-agentic-ai-workflows-with-webassembly/
- https://opensource.microsoft.com/blog/2025/08/06/introducing-wassette-webassembly-based-tools-for-ai-agents/
- https://www.e2b.dev/

---

## 16) Cursor Out-of-the-Box Safety Baseline

What Cursor includes by default (verify on your version):

- approvals for sensitive actions, especially terminal and privileged integrations
- scoped permissions and approval modes in CLI config
- security controls documented for agent operations and integrations
- optional platform sandboxing support depending on OS/version surface

Key docs:

- https://cursor.com/docs/agent/security
- https://cursor.com/docs/cli/reference/permissions
- https://cursor.com/docs/agent/terminal

Operator note:

- default posture is usually safer than ad hoc "approve everything"
- YOLO/auto-run shortcuts should be treated as temporary acceleration in controlled contexts

---

## 17) Destructive Command Guardrails

Base layer (official Cursor controls):

- permission policies and allow/deny rules
- agent security settings
- approval mode for risky command classes

Docs:

- https://cursor.com/docs/cli/reference/permissions
- https://cursor.com/docs/agent/security

Optional hardening layer (third-party):

- tools like `destructive_command_guard` can add extra blocking logic for dangerous shell commands
- useful in high-risk repos; evaluate operational overhead before team-wide rollout

Reference:

- https://github.com/Dicklesworthstone/destructive_command_guard

Policy note:

- do not rely on one guardrail
- combine permissions, branch strategy, reviews, and runtime checks

---

## 18) Local Backpressure with Lefthook

Lefthook is a practical local hook runner for pre-commit and pre-push checks.

Why it fits this kit:

- strong local backpressure
- can be adopted personally before team-wide rollout
- supports local override files for personal workflows

Docs:

- https://lefthook.dev/
- https://lefthook.dev/usage/commands.html
- https://lefthook.dev/usage/features.html

Suggested pattern:

- keep team hook config lightweight
- keep personal extras in local override files
- avoid turning hooks into slow, brittle pipelines

---

## 19) Bugbot Web Workflow (Practical)

What teams usually miss:

- Bugbot comments can include "Fix in Web" and "Fix in Cursor" entry points
- these are not just review comments; they can start fix workflows

Baseline flow:

1. Bugbot flags issue on PR
2. reviewer decides: fix now, defer, or dismiss with reason
3. trigger fix via web/editor entry point
4. agent proposes changes
5. run verification gates before merge

Manual trigger examples:

- `cursor review`
- `bugbot run`
- `@cursor fix` (when enabled and appropriate)

Docs:

- https://cursor.com/docs/bugbot
- https://cursor.com/docs/integrations/github

---

## 20) Update Tracking (Official + Third-Party)

Official sources:

- https://www.cursor.com/changelog
- https://changelog.cursor.sh/

Third-party monitor option:

- release trackers (for example Releasebot) can provide feed/alert wrappers around Cursor releases

Reference:

- https://releasebot.io/updates/cursor

Security note:

- treat third-party feeds as convenience, not source of truth
- confirm important changes against official Cursor docs/changelog

---

## 21) Agent Traces and Observability

Official Cursor surfaces:

- chat history for prior agent conversations
- shared transcripts for review, handoff, and audit-style context
- team controls and plan features can affect what is available

Docs:

- https://cursor.com/docs/agent/chat/history
- https://cursor.com/docs/shared-transcripts

Open-source observability tools (optional):

- Gryph: local-first audit trail across coding agents, including Cursor hooks
- CodexBar: local usage visibility and model/session monitoring, including Cursor
- cursor-otel-hook: OpenTelemetry-style instrumentation path

References:

- https://github.com/safedep/gryph
- https://github.com/steipete/codexbar
- https://github.com/LangGuard-AI/cursor-otel-hook

Who can use these:

- generally anyone can use OSS tools locally
- org policy may restrict local hooks, telemetry forwarding, or extension installs
- treat these tools as optional overlays, not required baseline workflow

UI expectations:

- CodexBar provides a macOS menu-bar UI for usage visibility
- Gryph is primarily hook + local database + CLI workflow (not a polished web dashboard by default)
- if you need dashboards, pair trace capture with your own BI/OTel stack

Compatibility note:

- CodexBar advertises Cursor support, but community reports include occasional plan-detection edge cases
- Gryph integrates through Cursor hooks and is better for action/audit traces than plan-usage meters

Workflow note:

- tools like DevSQL can query local history/transcript + git patterns to support reinforcement loops
- use results to improve workflow policy and prompt structure, not to overfit one-off sessions

---

## 22) Voice Dictation Input (Optional)

For prompt-heavy workflows, local dictation can speed up ideation and command entry.

Built-in note:

- Cursor voice-capable input features may be available depending on version/settings
- keep voice workflows optional and verify privacy posture before team rollout

Practical options:

- Handy: push-to-talk dictation into any text box, local-first
- whisper.cpp: local CLI transcription pipeline
- MacWhisper-class apps: local desktop transcription UX

References:

- https://handy.computer/
- https://github.com/cjpais/Handy
- https://github.com/ggerganov/whisper.cpp

Guidance:

- default to local transcription for sensitive repos
- test latency/accuracy before team rollout
- keep voice tools optional; keyboard-first workflow remains baseline

---

## 23) Extension and Module Caution

For security-sensitive teams:

- avoid recommending random extension stacks by default
- prefer built-in Cursor capabilities first
- add third-party modules only after security review and clear ownership

If your personal setup enables many modules:

- document which are mandatory vs optional
- validate that baseline workflow still works without extras
- keep enterprise-safe defaults in team docs

---

## 24) Lightweight Maintenance Loop

- run `scripts/check-cursor-docs.sh`
- update links that fail or redirect unexpectedly
- keep guidance short and operational

---

## 25) Weekly Insights Workflow (Cursor Equivalent)

Cursor does not currently document a native built-in `/insights` command equivalent.

Practical equivalent in Cursor:

1. define `/weekly-insights` in `.cursor/commands/weekly-insights.md`
2. attach a skill in `.cursor/skills/weekly-insights/SKILL.md`
3. feed local evidence (history, transcripts, commits, review notes, optional DevSQL output)
4. require output schema: friction patterns, high-yield prompts, policy updates, stop-list
5. promote only evidence-backed deltas into `AGENTS.local.md` or team rules

Why this works:

- command gives repeatable invocation
- skill gives reusable procedure
- evidence schema prevents weak, non-evidence recommendations
