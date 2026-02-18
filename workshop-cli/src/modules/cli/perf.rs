//! Performance budgets for Workshop CLI.
//!
//! Port from: ~/Dicklesworthstone/destructive_command_guard/src/perf.rs:35-100
//!
//! This module defines explicit latency budgets for all Workshop operations.
//! These constants serve as the source of truth for:
//! - CI benchmark enforcement (fail on regression)
//! - Runtime fail-open thresholds
//! - Documentation and expectations
//!
//! # Budget Philosophy
//!
//! Workshop commands should be fast. We define:
//! - **Target**: Expected p99 latency under normal conditions
//! - **Warning**: Latency that triggers a CI warning
//! - **Panic**: Latency that fails CI or triggers fail-open behavior
//!
//! # Performance Tiers
//!
//! | Tier | Path | Target | Warning | Panic |
//! |------|------|--------|---------|-------|
//! | 0 | Quick reject | < 1μs | < 10μs | > 50μs |
//! | 1 | Fast path | < 75μs | < 200μs | > 500μs |
//! | 2 | Taint check | < 100μs | < 250μs | > 1ms |
//! | 3 | Full pipeline | < 5ms | < 15ms | > 20ms |

use std::time::{Duration, Instant};

/// Performance budget for a single operation tier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PerformanceBudget {
    /// Target p99 latency (expected performance).
    pub target: Duration,
    /// Warning threshold (triggers CI warning).
    pub warning: Duration,
    /// Panic threshold (fails CI, triggers fail-open).
    pub panic: Duration,
}

impl PerformanceBudget {
    /// Create a new budget with microsecond thresholds.
    #[must_use]
    pub const fn from_micros(target_us: u64, warning_us: u64, panic_us: u64) -> Self {
        Self {
            target: Duration::from_micros(target_us),
            warning: Duration::from_micros(warning_us),
            panic: Duration::from_micros(panic_us),
        }
    }

    /// Create a budget from milliseconds (for longer operations).
    #[must_use]
    pub const fn from_ms(target_ms: u64, warning_ms: u64, panic_ms: u64) -> Self {
        Self {
            target: Duration::from_millis(target_ms),
            warning: Duration::from_millis(warning_ms),
            panic: Duration::from_millis(panic_ms),
        }
    }

    /// Quick reject budget for instant operations.
    pub const fn quick_reject() -> Self {
        Self::from_micros(1, 10, 50)
    }

    /// Fast path budget for simple operations.
    pub const fn fast_path() -> Self {
        Self::from_micros(75, 200, 500)
    }

    /// Taint check budget for security operations.
    pub const fn taint_check() -> Self {
        Self::from_micros(100, 250, 1000)
    }

    /// Full pipeline budget for complex operations.
    pub const fn full_pipeline() -> Self {
        Self::from_ms(5, 15, 20)
    }

    /// Cut command budget (extract insight).
    pub const fn cut_command() -> Self {
        Self::from_micros(1, 10, 50)
    }

    /// Carve command budget (search connections).
    pub const fn carve_command() -> Self {
        Self::from_micros(75, 200, 500)
    }

    /// Chamfer command budget (update work).
    pub const fn chamfer_command() -> Self {
        Self::from_micros(5, 20, 100)
    }

    /// Check command budget (validation).
    pub const fn check_command() -> Self {
        Self::from_micros(10, 50, 200)
    }

    /// Check if a duration exceeds the warning threshold.
    #[must_use]
    pub fn exceeds_warning(&self, duration: Duration) -> bool {
        duration > self.warning
    }

    /// Check if a duration exceeds the panic threshold.
    #[must_use]
    pub fn exceeds_panic(&self, duration: Duration) -> bool {
        duration > self.panic
    }

    /// Return the appropriate status for a duration.
    #[must_use]
    pub fn status(&self, duration: Duration) -> BudgetStatus {
        if duration > self.panic {
            BudgetStatus::Panic
        } else if duration > self.warning {
            BudgetStatus::Warning
        } else if duration > self.target {
            BudgetStatus::Elevated
        } else {
            BudgetStatus::Ok
        }
    }
}

impl Default for PerformanceBudget {
    fn default() -> Self {
        Self::fast_path()
    }
}

/// Status result from budget check.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BudgetStatus {
    /// Duration is within target.
    Ok,
    /// Duration exceeds target but within warning.
    Elevated,
    /// Duration exceeds warning but within panic.
    Warning,
    /// Duration exceeds panic threshold.
    Panic,
}

