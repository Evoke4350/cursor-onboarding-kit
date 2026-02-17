-- NOTE: pseudo-query notes from analytics channel, not production SQL.
-- Goal: spot weeks where raw coverage looks healthy but incidents still happen.

SELECT
  week_start,
  team_id,
  coverage_raw_percent,
  coverage_effective_percent,
  incident_count_p1,
  incident_count_p2
FROM scheduling_weekly_observability
WHERE coverage_raw_percent > 100
  AND incident_count_p1 > 0
ORDER BY week_start DESC;

-- Analyst note:
-- If this returns rows, leadership dashboard is likely over-reporting safety.

