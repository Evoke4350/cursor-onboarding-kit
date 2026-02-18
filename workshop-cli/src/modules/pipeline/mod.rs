//! Processing Pipeline (P-01 through P-04)
//!
//! Source: ~/arscontexta/generators/features/processing-pipeline.md

pub mod phases;
pub mod queue;

pub use phases::{Phase, Pipeline};
pub use queue::{QueuedTask, TaskQueue};
