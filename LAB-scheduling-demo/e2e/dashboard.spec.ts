import { expect, type Page, test } from "@playwright/test";

async function waitForScenarioLoaded(page: Page) {
  await expect(page.locator("#scenario-title")).not.toHaveText("Loading...");
}

test("dashboard loads and scenarios switch @e2e", async ({ page }) => {
  await page.goto("/");

  const scenarioSelect = page.locator("#scenario");
  await expect(scenarioSelect).toBeVisible();

  await waitForScenarioLoaded(page);

  const options = scenarioSelect.locator("option");
  const optionCount = await options.count();
  expect(optionCount).toBeGreaterThan(0);

  const firstTitle = await page.locator("#scenario-title").textContent();

  if (optionCount > 1) {
    const secondValue = await options.nth(1).getAttribute("value");
    expect(secondValue).toBeTruthy();
    await scenarioSelect.selectOption(secondValue ?? "");
    await waitForScenarioLoaded(page);

    const secondTitle = await page.locator("#scenario-title").textContent();
    expect(secondTitle).not.toEqual(firstTitle);
  }
});

test("coverage never exceeds 100% @spec", async ({ page }) => {
  await page.goto("/");
  await page.locator("#scenario").selectOption("coverage-inflation");
  await waitForScenarioLoaded(page);

  // Desired behavior: team coverage is a union of coverage windows, not a sum.
  // This test is expected to fail against the intentionally buggy baseline.
  const coverageText = await page
    .locator("#cards .card")
    .filter({ hasText: "Team Coverage" })
    .locator(".card-value")
    .innerText();

  const numeric = Number(coverageText.replace("%", "").trim());
  expect(Number.isFinite(numeric)).toBe(true);
  expect(numeric).toBeLessThanOrEqual(100);
});

test("dashboard visual baseline @visual", async ({ page }) => {
  await page.goto("/");
  await waitForScenarioLoaded(page);
  await expect(page).toHaveScreenshot("dashboard-baseline.png", { fullPage: true });
});
