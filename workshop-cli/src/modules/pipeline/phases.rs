//! Pipeline phases (5 Cs framework)
//!
//! Port from: processing-pipeline.md:12-89

use anyhow::Result;

/// Processing phases
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    /// Zero-friction capture to inbox
    Capture,
    /// Extract atomic insight
    Cut,
    /// Find connections
    Carve,
    /// Update older notes
    Chamfer,
    /// Validate
    Check,
}

/// Pipeline processor
pub struct Pipeline {
    current_phase: Phase,
}

impl Pipeline {
    pub fn new() -> Self {
        Self {
            current_phase: Phase::Capture,
        }
    }

    /// Run current phase
    pub fn run_phase(&mut self, phase: Phase) -> Result<()> {
        self.current_phase = phase;
        // TODO: Implement phase logic
        Ok(())
    }
}

impl Default for Pipeline {
    fn default() -> Self {
        Self::new()
    }
}
