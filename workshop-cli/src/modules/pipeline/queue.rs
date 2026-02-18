//! Task queue for pipeline
//!
//! Port from: processing-pipeline.md:117-176

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Task status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Complete,
    Failed,
}

/// Queued task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueuedTask {
    pub id: String,
    pub source_path: PathBuf,
    pub status: TaskStatus,
    pub created_at: i64,
    pub updated_at: i64,
}

/// Task queue
pub struct TaskQueue {
    path: PathBuf,
}

impl TaskQueue {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn push(&mut self, _task: QueuedTask) -> anyhow::Result<()> {
        // TODO: Implement
        Ok(())
    }

    pub fn pop(&mut self) -> anyhow::Result<Option<QueuedTask>> {
        // TODO: Implement
        Ok(None)
    }
}
