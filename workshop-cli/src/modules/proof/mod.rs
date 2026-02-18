//! Proof of Work (W-01 through W-05)
//!
//! Source: ~/showboat, ~/chartroom

pub mod showboat;
pub mod verify;
pub mod chart;

pub use showboat::ShowboatDoc;
pub use verify::verify_document;
pub use chart::create_chart;
