# WebMCP Integration Quickstart

This document covers the MCP-style endpoint exposed by `LAB-scheduling-demo`.

## Start the Service

```bash
cd cursor-onboarding-kit/LAB-scheduling-demo
export AGENT_DEV_KEY=lab-agent-key
tsx ui/server.ts
```

## Endpoint Contract

- URL: `POST /mcp`
- Protocol: JSON-RPC 2.0
- Auth header: `x-agent-key: $AGENT_DEV_KEY`

## List Tools

```bash
curl -s http://localhost:4173/mcp \
  -H "content-type: application/json" \
  -H "x-agent-key: lab-agent-key" \
  -d '{"jsonrpc":"2.0","id":"1","method":"tools/list","params":{}}'
```

## Call `generate_triage_packet`

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

## Related Docs

- `webmcp-docs/00-read-order.md`
- `webmcp-docs/01-protocol-delta-2025-11-25.md`
- `webmcp-docs/02-hosting-and-cli-patterns.md`
