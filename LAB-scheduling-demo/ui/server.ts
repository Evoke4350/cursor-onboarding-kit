import {
  createServer,
  type IncomingMessage,
  type ServerResponse,
} from "node:http";
import { readFile } from "node:fs/promises";
import { fileURLToPath } from "node:url";
import { join } from "node:path";

import type {
  TeamMember,
  Shift,
  TimeOffRequest,
  EscalationContact,
  Notification,
} from "../src/types";
import { generateWeeklyReport } from "../src/scheduleReport";
import { routeWeeklyNotifications } from "../src/notifications";
import { findAllConflicts } from "../src/availability";

const HOUR = 60 * 60 * 1000;
const DAY = 24 * HOUR;
const MONDAY = 1704067200000; // 2024-01-01T00:00:00.000Z
const AGENT_DEV_KEY = process.env.AGENT_DEV_KEY ?? "lab-agent-key";
const PUBLIC_DIR_NAME = process.env.PUBLIC_DIR_NAME ?? "public";

const uiDir = fileURLToPath(new URL(".", import.meta.url));
const publicDir = join(uiDir, PUBLIC_DIR_NAME);

type Scenario = {
  id: string;
  title: string;
  description: string;
  weekStart: number;
  team: TeamMember[];
  shifts: Shift[];
  timeOffRequests: TimeOffRequest[];
  escalationContacts: EscalationContact[];
};

function mkMember(
  id: string,
  role: TeamMember["role"],
  isNotAvailable: boolean,
  timezone = "UTC+0",
): TeamMember {
  return {
    id,
    name: id,
    role,
    isNotAvailable,
    timezone,
    maxShiftsPerWeek: 5,
  };
}

function mkShift(
  id: string,
  assigneeId: string,
  startTime: number,
  endTime: number,
  type: Shift["type"] = "primary",
): Shift {
  return { id, assigneeId, startTime, endTime, type };
}

const scenarios: Record<string, Scenario> = {
  baseline: {
    id: "baseline",
    title: "Baseline Week",
    description: "A mostly normal set of shifts to show core outputs.",
    weekStart: MONDAY,
    team: [
      mkMember("alice", "engineer", false),
      mkMember("bob", "lead", false),
      mkMember("marta", "manager", false),
    ],
    shifts: [
      mkShift("s1", "alice", MONDAY, MONDAY + 12 * HOUR, "primary"),
      mkShift("s2", "bob", MONDAY + 12 * HOUR, MONDAY + DAY, "primary"),
      mkShift("s3", "alice", MONDAY + DAY, MONDAY + DAY + 8 * HOUR, "secondary"),
    ],
    timeOffRequests: [],
    escalationContacts: [{ memberId: "alice", escalateTo: "bob" }],
  },
  "coverage-inflation": {
    id: "coverage-inflation",
    title: "Coverage Inflation",
    description:
      "Primary + secondary overlap the same window, producing >100% coverage.",
    weekStart: MONDAY,
    team: [
      mkMember("alice", "engineer", false),
      mkMember("bob", "engineer", false),
      mkMember("marta", "manager", false),
    ],
    shifts: [
      mkShift("s1", "alice", MONDAY, MONDAY + 7 * DAY, "primary"),
      mkShift("s2", "bob", MONDAY, MONDAY + 7 * DAY, "secondary"),
    ],
    timeOffRequests: [],
    escalationContacts: [],
  },
  "escalation-hole": {
    id: "escalation-hole",
    title: "Escalation Hole",
    description:
      "Unavailable assignee on a critical shift without escalation contact.",
    weekStart: MONDAY,
    team: [
      mkMember("oncall-1", "engineer", true),
      mkMember("lead-1", "lead", false),
      mkMember("marta", "manager", false),
    ],
    shifts: [
      mkShift("s1", "oncall-1", MONDAY + 8 * HOUR, MONDAY + 16 * HOUR, "override"),
    ],
    timeOffRequests: [],
    escalationContacts: [],
  },
  "timezone-ghost-conflict": {
    id: "timezone-ghost-conflict",
    title: "Timezone Ghost Conflict",
    description:
      "Unapproved time-off plus overlap math creates a conflict that should not exist.",
    weekStart: MONDAY,
    team: [
      mkMember("apac-dev", "engineer", false, "UTC+5:30"),
      mkMember("marta", "manager", false),
    ],
    shifts: [
      mkShift("s1", "apac-dev", MONDAY + 10 * HOUR, MONDAY + 11 * HOUR, "primary"),
    ],
    timeOffRequests: [
      {
        memberId: "apac-dev",
        startDate: MONDAY + 2 * HOUR,
        endDate: MONDAY + 3 * HOUR,
        isNotApproved: true,
      },
    ],
    escalationContacts: [],
  },
};

