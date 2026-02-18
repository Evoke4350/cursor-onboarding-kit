//! Guardrail trait - Abstract base for all security checks
//!
//! Port from: ~/agno/libs/agno/agno/guardrails/base.py:8-20

use anyhow::Result;
use async_trait::async_trait;

/// Base guardrail trait with sync and async check methods
#[async_trait]
pub trait Guardrail: Send + Sync {
    /// Check input synchronously
    fn check(&self, input: &str) -> Result<()>;

    /// Check input asynchronously
    async fn async_check(&self, input: &str) -> Result<()>;
}

// TODO: Implement specific guardrails (PII, injection, etc.)
