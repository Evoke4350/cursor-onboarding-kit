# Scheduling Demo Service

A lightweight scheduling service that models weekly on-call operations for small engineering teams.

North star: spot who needs a nudge before they burn out.

Training note: this lab includes intentionally planted defects and noisy context artifacts so teams can practice triage under ambiguity.

## Overview

The service provides:

- rotation and shift assignment primitives
- availability conflict detection
- escalation-aware notification routing
- weekly coverage and fairness reporting
- a small web dashboard for scenario visualization
- an MCP-style JSON-RPC endpoint for agent/tool integration

## Core Modules

- `src/rotation.ts`: shift selection and load distribution helpers
- `src/availability.ts`: time-off overlap and conflict checks
- `src/notifications.ts`: urgency/channel routing and summaries
- `src/scheduleReport.ts`: coverage, gaps, and member summaries
- `ui/server.ts`: HTTP server exposing UI, API, and MCP routes

## Runtime Interfaces

### Dashboard

- `GET /` -> scenario dashboard UI
- `GET /agent/dev-menu?agent=1` -> agent-only dev menu

### Scenario API

- `GET /api/scenarios` -> list available scenarios
- `GET /api/scenario?id=<scenario-id>` -> scenario detail payload

### MCP Endpoint

- `POST /mcp`
- Required header: `x-agent-key`
- Supported methods:
  - `tools/list`
  - `tools/call` (`generate_triage_packet`)

## Local Development

```bash
cd cursor-onboarding-kit/LAB-scheduling-demo
export AGENT_DEV_KEY=lab-agent-key
tsx ui/server.ts
```

Default URL: `http://localhost:4173`

Use a different port:

```bash
PORT=4310 tsx ui/server.ts
```

## Tests

```bash
cd cursor-onboarding-kit/LAB-scheduling-demo
tsx --test src/__tests__/*.ts
```

## Verification Upgrades (Property, E2E, Visual)

Agents are hypothesis generators; verification is the truth machine.

This lab includes optional verification layers you can add on top:

- Property-based tests (`fast-check`)
- End-to-end tests (`@playwright/test`)
- Visual regression (Playwright screenshots)
- Property-based UI testing (Bombadil)

See `VERIFICATION.md` for how to run them and why they matter.

## Docker

Build:

```bash
cd cursor-onboarding-kit/LAB-scheduling-demo
docker build -t scheduling-demo:latest .
```

Run:

```bash
docker run --rm -p 4173:4173 --name scheduling-demo scheduling-demo:latest
```

## MCP Smoke Check

```bash
curl -s http://localhost:4173/mcp \
  -H "content-type: application/json" \
  -H "x-agent-key: lab-agent-key" \
  -d '{"jsonrpc":"2.0","id":"1","method":"tools/list","params":{}}'
```

## Repository Notes

- `webmcp-docs/` contains compressed protocol/runtime notes for MCP-related work.
- `40-TEMPLATES/INSTRUCTION-STARTER-PACK/` contains optional AGENTS templates for teams that want to add agent memory files.

## License

This project uses a mixed-license model (similar to choosealicense.com):

- Source code and runtime assets are licensed under MIT (`LICENSE`).
- Documentation and instructional content are licensed under CC BY 4.0 (`LICENSE-CONTENT.md`).
