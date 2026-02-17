# WebMCP Agent Quickstart (Compressed)

This is an **agent-first** interface for the lab scheduling demo.  
Humans can click it, but the contract is meant for tools.

## 1) Boot

```bash
cd cursor-onboarding-kit/LAB-scheduling-demo
export AGENT_DEV_KEY=lab-agent-key
tsx ui/server.ts
```

UI:

- Dashboard: [http://localhost:4173](http://localhost:4173)
- Agent menu: [http://localhost:4173/agent/dev-menu?agent=1](http://localhost:4173/agent/dev-menu?agent=1)

## 2) MCP Endpoint

- URL: `POST /mcp`
- Auth header: `x-agent-key: $AGENT_DEV_KEY`
- Protocol: JSON-RPC 2.0
- Tool exposed: `generate_triage_packet`

## 2.5) Retrieval-First Docs (Required Before MCP Edits)

Read these first:

1. `webmcp-docs/00-read-order.md`
2. `webmcp-docs/01-protocol-delta-2025-11-25.md`
3. `webmcp-docs/02-vercel-hosting-and-cli.md`
4. `webmcp-docs/03-next16-runtime-notes.md`

Reason: WebMCP/Next runtime behavior evolves faster than model memory.

## 3) List Tools

```bash
curl -s http://localhost:4173/mcp \
  -H "content-type: application/json" \
  -H "x-agent-key: lab-agent-key" \
  -d '{"jsonrpc":"2.0","id":"1","method":"tools/list","params":{}}'
```

## 4) Call Tool

```bash
curl -s http://localhost:4173/mcp \
  -H "content-type: application/json" \
  -H "x-agent-key: lab-agent-key" \
  -d '{
    "jsonrpc":"2.0",
    "id":"2",
    "method":"tools/call",
    "params":{
      "name":"generate_triage_packet",
      "arguments":{"scenarioId":"escalation-hole","ticketHint":"SCHED-2427"}
    }
  }'
```

## 5) What The Tool Returns

- Scenario summary (`coverage`, `fairness`, gap count, conflicts)
- Detected issue list (high-signal bug indicators)
- Recommended ticket id
- Next command checklist

## 6) Agent-Only Autopilot Loop Demo

In a second terminal (with server still running):

```bash
cd cursor-onboarding-kit/LAB-scheduling-demo
export AGENT_DEV_KEY=lab-agent-key
AUTOPILOT_CYCLES=3 AUTOPILOT_INTERVAL_MS=2500 tsx ui/agent-autopilot-demo.ts
```

Output lands in:

- `agent-autopilot-output/events.ndjson`
- `agent-autopilot-output/tickets/*.md`
- `agent-autopilot-output/pr-drafts/*.md`

This simulates an agent that continuously:

1. polls `/mcp`
2. triages scenarios
3. proposes ticket/PR artifacts automatically
4. reruns characterization baseline checks

Security/limits knobs:

- `AUTOPILOT_ALLOWED_URL_PREFIXES` (default allows localhost prefixes)
- `AUTOPILOT_ALLOWED_HTTP_METHODS` (default `POST`)
- `AUTOPILOT_MAX_COMMAND_MS` (default `12000`)
- `AUTOPILOT_MAX_CYCLE_MS` (default `45000`)
- `AUTOPILOT_MAX_FILE_WRITES` (default `64`)

## 7) Notes

- This preserves intentional lab bugs.
- This is a minimal WebMCP-style shim, not a full production MCP gateway.
- If you need strict spec conformance, write protocol tests first.

## 8) Docker Log-Debug Drill

Build and run:

```bash
cd cursor-onboarding-kit/LAB-scheduling-demo
docker build -t scheduling-lab:2026 .
docker run --rm -p 4173:4173 --name scheduling-lab scheduling-lab:2026
```

The container is intentionally misconfigured (`PUBLIC_DIR_NAME=public-missing`), so static UI reads fail.
Use `docker logs scheduling-lab` to diagnose, then rerun with `-e PUBLIC_DIR_NAME=public`.
