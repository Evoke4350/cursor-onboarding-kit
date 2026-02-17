import { appendFile, mkdir, writeFile } from "node:fs/promises";
import { join } from "node:path";
import { spawnSync } from "node:child_process";

type JsonRpcEnvelope = {
  jsonrpc: "2.0";
  id: string;
  method: string;
  params: Record<string, unknown>;
};

type JsonRpcError = {
  code: number;
  message: string;
};

type JsonRpcResponse<T> = {
  jsonrpc: "2.0";
  id: string | number | null;
  result?: T;
  error?: JsonRpcError;
};

type TriagePacket = {
  objective: "triage_packet";
  scenarioId: string;
  scenarioTitle: string;
  generatedAt: string;
  summary: {
    teamCoverage: number;
    fairnessScore: number;
    teamGapCount: number;
    notificationCount: number;
    conflictCount: number;
  };
  detectedIssues: string[];
  recommendedTicket: string;
  notes: string;
  nextCommands: string[];
};

type ToolCallResult = {
  structuredContent?: TriagePacket;
};

type TriageSummary = {
  severity: "SEV-1" | "SEV-2" | "SEV-3";
  action: string;
};

type TestStatus = {
  ok: boolean;
  passCount: number | null;
  failCount: number | null;
};

type AgentCommand<TArgs, TResult> = {
  name: string;
  run: (args: TArgs, context: CommandContext) => Promise<TResult>;
};

type CommandContext = {
  cycle: number;
  scenarioId?: string;
};

const baseUrl = process.env.MCP_BASE_URL ?? "http://localhost:4173";
const agentKey = process.env.AGENT_DEV_KEY ?? "lab-agent-key";
const outputDir = process.env.AUTOPILOT_OUTPUT_DIR ?? "agent-autopilot-output";
const maxCycles = Number(process.env.AUTOPILOT_CYCLES ?? "3");
const intervalMs = Number(process.env.AUTOPILOT_INTERVAL_MS ?? "4000");
const maxCommandRuntimeMs = Number(
  process.env.AUTOPILOT_MAX_COMMAND_MS ?? "12000",
);
const maxCycleRuntimeMs = Number(process.env.AUTOPILOT_MAX_CYCLE_MS ?? "45000");
const maxFileWrites = Number(process.env.AUTOPILOT_MAX_FILE_WRITES ?? "64");
const allowedHttpMethods = new Set(
  (process.env.AUTOPILOT_ALLOWED_HTTP_METHODS ?? "POST")
    .split(",")
    .map((x) => x.trim().toUpperCase())
    .filter(Boolean),
);
const scenarios = (
  process.env.AUTOPILOT_SCENARIOS ??
  "escalation-hole,timezone-ghost-conflict,coverage-inflation"
)
  .split(",")
  .map((s) => s.trim())
  .filter(Boolean);

const mcpUrl = new URL("/mcp", baseUrl).toString();
const allowedUrlPrefixes = (
  process.env.AUTOPILOT_ALLOWED_URL_PREFIXES ?? `${baseUrl},http://localhost:,http://127.0.0.1:`
)
  .split(",")
  .map((x) => x.trim())
  .filter(Boolean);

const ticketsDir = join(outputDir, "tickets");
const draftsDir = join(outputDir, "pr-drafts");
const logPath = join(outputDir, "events.ndjson");
let usedFileWrites = 0;

function defineCommand<TArgs, TResult>(
  name: string,
  run: (args: TArgs, context: CommandContext) => Promise<TResult>,
): AgentCommand<TArgs, TResult> {
  return { name, run };
}

async function invokeCommand<TArgs, TResult>(
  command: AgentCommand<TArgs, TResult>,
  args: TArgs,
  context: CommandContext,
): Promise<TResult> {
  const startedAt = Date.now();
  let result: TResult | undefined;
  let failure: unknown;

  try {
    result = await command.run(args, context);
  } catch (error) {
    failure = error;
  }

  const elapsedMs = Date.now() - startedAt;
  if (elapsedMs > maxCommandRuntimeMs) {
    throw new Error(
      `Command ${command.name} exceeded ${maxCommandRuntimeMs}ms (actual ${elapsedMs}ms)`,
    );
  }
  if (failure) throw failure;

  return result as TResult;
}

function enforceNetworkPolicy(): void {
  if (!allowedHttpMethods.has("POST")) {
    throw new Error("Policy violation: POST must be allowed for JSON-RPC transport");
  }

  const parsed = new URL(mcpUrl);
  if (!["http:", "https:"].includes(parsed.protocol)) {
    throw new Error(`Policy violation: unsupported MCP protocol ${parsed.protocol}`);
  }

  if (!allowedUrlPrefixes.some((prefix) => mcpUrl.startsWith(prefix))) {
    throw new Error(
      `Policy violation: MCP URL ${mcpUrl} not in AUTOPILOT_ALLOWED_URL_PREFIXES`,
    );
  }
}

