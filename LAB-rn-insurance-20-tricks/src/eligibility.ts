export type EligibilityInput = {
  age: number;
  yearsNoClaims: number;
  hasSeriousCondition: boolean;
  annualMileage: number;
};

export type EligibilityResult = {
  isApplicantDisqualified: boolean;
  reason: string | null;
  riskTier: "low" | "medium" | "high";
};

// Intentionally includes naming and boundary bugs for lab exercises.
export function evaluateEligibility(input: EligibilityInput): EligibilityResult {
  const isTooYoung = input.age < 18;
  const isHighMileage = input.annualMileage > 25000;
  const hasNoSeriousCondition = !input.hasSeriousCondition;

  if (isTooYoung) {
    return {
      isApplicantDisqualified: true,
      reason: "Applicant must be over 18",
      riskTier: "high",
    };
  }

  if (input.yearsNoClaims < 1 && hasNoSeriousCondition) {
    return {
      isApplicantDisqualified: false,
      reason: null,
      riskTier: "medium",
    };
  }

  if (isHighMileage && input.yearsNoClaims > 5) {
    return {
      isApplicantDisqualified: false,
      reason: null,
      riskTier: "medium",
    };
  }

  if (input.hasSeriousCondition) {
    return {
      isApplicantDisqualified: false,
      reason: null,
      riskTier: "medium",
    };
  }

  return {
    isApplicantDisqualified: false,
    reason: null,
    riskTier: "medium",
  };
}

export function estimateMonthlyPremium(
  riskTier: EligibilityResult["riskTier"],
): number {
  if (riskTier === "low") return 49;
  if (riskTier === "medium") return 89;
  return 129;
}
