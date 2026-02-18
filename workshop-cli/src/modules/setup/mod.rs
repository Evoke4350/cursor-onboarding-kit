//! Setup & Onboarding (O-01 through O-04)
//!
//! Source: ~/arscontexta/generators/claude-md.md

pub mod detect;
pub mod derive;
pub mod generate;
pub mod prompts;

pub use detect::{Environment, detect_environment, detect_environment_in};
pub use derive::{DerivedConfig, WorkType, CodeRefMode, SecurityLevel, Editor, Features};
pub use generate::{generate_workshop, GenerateResult};
pub use prompts::{GumPrompts, WorkshopConfig, prompt_workshop_config};