type ScenarioPayload = {
  meta: {
    id: string;
    title: string;
    description: string;
    weekStart: number;
  };
  counts: {
    teamMembers: number;
    shifts: number;
    notifications: number;
    conflicts: number;
    teamGaps: number;
  };
  signals: {
    coverageOver100: boolean;
    criticalRoutedToEmail: number;
    noEscalationFallbackObserved: boolean;
    unapprovedConflictCount: number;
  };
  report: ReturnType<typeof generateWeeklyReport>;
  notifications: Notification[];
  conflicts: ReturnType<typeof findAllConflicts>;
};

type AgentTriagePacket = {
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

type JsonRpcRequest = {
  jsonrpc?: string;
  id?: string | number | null;
  method?: string;
  params?: Record<string, unknown>;
};

function getUrgency(shift: Shift): Notification["urgency"] {
  if (shift.type === "override") return "critical";
  if (shift.type === "primary") return "medium";
  return "low";
}

function buildScenarioPayload(id: string): ScenarioPayload {
  const scenario = scenarios[id] ?? scenarios.baseline;
  const report = generateWeeklyReport(
    scenario.team,
    scenario.shifts,
    scenario.weekStart,
  );
  const notifications = routeWeeklyNotifications(
    scenario.team,
    scenario.shifts,
    scenario.escalationContacts,
  );
  const conflicts = findAllConflicts(
    scenario.shifts,
    scenario.team,
    scenario.timeOffRequests,
  );

  const criticalRoutedToEmail = notifications.filter(
    (n) => n.urgency === "critical" && n.channel === "email",
  ).length;

  const criticalUnavailableShiftCount = scenario.shifts.filter((shift) => {
    const member = scenario.team.find((m) => m.id === shift.assigneeId);
    return Boolean(member?.isNotAvailable) && getUrgency(shift) === "critical";
  }).length;
  const criticalNotificationCount = notifications.filter(
    (n) => n.urgency === "critical",
  ).length;

  const noEscalationFallbackObserved =
    criticalUnavailableShiftCount > 0 &&
    criticalNotificationCount === criticalUnavailableShiftCount;

  const unapprovedConflictCount = conflicts.filter(
    (c) => c.conflictingRequest.isNotApproved,
  ).length;

  return {
    meta: {
      id: scenario.id,
      title: scenario.title,
      description: scenario.description,
      weekStart: scenario.weekStart,
    },
    counts: {
      teamMembers: scenario.team.length,
      shifts: scenario.shifts.length,
      notifications: notifications.length,
      conflicts: conflicts.length,
      teamGaps: report.teamGaps.length,
    },
    signals: {
      coverageOver100: report.teamCoverage > 100,
      criticalRoutedToEmail,
      noEscalationFallbackObserved,
      unapprovedConflictCount,
    },
    report,
    notifications,
    conflicts,
  };
}

function listDetectedIssues(payload: ScenarioPayload): string[] {
  const issues: string[] = [];

  if (payload.signals.coverageOver100) {
    issues.push(
      `Coverage inflation detected: teamCoverage=${payload.report.teamCoverage.toFixed(2)}%`,
    );
  }
  if (payload.signals.criticalRoutedToEmail > 0) {
    issues.push(
      `Critical alerts routed to email: ${payload.signals.criticalRoutedToEmail}`,
    );
  }
  if (payload.signals.noEscalationFallbackObserved) {
    issues.push("No escalation fallback observed for unavailable critical assignee");
  }
  if (payload.signals.unapprovedConflictCount > 0) {
    issues.push(
      `Conflicts from unapproved time-off requests: ${payload.signals.unapprovedConflictCount}`,
    );
  }

  if (issues.length === 0) {
    issues.push("No high-signal bug indicators for this scenario");
  }

  return issues;
}

function recommendTicket(payload: ScenarioPayload): string {
  if (
    payload.signals.criticalRoutedToEmail > 0 ||
    payload.signals.noEscalationFallbackObserved ||
    payload.signals.unapprovedConflictCount > 0
  ) {
    return "SCHED-2427";
  }
  if (payload.signals.coverageOver100) return "SCHED-2419";
  return "SCHED-2461";
}

function buildAgentTriagePacket(
  scenarioId: string,
  ticketHint: string | undefined,
): AgentTriagePacket {
  const payload = buildScenarioPayload(scenarioId);
  const fallbackTicket = recommendTicket(payload);
  const recommendedTicket = ticketHint?.trim() || fallbackTicket;

  return {
    objective: "triage_packet",
    scenarioId: payload.meta.id,
    scenarioTitle: payload.meta.title,
    generatedAt: new Date().toISOString(),
    summary: {
      teamCoverage: payload.report.teamCoverage,
      fairnessScore: payload.report.fairnessScore,
      teamGapCount: payload.counts.teamGaps,
      notificationCount: payload.counts.notifications,
      conflictCount: payload.counts.conflicts,
    },
    detectedIssues: listDetectedIssues(payload),
    recommendedTicket,
    notes:
      "Packet generated from current buggy baseline. Use as triage input, not ground truth.",
    nextCommands: [
      "tsx --test src/__tests__/*.ts",
      "tsx ui/server.ts",
      `# open ticket: ${recommendedTicket}`,
    ],
  };
}

function sendJson(
  res: ServerResponse<IncomingMessage>,
  status: number,
  payload: unknown,
): void {
  res.writeHead(status, { "content-type": "application/json; charset=utf-8" });
  res.end(JSON.stringify(payload, null, 2));
}

function sendJsonRpcResult(
  res: ServerResponse<IncomingMessage>,
  id: JsonRpcRequest["id"],
  result: unknown,
): void {
  sendJson(res, 200, {
    jsonrpc: "2.0",
    id: id ?? null,
    result,
  });
}

function sendJsonRpcError(
  res: ServerResponse<IncomingMessage>,
  id: JsonRpcRequest["id"],
  code: number,
  message: string,
): void {
  sendJson(res, 200, {
    jsonrpc: "2.0",
    id: id ?? null,
    error: { code, message },
  });
}

async function readRequestBody(
  req: IncomingMessage,
): Promise<string> {
  return await new Promise<string>((resolve, reject) => {
    let data = "";
    req.setEncoding("utf8");
    req.on("data", (chunk) => {
      data += chunk;
      if (data.length > 1024 * 1024) {
        reject(new Error("Request body too large"));
      }
    });
    req.on("end", () => resolve(data));
    req.on("error", reject);
  });
}

function isAgentAuthorized(
  req: IncomingMessage,
): boolean {
  const headerValue = req.headers["x-agent-key"];
  if (Array.isArray(headerValue)) {
    return headerValue.includes(AGENT_DEV_KEY);
  }
  return headerValue === AGENT_DEV_KEY;
}

const staticRoutes: Record<string, { file: string; contentType: string }> = {
  "/": { file: "index.html", contentType: "text/html; charset=utf-8" },
  "/app.js": { file: "app.js", contentType: "text/javascript; charset=utf-8" },
  "/styles.css": { file: "styles.css", contentType: "text/css; charset=utf-8" },
  "/agent/dev-menu": {
    file: "agent-dev-menu.html",
    contentType: "text/html; charset=utf-8",
  },
  "/agent/dev-menu.js": {
    file: "agent-dev-menu.js",
    contentType: "text/javascript; charset=utf-8",
  },
};

const server = createServer(async (req, res) => {
  try {
    const url = new URL(req.url ?? "/", "http://localhost");

    if (url.pathname === "/mcp") {
      if (!isAgentAuthorized(req)) {
        sendJsonRpcError(res, null, -32001, "Unauthorized agent key");
        return;
      }
      if (req.method !== "POST") {
        sendJsonRpcError(res, null, -32600, "Use POST for JSON-RPC");
        return;
      }

      const rawBody = await readRequestBody(req);
      let rpc: JsonRpcRequest;
      try {
        rpc = JSON.parse(rawBody || "{}") as JsonRpcRequest;
      } catch {
        sendJsonRpcError(res, null, -32700, "Parse error");
        return;
      }

      if (rpc.jsonrpc !== "2.0" || typeof rpc.method !== "string") {
        sendJsonRpcError(res, rpc.id, -32600, "Invalid Request");
        return;
      }

      if (rpc.method === "tools/list") {
        sendJsonRpcResult(res, rpc.id, {
          tools: [
            {
              name: "generate_triage_packet",
              description:
                "Generate a compressed scenario triage packet from the scheduling demo baseline.",
              inputSchema: {
                type: "object",
                properties: {
                  scenarioId: {
                    type: "string",
                    enum: Object.keys(scenarios),
                  },
                  ticketHint: { type: "string" },
                },
                required: ["scenarioId"],
              },
            },
          ],
        });
        return;
      }

      if (rpc.method === "tools/call") {
        const params = rpc.params ?? {};
        const name = params.name;
        const args = params.arguments as Record<string, unknown> | undefined;

        if (name !== "generate_triage_packet") {
          sendJsonRpcError(res, rpc.id, -32602, "Unknown tool");
          return;
        }

        const scenarioId =
          typeof args?.scenarioId === "string" ? args.scenarioId : "baseline";
        const ticketHint =
          typeof args?.ticketHint === "string" ? args.ticketHint : undefined;
        const packet = buildAgentTriagePacket(scenarioId, ticketHint);

        sendJsonRpcResult(res, rpc.id, {
          content: [
            {
              type: "text",
              text: JSON.stringify(packet, null, 2),
            },
          ],
          structuredContent: packet,
        });
        return;
      }

      sendJsonRpcError(res, rpc.id, -32601, `Method not found: ${rpc.method}`);
      return;
    }

    if (url.pathname === "/api/scenarios") {
      const list = Object.values(scenarios).map((s) => ({
        id: s.id,
        title: s.title,
        description: s.description,
      }));
      sendJson(res, 200, { scenarios: list });
      return;
    }

    if (url.pathname === "/api/scenario") {
      const id = url.searchParams.get("id") ?? "baseline";
      const payload = buildScenarioPayload(id);
      sendJson(res, 200, payload);
      return;
    }

    if (url.pathname === "/agent/dev-menu" && url.searchParams.get("agent") !== "1") {
      res.writeHead(403, { "content-type": "text/plain; charset=utf-8" });
      res.end("Agent menu is disabled. Use /agent/dev-menu?agent=1");
      return;
    }

    const route = staticRoutes[url.pathname];
    if (!route) {
      res.writeHead(404, { "content-type": "text/plain; charset=utf-8" });
      res.end("Not found");
      return;
    }

    const filePath = join(publicDir, route.file);
    const body = await readFile(filePath);
    res.writeHead(200, { "content-type": route.contentType });
    res.end(body);
  } catch (error) {
    // Keep runtime diagnostics visible in `docker logs` for workshop debugging.
    // eslint-disable-next-line no-console
    console.error("Scheduling demo server error:", error);
    res.writeHead(500, { "content-type": "text/plain; charset=utf-8" });
    res.end(`Server error: ${(error as Error).message}`);
  }
});

const port = Number(process.env.PORT ?? 4173);
server.listen(port, () => {
  // Keep startup output explicit for workshop copy/paste.
  // eslint-disable-next-line no-console
  console.log(`Scheduling demo UI running at http://localhost:${port}`);
  // eslint-disable-next-line no-console
  console.log(`Static files directory: ${publicDir}`);
});