function enforceCycleBudget(cycleStart: number, cycle: number): void {
  const elapsedMs = Date.now() - cycleStart;
  if (elapsedMs > maxCycleRuntimeMs) {
    throw new Error(
      `Cycle ${cycle} exceeded ${maxCycleRuntimeMs}ms (actual ${elapsedMs}ms)`,
    );
  }
}

function consumeWriteBudget(count = 1): void {
  usedFileWrites += count;
  if (usedFileWrites > maxFileWrites) {
    throw new Error(
      `Write budget exceeded: ${usedFileWrites}/${maxFileWrites} file writes`,
    );
  }
}

function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function callMcp<T>(
  method: string,
  params: Record<string, unknown>,
): Promise<T> {
  const payload: JsonRpcEnvelope = {
    jsonrpc: "2.0",
    id: `${Date.now()}-${Math.random().toString(36).slice(2, 8)}`,
    method,
    params,
  };

  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), maxCommandRuntimeMs);

  try {
    const res = await fetch(mcpUrl, {
      method: "POST",
      headers: {
        "content-type": "application/json",
        "x-agent-key": agentKey,
      },
      body: JSON.stringify(payload),
      signal: controller.signal,
    });

    if (!res.ok) {
      throw new Error(`MCP HTTP ${res.status}`);
    }

    const body = (await res.json()) as JsonRpcResponse<T>;
    if (body.error) {
      throw new Error(`MCP error ${body.error.code}: ${body.error.message}`);
    }
    if (!body.result) {
      throw new Error("MCP returned empty result");
    }

    return body.result;
  } finally {
    clearTimeout(timeout);
  }
}

function classifyIncident(packet: TriagePacket): TriageSummary {
  const issueText = packet.detectedIssues.join(" | ").toLowerCase();

  if (
    issueText.includes("critical alerts routed to email") ||
    issueText.includes("no escalation fallback")
  ) {
    return {
      severity: "SEV-1",
      action: "Patch escalation and critical routing path immediately",
    };
  }

  if (
    issueText.includes("coverage inflation") ||
    issueText.includes("conflicts from unapproved")
  ) {
    return {
      severity: "SEV-2",
      action: "Prepare scoped logic fix and validate with targeted tests",
    };
  }

  return {
    severity: "SEV-3",
    action: "Monitor and collect more evidence before code changes",
  };
}

function runCharacterizationTests(): TestStatus {
  const proc = spawnSync("/bin/zsh", ["-lc", "tsx --test src/__tests__/*.ts"], {
    encoding: "utf8",
    maxBuffer: 2 * 1024 * 1024,
    timeout: maxCommandRuntimeMs,
    killSignal: "SIGKILL",
  });

  if (proc.error && proc.error.message.includes("ETIMEDOUT")) {
    throw new Error(`Characterization tests timed out at ${maxCommandRuntimeMs}ms`);
  }

  const output = `${proc.stdout ?? ""}\n${proc.stderr ?? ""}`;
  const pass = output.match(/ℹ pass (\d+)/);
  const fail = output.match(/ℹ fail (\d+)/);

  return {
    ok: proc.status === 0,
    passCount: pass ? Number(pass[1]) : null,
    failCount: fail ? Number(fail[1]) : null,
  };
}

async function writeTicketUpdate(
  ticketId: string,
  packet: TriagePacket,
  severity: string,
  testStatus: TestStatus,
): Promise<void> {
  consumeWriteBudget();

  const now = new Date().toISOString();
  const path = join(ticketsDir, `${ticketId}.md`);
  const body = [
    `# ${ticketId} Autopilot Update`,
    ``,
    `Updated: ${now}`,
    `Scenario: ${packet.scenarioTitle} (${packet.scenarioId})`,
    `Severity: ${severity}`,
    ``,
    `## Detected Issues`,
    ...packet.detectedIssues.map((x) => `- ${x}`),
    ``,
    `## Test Baseline`,
    `- characterization_pass: ${testStatus.ok}`,
    `- pass_count: ${testStatus.passCount ?? "unknown"}`,
    `- fail_count: ${testStatus.failCount ?? "unknown"}`,
    ``,
    `## Suggested Next Commands`,
    ...packet.nextCommands.map((x) => `- \`${x}\``),
    ``,
  ].join("\n");

  await writeFile(path, body, "utf8");
}

async function writePrDraft(
  ticketId: string,
  packet: TriagePacket,
  severity: string,
  action: string,
): Promise<void> {
  consumeWriteBudget();

  const path = join(draftsDir, `${ticketId}-autopilot-pr.md`);
  const body = [
    `# PR Draft: ${ticketId} Autopilot Proposal`,
    ``,
    `## Summary`,
    `Autopilot detected \`${severity}\` conditions from scenario \`${packet.scenarioId}\` and recommends a scoped fix PR.`,
    ``,
    `## Why`,
    action,
    ``,
    `## Scope`,
    `- target_ticket: ${ticketId}`,
    `- scenario_source: ${packet.scenarioTitle}`,
    `- preserve_characterization_baseline: true`,
    ``,
    `## Test Plan`,
    `- Run \`tsx --test src/__tests__/*.ts\``,
    `- Add targeted truth tests for this ticket`,
    `- Keep legacy characterization tests labeled as legacy`,
    ``,
  ].join("\n");

  await writeFile(path, body, "utf8");
}

