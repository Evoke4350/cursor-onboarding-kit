const scenarioSelect = document.getElementById("scenario");
const scenarioTitle = document.getElementById("scenario-title");
const scenarioDescription = document.getElementById("scenario-description");
const cards = document.getElementById("cards");
const signals = document.getElementById("signals");
const reportCards = document.getElementById("report-cards");
const notificationsCards = document.getElementById("notifications-cards");
const conflictsCards = document.getElementById("conflicts-cards");

function formatPercent(value) {
  return `${Number(value).toFixed(1)}%`;
}

function formatDate(ms) {
  if (typeof ms !== "number") return "not sent";
  return new Date(ms).toISOString();
}

function renderCards(data) {
  const items = [
    { label: "Team Coverage", value: formatPercent(data.report.teamCoverage) },
    { label: "Fairness Score", value: data.report.fairnessScore.toFixed(1) },
    { label: "Team Gaps", value: data.counts.teamGaps },
    { label: "Notifications", value: data.counts.notifications },
    { label: "Conflicts", value: data.counts.conflicts },
    { label: "Over-Cap Members", value: data.report.overCapMembers.length },
  ];

  cards.innerHTML = items
    .map(
      (item) =>
        `<div class="card"><p class="card-label">${item.label}</p><p class="card-value">${item.value}</p></div>`,
    )
    .join("");
}

function renderSignals(data) {
  const signalRows = [
    {
      label: "Coverage over 100%",
      value: data.signals.coverageOver100 ? "YES" : "NO",
      danger: data.signals.coverageOver100,
    },
    {
      label: "Critical alerts routed to email",
      value: String(data.signals.criticalRoutedToEmail),
      danger: data.signals.criticalRoutedToEmail > 0,
    },
    {
      label: "No fallback observed for critical unavailable assignee",
      value: data.signals.noEscalationFallbackObserved ? "YES" : "NO",
      danger: data.signals.noEscalationFallbackObserved,
    },
    {
      label: "Conflicts from unapproved requests",
      value: String(data.signals.unapprovedConflictCount),
      danger: data.signals.unapprovedConflictCount > 0,
    },
  ];

  signals.innerHTML = signalRows
    .map(
      (row) =>
        `<li class="${row.danger ? "danger" : "ok"}"><span>${row.label}</span><strong>${row.value}</strong></li>`,
    )
    .join("");
}

function renderReportCards(data) {
  const summaryCards = [
    { label: "Team Coverage", value: formatPercent(data.report.teamCoverage) },
    { label: "Fairness Score", value: data.report.fairnessScore.toFixed(1) },
    { label: "Team Gaps", value: data.report.teamGaps.length },
    {
      label: "Over-Cap Members",
      value: data.report.overCapMembers.length
        ? data.report.overCapMembers.join(", ")
        : "none",
    },
  ];

  const memberCards = data.report.memberSummaries.map(
    (member) => `
      <div class="card">
        <p class="card-label">Member ${member.memberId}</p>
        <p class="card-value stack-value">${member.shiftCount} shifts</p>
        <p class="card-meta">${member.totalHours.toFixed(1)} hours</p>
        <p class="card-meta">coverage ${formatPercent(member.coveragePercent)}</p>
        <p class="card-meta">gaps ${member.gaps.length}</p>
      </div>
    `,
  );

  reportCards.innerHTML = [...summaryCards.map(
    (item) => `
      <div class="card">
        <p class="card-label">${item.label}</p>
        <p class="card-value stack-value">${item.value}</p>
      </div>
    `,
  ), ...memberCards].join("");
}

function renderNotificationsCards(data) {
  if (data.notifications.length === 0) {
    notificationsCards.innerHTML = `
      <div class="card">
        <p class="card-label">Notifications</p>
        <p class="card-value stack-value">none</p>
      </div>
    `;
    return;
  }

  notificationsCards.innerHTML = data.notifications
    .map(
      (notification) => `
        <div class="card">
          <p class="card-label">${notification.urgency} via ${notification.channel}</p>
          <p class="card-value stack-value">${notification.recipientId}</p>
          <p class="card-meta">priority ${notification.priority}</p>
          <p class="card-meta">${formatDate(notification.sentAt)}</p>
        </div>
      `,
    )
    .join("");
}

function renderConflictsCards(data) {
  if (data.conflicts.length === 0) {
    conflictsCards.innerHTML = `
      <div class="card">
        <p class="card-label">Conflicts</p>
        <p class="card-value stack-value">none</p>
      </div>
    `;
    return;
  }

  conflictsCards.innerHTML = data.conflicts
    .map(
      (conflict) => `
        <div class="card">
          <p class="card-label">member ${conflict.member.id}</p>
          <p class="card-value stack-value">${conflict.shift.type} shift</p>
          <p class="card-meta">request approved flag: ${String(!conflict.conflictingRequest.isNotApproved)}</p>
          <p class="card-meta">shift ${new Date(conflict.shift.startTime).toISOString()} - ${new Date(conflict.shift.endTime).toISOString()}</p>
        </div>
      `,
    )
    .join("");
}

async function loadScenario(id) {
  const res = await fetch(`/api/scenario?id=${encodeURIComponent(id)}`);
  const data = await res.json();

  scenarioTitle.textContent = data.meta.title;
  scenarioDescription.textContent = data.meta.description;

  renderCards(data);
  renderSignals(data);
  renderReportCards(data);
  renderNotificationsCards(data);
  renderConflictsCards(data);
}

async function boot() {
  const res = await fetch("/api/scenarios");
  const data = await res.json();

  scenarioSelect.innerHTML = data.scenarios
    .map((s) => `<option value="${s.id}">${s.title}</option>`)
    .join("");

  scenarioSelect.addEventListener("change", () => {
    void loadScenario(scenarioSelect.value);
  });

  if (data.scenarios.length > 0) {
    const requested = new URLSearchParams(window.location.search).get("scenario");
    const initialScenario =
      requested && data.scenarios.some((s) => s.id === requested)
        ? requested
        : data.scenarios[0].id;

    scenarioSelect.value = initialScenario;
    await loadScenario(initialScenario);
  }
}

void boot();
