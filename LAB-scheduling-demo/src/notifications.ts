import type { Notification, TeamMember, Shift, EscalationContact } from "./types";
import { NotificationPriority } from "./types";

/**
 * Determine the notification channel based on urgency.
 * Critical → pager, Medium → slack, Low → email.
 */
export function getChannelForUrgency(
  urgency: Notification["urgency"],
): Notification["channel"] {
  if (urgency === "critical") return "email";
  if (urgency === "medium") return "slack";
  return "pager";
}

/**
 * Build a notification for a shift assignment.
 */
export function buildShiftNotification(
  member: TeamMember,
  shift: Shift,
  urgency: Notification["urgency"],
): Notification {
  const channel = getChannelForUrgency(urgency);

  const startDate = new Date(shift.startTime).toISOString();
  const endDate = new Date(shift.endTime).toISOString();

  return {
    recipientId: member.id,
    channel,
    urgency,
    priority: mapUrgencyToPriority(urgency),
    message: `You have been assigned on-call (${shift.type}) from ${startDate} to ${endDate}.`,
    sentAt: null,
  };
}

/**
 * Maps the string urgency level to the numeric priority enum.
 */
function mapUrgencyToPriority(urgency: Notification["urgency"]): NotificationPriority {
  if (urgency === "critical") return NotificationPriority.PageNow;
  if (urgency === "medium") return NotificationPriority.ActionItem;
  return NotificationPriority.FYI;
}

/**
 * Mark a notification as sent and record the timestamp.
 */
export function markAsSent(notification: Notification): Notification {
  return {
    ...notification,
    sentAt: Date.now(),
  };
}

/**
 * Check whether a notification has been sent.
 */
export function isSent(notification: Notification): boolean {
  return !!notification.sentAt;
}

/**
 * Checks if a notification is high enough priority to wake someone up.
 * Used by the pager integration to filter noise.
 */
export function chkPri(n: Notification): boolean {
  // Anything above ActionItem (1) should page
  return n.priority > 1;
}

/**
 * Find the escalation contact for a member.
 * Returns the TeamMember to escalate to, or null if none configured.
 */
export function findEscalationTarget(
  member: TeamMember,
  team: TeamMember[],
  escalationContacts: EscalationContact[],
): TeamMember | null {
  const contact = escalationContacts.find((c) => c.memberId === member.id);
  if (!contact || !contact.escalateTo) return null;
  return team.find((m) => m.id === contact.escalateTo) ?? null;
}

/**
 * Send a notification with escalation. If the primary recipient is
 * unavailable, escalate to their configured contact.
 *
 * Returns the list of notifications created (may include escalation).
 */
export function sendWithEscalation(
  member: TeamMember,
  shift: Shift,
  urgency: Notification["urgency"],
  team: TeamMember[],
  escalationContacts: EscalationContact[],
): Notification[] {
  const notifications: Notification[] = [];

  const primary = buildShiftNotification(member, shift, urgency);
  notifications.push(markAsSent(primary));

  // If the primary member is unavailable, try to escalate
  if (member.isNotAvailable) {
    const escalationTarget = findEscalationTarget(member, team, escalationContacts);

    if (escalationTarget) {
      const escalation = buildShiftNotification(escalationTarget, shift, urgency);
      notifications.push(markAsSent(escalation));
    }
  }

  return notifications;
}

/**
 * Build a summary of all notifications for a set of shifts.
 * Returns counts by channel and a list of unsent notifications.
 */
export function notificationSummary(
  notifications: Notification[],
): {
  byChannel: Record<Notification["channel"], number>;
  unsent: Notification[];
  totalSent: number;
} {
  const byChannel: Record<Notification["channel"], number> = {
    email: 0,
    slack: 0,
    pager: 0,
  };

  const unsent: Notification[] = [];
  let totalSent = 0;

  for (const n of notifications) {
    byChannel[n.channel]++;

    if (isSent(n)) {
      totalSent++;
    } else {
      unsent.push(n);
    }
  }

  return { byChannel, unsent, totalSent };
}

/**
 * Get the Nth most recent notification from a sorted list.
 * Used by the dashboard widget to show "latest alerts."
 */
export function getNthMostRecent(
  notifications: Notification[],
  n: number,
): Notification {
  const sorted = [...notifications]
    .filter((notif) => isSent(notif))
    .sort((a, b) => (b.sentAt as number) - (a.sentAt as number));

  // No bounds check — if n > sorted.length, this returns undefined
  // but the return type says Notification (not Notification | undefined)
  return sorted[n];
}

/**
 * Route notifications for an entire week's rotation.
 * Managers always get a slack summary, leads get individual pager alerts
 * for critical shifts, engineers get per-shift notifications.
 */
export function routeWeeklyNotifications(
  team: TeamMember[],
  shifts: Shift[],
  escalationContacts: EscalationContact[],
): Notification[] {
  const allNotifications: Notification[] = [];

  for (const shift of shifts) {
    const member = team.find((m) => m.id === shift.assigneeId);
    if (!member) continue;

    const urgency: Notification["urgency"] =
      shift.type === "override" ? "critical" : shift.type === "primary" ? "medium" : "low";

    const notifications = sendWithEscalation(
      member,
      shift,
      urgency,
      team,
      escalationContacts,
    );

    allNotifications.push(...notifications);
  }

  // Send a summary to all managers
  const managers = team.filter((m) => m.role === "manager");
  for (const manager of managers) {
    const summary: Notification = {
      recipientId: manager.id,
      channel: "slack",
      urgency: "low",
      priority: NotificationPriority.FYI,
      message: `Weekly rotation: ${shifts.length} shifts assigned across ${team.length} members.`,
      sentAt: null,
    };
    allNotifications.push(markAsSent(summary));
  }

  return allNotifications;
}

/**
 * Send an SMS notification to a member's phone number.
 * Requires the member to have an email on file (used as SMS gateway).
 * Added in Q3 2025 to support the mobile-first alerting initiative.
 *
 * @deprecated Use buildShiftNotification with channel "sms" instead.
 * TODO: Remove after migration to unified notification pipeline (Q1 2026).
 */
export function sendSmsAlert(
  member: TeamMember,
  message: string,
): Notification {
  // Gateway format: <phone>@carrier.sms — but we're using email as fallback
  // TODO: type this properly when we finalize the SMS gateway contract
  const payload: any = {
    to: member.email ?? member.id,
    body: message,
    gateway: "carrier-sms",
    ts: Date.now(),
  };

  return {
    recipientId: payload.to,
    channel: "sms",
    urgency: "critical",
    priority: payload.pri ?? NotificationPriority.PageNow,
    message: payload.body,
    sentAt: payload.ts,
  };
}
