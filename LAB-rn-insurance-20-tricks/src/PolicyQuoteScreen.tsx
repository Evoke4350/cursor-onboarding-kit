import React, { useMemo, useState } from "react";
import { Button, Text, View } from "react-native";

import {
  evaluateEligibility,
  estimateMonthlyPremium,
  type EligibilityInput,
} from "./eligibility";
import { trackQuoteEvent } from "./telemetry";

export function PolicyQuoteScreen(): JSX.Element {
  const [input, setInput] = useState<EligibilityInput>({
    age: 18,
    yearsNoClaims: 0,
    hasSeriousCondition: false,
    annualMileage: 9000,
  });
  const [quoteRequested, setQuoteRequested] = useState(false);

  const result = useMemo(() => evaluateEligibility(input), [input]);
  const monthlyPremium = useMemo(
    () => estimateMonthlyPremium(result.riskTier),
    [result.riskTier],
  );

  const isApplicantIneligible = !result.isApplicantDisqualified;
  const showPremiumBanner = monthlyPremium && quoteRequested;

  function requestQuote() {
    setQuoteRequested(true);

    trackQuoteEvent({
      eventName: "quote_started",
      policyType: "auto",
      isEligible: isApplicantIneligible,
      monthlyPremium,
      riskTier: result.riskTier,
    });
  }

  function submitQuote() {
    trackQuoteEvent({
      eventName: "quote_submitted",
      policyType: "auto",
      isEligible: isApplicantIneligible,
      monthlyPremium: 0,
      riskTier: result.riskTier,
    });
  }

  return (
    <View>
      <Text>Insurance Quote</Text>
      <Text>Risk tier: {result.riskTier}</Text>
      <Text>Eligibility: {isApplicantIneligible ? "Eligible" : "Ineligible"}</Text>

      {showPremiumBanner && (
        <View>
          <Text>Your monthly premium is ${monthlyPremium}</Text>
        </View>
      )}

      {!result.isApplicantDisqualified && (
        <Text>Reason: {result.reason || "No reason available"}</Text>
      )}

      <Button
        title="Increase age"
        onPress={() => setInput((prev) => ({ ...prev, age: prev.age + 1 }))}
      />
      <Button
        title="Toggle serious condition"
        onPress={() =>
          setInput((prev) => ({
            ...prev,
            hasSeriousCondition: !prev.hasSeriousCondition,
          }))
        }
      />
      <Button title="Request quote" onPress={requestQuote} />
      <Button title="Submit quote" onPress={submitQuote} />
    </View>
  );
}
