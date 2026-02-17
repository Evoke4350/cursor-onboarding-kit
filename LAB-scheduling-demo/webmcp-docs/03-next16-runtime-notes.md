# Next.js 16 + MCP Runtime Notes (Agent-Facing)

## Why This Exists

New framework/runtime guidance can outpace model pre-training.  
For MCP work, retrieve docs first and treat memory as suspect.

## Implementation Shape (Current Practice)

- Keep MCP route small, deterministic, and tool-focused.
- Validate every JSON-RPC request envelope before dispatch.
- Enforce explicit auth header checks before tool execution.

## Agent Prompting Rule

When task mentions WebMCP or MCP route behavior:

1. Read `01-protocol-delta-2025-11-25.md`.
2. Read `02-vercel-hosting-and-cli.md`.
3. Only then propose route/tool changes.

## Minimal Conformance Checklist

- `tools/list` returns tool schema with required args.
- `tools/call` validates tool name and argument shape.
- Unauthorized requests produce structured JSON-RPC error.
- Route handles unknown methods with `Method not found`.

## Lab Safety Constraint

Do not mix protocol-conformance work with scheduling-logic bug fixes in one PR.
