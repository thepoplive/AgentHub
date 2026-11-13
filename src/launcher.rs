use anyhow::{Context, Result};
use std::{path::Path, process::Command};
use crate::agent::Agent;

pub fn launch(agent: &Agent, workspace: &Path, args: &[String]) -> Result<()> {
    let workspace = workspace.canonicalize().with_context(|| format!("Workspace does not exist: {}", workspace.display()))?;
    let status = Command::new(&agent.path).args(args).current_dir(&workspace).status()
        .with_context(|| format!("Failed to launch {}", agent.path.display()))?;
    if !status.success() { anyhow::bail!("Agent exited with status: {status}"); }
    Ok(())
}