async function appendEventLog(event: Record<string, unknown>): Promise<void> {
  consumeWriteBudget();
  await appendFile(logPath, `${JSON.stringify(event)}\n`, "utf8");
}

async function ensureDirs(): Promise<void> {
  await mkdir(outputDir, { recursive: true });
  await mkdir(ticketsDir, { recursive: true });
  await mkdir(draftsDir, { recursive: true });
}

const listToolsCommand = defineCommand<{}, { tools: unknown[] }>(
  "mcp_list_tools",
  async () => await callMcp<{ tools: unknown[] }>("tools/list", {}),
);

const runTestsCommand = defineCommand<{}, TestStatus>(
  "run_characterization_tests",
  async () => runCharacterizationTests(),
);

const generatePacketCommand = defineCommand<
  { scenarioId: string; ticketHint?: string },
  TriagePacket
>("mcp_generate_triage_packet", async ({ scenarioId, ticketHint }) => {
  const toolCall = await callMcp<ToolCallResult>("tools/call", {
    name: "generate_triage_packet",
    arguments: { scenarioId, ticketHint },
  });
  const packet = toolCall.structuredContent;
  if (!packet) {
    throw new Error(`Missing structuredContent for scenario ${scenarioId}`);
  }
  return packet;
});

const writeTicketCommand = defineCommand<
  { ticketId: string; packet: TriagePacket; severity: string; testStatus: TestStatus },
  void
>("write_ticket_update", async ({ ticketId, packet, severity, testStatus }) => {
  await writeTicketUpdate(ticketId, packet, severity, testStatus);
});

const writePrDraftCommand = defineCommand<
  { ticketId: string; packet: TriagePacket; severity: string; action: string },
  void
>("write_pr_draft", async ({ ticketId, packet, severity, action }) => {
  await writePrDraft(ticketId, packet, severity, action);
});

const appendEventCommand = defineCommand<{ event: Record<string, unknown> }, void>(
  "append_event_log",
  async ({ event }) => await appendEventLog(event),
);

async function runCycle(cycle: number): Promise<void> {
  const cycleStart = Date.now();
  const testStatus = await invokeCommand(runTestsCommand, {}, { cycle });

  for (const scenarioId of scenarios) {
    enforceCycleBudget(cycleStart, cycle);

    const packet = await invokeCommand(
      generatePacketCommand,
      { scenarioId },
      { cycle, scenarioId },
    );
    const incident = classifyIncident(packet);
    const ticketId = packet.recommendedTicket;

    await invokeCommand(
      writeTicketCommand,
      { ticketId, packet, severity: incident.severity, testStatus },
      { cycle, scenarioId },
    );
    await invokeCommand(
      writePrDraftCommand,
      {
        ticketId,
        packet,
        severity: incident.severity,
        action: incident.action,
      },
      { cycle, scenarioId },
    );
    await invokeCommand(
      appendEventCommand,
      {
        event: {
          ts: new Date().toISOString(),
          cycle,
          scenarioId: packet.scenarioId,
          ticketId,
          severity: incident.severity,
          issues: packet.detectedIssues,
          characterization: testStatus,
          writeBudget: `${usedFileWrites}/${maxFileWrites}`,
        },
      },
      { cycle, scenarioId },
    );

    // Intentional operator-friendly logging for live demos.
    // eslint-disable-next-line no-console
    console.log(
      `[cycle ${cycle}] scenario=${packet.scenarioId} ticket=${ticketId} severity=${incident.severity}`,
    );
  }

  enforceCycleBudget(cycleStart, cycle);
}

async function main(): Promise<void> {
  enforceNetworkPolicy();
  await ensureDirs();

  // eslint-disable-next-line no-console
  console.log(
    `Autopilot starting: base=${baseUrl} scenarios=${scenarios.join(",")} cycles=${maxCycles}`,
  );
  // eslint-disable-next-line no-console
  console.log(
    `policy: allowedPrefixes=${allowedUrlPrefixes.join("|")} maxCommandMs=${maxCommandRuntimeMs} maxCycleMs=${maxCycleRuntimeMs} maxWrites=${maxFileWrites}`,
  );

  await invokeCommand(listToolsCommand, {}, { cycle: 0 });

  for (let cycle = 1; cycle <= maxCycles; cycle++) {
    await runCycle(cycle);
    if (cycle < maxCycles) {
      await sleep(intervalMs);
    }
  }

  // eslint-disable-next-line no-console
  console.log(`Autopilot done. Output written to ${outputDir}/`);
}

void main().catch((error) => {
  // eslint-disable-next-line no-console
  console.error(`Autopilot failed: ${(error as Error).message}`);
  process.exitCode = 1;
});

