//! Workshop - AI-native cognitive architecture for agentic operators
//!
//! This is the CLI entry point for the Workshop system.
//! See specs/blackboard-bootstrap/CURSOR-BOOTSTRAP.md for full documentation.

use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod cli;
mod modules;

pub use modules::{security, cursor, memory, pipeline, setup, tools, proof, cli as cli_modules};

/// Workshop - Agentic Blackboard CLI
#[derive(Parser, Debug)]
#[command(name = "workshop")]
#[command(author = "Agentic Blackboard Contributors")]
#[command(version = "0.1.0")]
#[command(about = "AI-native cognitive architecture for agentic operators")]
#[command(long_about = "
Workshop is a cognitive architecture for AI agents operating in Cursor IDE.

It provides:
  - Taint analysis for security (protect secrets from exfiltration)
  - Cursor SQLite integration (read conversation history)
  - Knowledge management (UserMemory schema, workshop folders)
  - Processing pipeline (5 Cs: Capture, Cut, Carve, Chamfer, Check)
  - Proof of work (Showboat executable documents)

The guiding principle: \"Ask me clarifying questions until you know what
I want to build and walk me through the setup step by step.\"
")]
struct Cli {
    /// Output in machine-readable JSON format
    #[arg(short, long, global = true, env = "WORKSHOP_ROBOT")]
    robot: bool,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Initialize a new workshop in the current or specified directory
    Init {
        /// Target directory for workshop
        #[arg(default_value = ".")]
        path: String,

        /// Non-interactive mode (use defaults)
        #[arg(short, long)]
        non_interactive: bool,

        /// Work type: software, research, writing
        #[arg(short, long)]
        work_type: Option<String>,

        /// Security level: none, basic, paranoid
        #[arg(short, long)]
        security: Option<String>,
    },

    /// Extract atomic insight from source (P-01)
    Cut {
        /// Source file to extract from
        source: String,

        /// Include code references
        #[arg(short, long)]
        with_code_ref: bool,

        /// Output path for shaving
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Find connections via search (P-02)
    Carve {
        /// Search query
        query: String,

        /// Search in specific domain
        #[arg(short, long)]
        domain: Option<String>,
    },

    /// Update older shavings with new context (P-03)
    Chamfer {
        /// Shaving to update
        shaving: String,

        /// New context to add
        context: String,
    },

    /// Validate everything (P-04)
    Check {
        /// Run adversarial tests
        #[arg(short, long)]
        adversarial: bool,

        /// Verify showboat documents
        #[arg(short, long)]
        verify: bool,
    },

    /// Read Cursor SQLite database (C-01)
    Cursor {
        /// List all conversations
        #[arg(short, long)]
        list: bool,

        /// Export specific conversation
        #[arg(short, long)]
        export: Option<String>,
    },

    /// Taint analysis operations (S-01, S-02)
    Taint {
        /// Show current taint state
        #[arg(short, long)]
        status: bool,

        /// Mark a path as tainted source
        #[arg(short, long)]
        mark: Option<String>,

        /// Check if command would be blocked
        #[arg(short, long)]
        check: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    if cli.verbose {
        tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer())
            .with(tracing_subscriber::EnvFilter::from_default_env())
            .init();
    }

    // Dispatch to command handlers
    match cli.command {
        Commands::Init { path, non_interactive, work_type, security } => {
            cli::init::run(path, non_interactive, work_type, security, cli.robot)
        }
        Commands::Cut { source, with_code_ref, output } => {
            cli::cut::run(source, with_code_ref, output, cli.robot)
        }
        Commands::Carve { query, domain } => {
            cli::carve::run(query, domain, cli.robot)
        }
        Commands::Chamfer { shaving, context } => {
            cli::chamfer::run(shaving, context, cli.robot)
        }
        Commands::Check { adversarial, verify } => {
            cli::check::run(adversarial, verify, cli.robot)
        }
        Commands::Cursor { list, export } => {
            cli::cursor::run(list, export, cli.robot)
        }
        Commands::Taint { status, mark, check } => {
            cli::taint::run(status, mark, check, cli.robot)
        }
    }
}
