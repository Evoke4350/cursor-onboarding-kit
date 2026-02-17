---
applyTo: "**/telemetry.ts,**/*telemetry*.ts"
---

# Telemetry Instructions (Template)

- Keep event field names stable unless explicitly requested.
- Do not invert semantic flags accidentally (`is_eligible` should match actual eligibility).
- Keep numeric payload fields numeric.
- When changing telemetry payloads, summarize contract impact in the response.
