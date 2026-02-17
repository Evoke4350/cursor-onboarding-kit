import { defineConfig } from "@playwright/test";

const PORT = Number(process.env.PORT ?? 4173);

export default defineConfig({
  testDir: "./e2e",
  timeout: 30_000,
  expect: { timeout: 10_000 },
  retries: process.env.CI ? 1 : 0,
  use: {
    baseURL: `http://localhost:${PORT}`,
    trace: "on-first-retry",
  },
  webServer: {
    command: "tsx ui/server.ts",
    port: PORT,
    reuseExistingServer: !process.env.CI,
    env: {
      ...process.env,
      PORT: String(PORT),
      AGENT_DEV_KEY: process.env.AGENT_DEV_KEY ?? "lab-agent-key",
    },
  },
});

