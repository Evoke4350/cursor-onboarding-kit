//! Workshop library - AI-native cognitive architecture
//!
//! This crate provides the core functionality for the Workshop CLI.

pub mod cli;
pub mod modules;

// Re-export commonly used modules for convenience
pub use modules::{security, cursor, memory, pipeline, setup, tools, proof};
