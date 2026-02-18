//! Processing Pipeline (P-01 through P-04)
//!
//! Source: ~/arscontexta/generators/features/processing-pipeline.md

pub mod phases;
pub mod queue;
pub mod cut;
pub mod carve;
pub mod chamfer;

pub use phases::{Phase, Pipeline};
pub use queue::{QueuedTask, TaskQueue};
pub use cut::{Shaving, CutConfig, extract_shaving, save_shaving};
pub use carve::{SearchResult, SearchConfig, search, search_shavings};
pub use chamfer::{ChamferResult, ChamferConfig, update_shaving, find_stale_shavings};