impl BudgetStatus {
    /// Returns true if the status indicates acceptable performance.
    #[must_use]
    pub fn is_ok(&self) -> bool {
        matches!(self, BudgetStatus::Ok | BudgetStatus::Elevated)
    }

    /// Returns true if the status indicates a problem.
    #[must_use]
    pub fn is_problem(&self) -> bool {
        matches!(self, BudgetStatus::Warning | BudgetStatus::Panic)
    }
}

/// A deadline for operation completion, used for fail-open behavior.
///
/// The Deadline tracks when an operation started and how long it's allowed
/// to run. When the deadline is exceeded, expensive operations should be
/// skipped and the command allowed to proceed (fail-open).
#[derive(Debug, Clone, Copy)]
pub struct Deadline {
    /// When the deadline started.
    start: Instant,
    /// Maximum duration allowed.
    max_duration: Duration,
}

impl Deadline {
    /// Create a new deadline with the given maximum duration.
    #[must_use]
    pub fn new(max_duration: Duration) -> Self {
        Self {
            start: Instant::now(),
            max_duration,
        }
    }

    /// Create a deadline from the full pipeline panic threshold.
    #[must_use]
    pub fn default_pipeline() -> Self {
        Self::new(PerformanceBudget::full_pipeline().panic)
    }

    /// Check if the deadline has been exceeded.
    #[must_use]
    pub fn is_exceeded(&self) -> bool {
        self.start.elapsed() > self.max_duration
    }

    /// Get the remaining time before the deadline, or None if exceeded.
    #[must_use]
    pub fn remaining(&self) -> Option<Duration> {
        self.max_duration.checked_sub(self.start.elapsed())
    }

    /// Get the elapsed time since the deadline started.
    #[must_use]
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    /// Get the maximum duration for this deadline.
    #[must_use]
    pub const fn max_duration(&self) -> Duration {
        self.max_duration
    }

    /// Check if there's enough time remaining for an operation with the given budget.
    #[must_use]
    pub fn has_budget_for(&self, budget: &PerformanceBudget) -> bool {
        self.remaining().is_some_and(|r| r > budget.panic)
    }
}

/// Absolute maximum for any operation (fail-open threshold).
pub const ABSOLUTE_MAX: Duration = Duration::from_millis(200);

// =============================================================================
// Preset Budgets
// =============================================================================

/// Budget for commands rejected by quick checks.
pub const QUICK_REJECT: PerformanceBudget = PerformanceBudget::quick_reject();

/// Budget for fast path operations.
pub const FAST_PATH: PerformanceBudget = PerformanceBudget::fast_path();

/// Budget for taint analysis operations.
pub const TAINT_CHECK: PerformanceBudget = PerformanceBudget::taint_check();

/// Budget for full pipeline operations.
pub const FULL_PIPELINE: PerformanceBudget = PerformanceBudget::full_pipeline();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_reject_budget() {
        let budget = PerformanceBudget::quick_reject();
        assert_eq!(budget.target, Duration::from_micros(1));
        assert_eq!(budget.warning, Duration::from_micros(10));
        assert_eq!(budget.panic, Duration::from_micros(50));
    }

    #[test]
    fn test_budget_status() {
        let budget = PerformanceBudget::fast_path();

        assert_eq!(budget.status(Duration::from_micros(50)), BudgetStatus::Ok);
        assert_eq!(budget.status(Duration::from_micros(100)), BudgetStatus::Elevated);
        assert_eq!(budget.status(Duration::from_micros(300)), BudgetStatus::Warning);
        assert_eq!(budget.status(Duration::from_micros(600)), BudgetStatus::Panic);
    }

    #[test]
    fn test_deadline() {
        let deadline = Deadline::new(Duration::from_millis(10));
        assert!(!deadline.is_exceeded());
        assert!(deadline.remaining().is_some());
    }

    #[test]
    fn test_exceeds_methods() {
        let budget = PerformanceBudget::fast_path();

        assert!(!budget.exceeds_warning(Duration::from_micros(100)));
        assert!(budget.exceeds_warning(Duration::from_micros(300)));

        assert!(!budget.exceeds_panic(Duration::from_micros(400)));
        assert!(budget.exceeds_panic(Duration::from_micros(600)));
    }
}
