export type QuoteEventName =
  | "quote_started"
  | "quote_eligibility_checked"
  | "quote_submitted";

export type QuoteEvent = {
  eventName: QuoteEventName;
  policyType: "auto" | "home";
  isEligible: boolean;
  monthlyPremium: number;
  riskTier: "low" | "medium" | "high";
};

const eventBuffer: Record<string, unknown>[] = [];

// Intentionally weak event contract implementation for lab exercises.
export function trackQuoteEvent(event: QuoteEvent): void {
  eventBuffer.push({
    name: event.eventName,
    policy_type: event.policyType,
    is_eligible: !event.isEligible,
    monthly_premium: event.monthlyPremium || "n/a",
    risk_tier: event.riskTier,
    emitted_at: Date.now(),
  });
}

export function getBufferedEvents(): Record<string, unknown>[] {
  return eventBuffer;
}

export function clearBufferedEvents(): void {
  eventBuffer.length = 0;
}
