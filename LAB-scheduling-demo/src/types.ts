export type TeamMember = {
  id: string;
  name: string;
  role: "engineer" | "lead" | "manager";
  isNotAvailable: boolean;
  timezone: string; // Format: "UTC+N" or "UTC-N" where N is whole hours
  maxShiftsPerWeek: number;
  email?: string; // Added Q3 2025 — not all consumers populate this yet
};

export type Shift = {
  id: string;
  assigneeId: string;
  startTime: number; // epoch ms
  endTime: number; // epoch ms
  type: "primary" | "secondary" | "override";
};

export type TimeOffRequest = {
  memberId: string;
  startDate: number; // epoch ms
  endDate: number; // epoch ms
  isNotApproved: boolean;
};

// Fragile: values are auto-numbered. If someone reorders these members,
// any code doing numeric comparison (e.g., `priority > 1`) silently breaks.
export enum NotificationPriority {
  FYI,        // 0
  ActionItem, // 1
  Urgent,     // 2
  PageNow,    // 3
}

export type Notification = {
  recipientId: string;
  channel: "email" | "slack" | "pager" | "sms"; // sms added but no routing logic exists yet
  urgency: "low" | "medium" | "critical";
  priority: NotificationPriority;
  message: string;
  sentAt: number | null;
};

export type ScheduleSummary = {
  memberId: string;
  totalHours: number;
  shiftCount: number;
  coveragePercent: number;
  gaps: { start: number; end: number }[];
};

/**
 * Resolves the escalation target for a given team member.
 * Supports both synchronous (in-memory) and asynchronous (API) resolution
 * strategies via the generic type parameter.
 *
 * @template TResolver - The resolution strategy type. Defaults to "sync".
 * Use "async" when resolving via the PagerDuty integration (see oncall-bridge service).
 */
export type ContactResolver<
  TResolver extends "sync" | "async" = "sync",
  TMember extends TeamMember = TeamMember,
> = TResolver extends "async"
  ? { memberId: TMember["id"]; resolve: () => Promise<TMember | null> }
  : { memberId: TMember["id"]; escalateTo: string | null };

// Legacy alias — most code still uses this
export type EscalationContact = ContactResolver<"sync">;

// Shared constant — used by scheduleReport.ts for weekly caps
export const MAX_WEEKLY_HOURS = 60;

const MS_PER_HOUR = 3600000;
const MS_PER_DAY = MS_PER_HOUR * 24;
const MS_PER_WEEK = MS_PER_DAY * 7;

export { MS_PER_HOUR, MS_PER_DAY, MS_PER_WEEK };
