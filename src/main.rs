mod agent;
mod config;
mod discovery;
mod launcher;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::discovery::Discovery;

#[derive(Debug, Parser)]
#[command(name = "agenthub", version, about = "Discover and launch local AI coding agents")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Scan the current machine for agent-like tools.
    Scan { #[arg(long)] json: bool },
    /// List discovered tools.
    List { #[arg(long)] json: bool },
    /// Launch a discovered executable in a workspace.
    Run {
        /// Executable name or discovered tool ID.
        agent: String,
        /// Working directory for the process.
        #[arg(short, long, default_value = ".")]
        workspace: PathBuf,
        /// Arguments passed to the executable.
        #[arg(last = true)]
        args: Vec<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let discovery = Discovery::new(config::load()?);

    match cli.command.unwrap_or(Commands::Scan { json: false }) {
        Commands::Scan { json } | Commands::List { json } => {
            let agents = discovery.scan()?;
            if json {
                println!("{}", serde_json::to_string_pretty(&agents)?);
            } else if agents.is_empty() {
                println!("No candidates found. Add a custom executable or install a tool on PATH.");
            } else {
                agent::print_table(&agents);
                println!("\n{} candidate(s) found.", agents.len());
            }
        }
        Commands::Run { agent, workspace, args } => {
            let agents = discovery.scan()?;
            let selected = agents.iter()
                .find(|item| item.id == agent || item.name == agent || item.path.to_string_lossy() == agent)
                .ok_or_else(|| anyhow::anyhow!("No discovered executable matches '{agent}'. Run 'agenthub scan'."))?;
            launcher::launch(selected, &workspace, &args)?;
        }
    }
    Ok(())
}
