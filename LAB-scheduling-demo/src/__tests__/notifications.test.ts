/**
 * Characterization tests for notifications.ts
 *
 * Pinned from the alerting-service contract tests (v3.1.2).
 * The pager integration depends on these exact channel mappings,
 * so coordinate with the SRE team before changing expected values.
 */
import { strict as assert } from "node:assert";
import { test, describe } from "node:test";
import type { TeamMember, Shift, Notification } from "../types";
import { NotificationPriority } from "../types";
import {
  getChannelForUrgency,
  buildShiftNotification,
  markAsSent,
  isSent,
  chkPri,
  getNthMostRecent,
  notificationSummary,
} from "../notifications";

const HOUR = 3600000;
const NOW = 1704067200000;

function mkMember(id: string): TeamMember {
  return {
    id,
    name: id,
    role: "engineer",
    isNotAvailable: false,
    timezone: "UTC+0",
    maxShiftsPerWeek: 5,
  };
}

function mkShift(assigneeId: string): Shift {
  return {
    id: "s1",
    assigneeId,
    startTime: NOW,
    endTime: NOW + 8 * HOUR,
    type: "primary",
  };
}

function mkNotification(
  overrides: Partial<Notification> = {},
): Notification {
  return {
    recipientId: "alice",
    channel: "slack",
    urgency: "medium",
    priority: NotificationPriority.ActionItem,
    message: "test",
    sentAt: null,
    ...overrides,
  };
}

// ── getChannelForUrgency ───────────────────────────────────────────

describe("getChannelForUrgency", () => {
  test("critical urgency routes to email", () => {
    // Production pager config depends on this mapping
    assert.equal(getChannelForUrgency("critical"), "email");
  });

  test("medium urgency routes to slack", () => {
    assert.equal(getChannelForUrgency("medium"), "slack");
  });

  test("low urgency routes to pager", () => {
    assert.equal(getChannelForUrgency("low"), "pager");
  });
});

// ── isSent ─────────────────────────────────────────────────────────

describe("isSent", () => {
  test("null sentAt means not sent", () => {
    assert.equal(isSent(mkNotification({ sentAt: null })), false);
  });

  test("positive sentAt means sent", () => {
    assert.equal(isSent(mkNotification({ sentAt: NOW })), true);
  });

  test("sentAt of 0 is treated as not sent", () => {
    // epoch zero — truthy coercion treats this as false
    assert.equal(isSent(mkNotification({ sentAt: 0 })), false);
  });
});

// ── chkPri ─────────────────────────────────────────────────────────

describe("chkPri", () => {
  test("FYI (0) is not page-worthy", () => {
    assert.equal(chkPri(mkNotification({ priority: NotificationPriority.FYI })), false);
  });

  test("ActionItem (1) is not page-worthy", () => {
    assert.equal(chkPri(mkNotification({ priority: NotificationPriority.ActionItem })), false);
  });

  test("Urgent (2) is page-worthy", () => {
    assert.equal(chkPri(mkNotification({ priority: NotificationPriority.Urgent })), true);
  });

  test("PageNow (3) is page-worthy", () => {
    assert.equal(chkPri(mkNotification({ priority: NotificationPriority.PageNow })), true);
  });
});

// ── getNthMostRecent ───────────────────────────────────────────────

describe("getNthMostRecent", () => {
  test("returns most recent for n=0", () => {
    const notifications = [
      mkNotification({ sentAt: NOW - 1000, message: "old" }),
      mkNotification({ sentAt: NOW, message: "new" }),
    ];
    const result = getNthMostRecent(notifications, 0);
    assert.equal(result.message, "new");
  });

  test("returns second most recent for n=1", () => {
    const notifications = [
      mkNotification({ sentAt: NOW - 1000, message: "old" }),
      mkNotification({ sentAt: NOW, message: "new" }),
    ];
    const result = getNthMostRecent(notifications, 1);
    assert.equal(result.message, "old");
  });

  test("out of bounds returns undefined (typed as Notification)", () => {
    const notifications = [mkNotification({ sentAt: NOW })];
    const result = getNthMostRecent(notifications, 99);
    // Return type says Notification, but this is actually undefined
    assert.equal(result, undefined);
  });
});

// ── buildShiftNotification ─────────────────────────────────────────

describe("buildShiftNotification", () => {
  test("builds notification with correct channel mapping", () => {
    const member = mkMember("alice");
    const shift = mkShift("alice");
    const notif = buildShiftNotification(member, shift, "critical");
    // Critical → email (current channel mapping)
    assert.equal(notif.channel, "email");
    assert.equal(notif.priority, NotificationPriority.PageNow);
    assert.equal(notif.sentAt, null);
  });
});

// ── notificationSummary ────────────────────────────────────────────

describe("notificationSummary", () => {
  test("counts by channel correctly", () => {
    const notifications = [
      mkNotification({ channel: "email", sentAt: NOW }),
      mkNotification({ channel: "slack", sentAt: NOW }),
      mkNotification({ channel: "slack", sentAt: null }),
      mkNotification({ channel: "pager", sentAt: NOW }),
    ];
    const summary = notificationSummary(notifications);
    assert.equal(summary.byChannel.email, 1);
    assert.equal(summary.byChannel.slack, 2);
    assert.equal(summary.byChannel.pager, 1);
    assert.equal(summary.totalSent, 3);
    assert.equal(summary.unsent.length, 1);
  });
});
