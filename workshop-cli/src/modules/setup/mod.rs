//! Setup & Onboarding (O-01 through O-04)
//!
//! Source: ~/arscontexta/generators/claude-md.md

pub mod detect;
pub mod derive;
pub mod generate;
pub mod prompts;

pub use detect::detect_environment;
pub use derive::DerivedConfig;
pub use generate::generate_workshop;
pub use prompts::{GumPrompts, WorkshopConfig, prompt_workshop_config};
