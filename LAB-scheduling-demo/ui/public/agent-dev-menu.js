const form = document.getElementById("agent-form");
const scenarioSelect = document.getElementById("scenario-id");
const ticketHintInput = document.getElementById("ticket-hint");
const agentKeyInput = document.getElementById("agent-key");
const output = document.getElementById("output");

function rpcEnvelope(id, method, params) {
  return {
    jsonrpc: "2.0",
    id,
    method,
    params,
  };
}

async function fetchScenarios() {
  const res = await fetch("/api/scenarios");
  if (!res.ok) throw new Error(`Failed to load scenarios: ${res.status}`);
  const data = await res.json();
  return data.scenarios || [];
}

async function callMcp(request, agentKey) {
  const res = await fetch("/mcp", {
    method: "POST",
    headers: {
      "content-type": "application/json",
      "x-agent-key": agentKey,
    },
    body: JSON.stringify(request),
  });
  return await res.json();
}

async function boot() {
  const scenarios = await fetchScenarios();
  scenarioSelect.innerHTML = scenarios
    .map((s) => `<option value="${s.id}">${s.title}</option>`)
    .join("");
}

form.addEventListener("submit", async (event) => {
  event.preventDefault();
  output.textContent = "Running tool call...";

  const scenarioId = scenarioSelect.value;
  const ticketHint = ticketHintInput.value.trim();
  const agentKey = agentKeyInput.value.trim();

  const request = rpcEnvelope(Date.now(), "tools/call", {
    name: "generate_triage_packet",
    arguments: {
      scenarioId,
      ticketHint: ticketHint || undefined,
    },
  });

  try {
    const response = await callMcp(request, agentKey);
    output.textContent = JSON.stringify(response, null, 2);
  } catch (error) {
    output.textContent = JSON.stringify(
      { error: String(error) },
      null,
      2,
    );
  }
});

void boot();

