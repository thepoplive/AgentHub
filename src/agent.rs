use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentStatus {
    Discovered,
    Recognized,
    Supported,
    Launchable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: String,
    pub name: String,
    pub path: PathBuf,
    pub source: String,
    pub status: AgentStatus,
    pub adapter: String,
}

impl Agent {
    pub fn new(path: PathBuf, source: impl Into<String>, recognized: bool) -> Self {
        let name = path.file_name().and_then(|v| v.to_str()).unwrap_or("unknown").to_owned();
        let id = format!("{}-{:x}", slug(&name), stable_hash(&path.to_string_lossy()));
        Self {
            id, name, path, source: source.into(),
            status: if recognized { AgentStatus::Launchable } else { AgentStatus::Discovered },
            adapter: "generic-process".to_owned(),
        }
    }
}

fn slug(value: &str) -> String {
    value.chars().map(|ch| if ch.is_ascii_alphanumeric() { ch.to_ascii_lowercase() } else { '-' })
        .collect::<String>().trim_matches('-').to_owned()
}

fn stable_hash(value: &str) -> u64 {
    value.bytes().fold(0xcbf29ce484222325, |hash, byte| (hash ^ u64::from(byte)).wrapping_mul(0x100000001b3))
}

pub fn print_table(agents: &[Agent]) {
    println!("{:<24} {:<14} {:<20} PATH", "NAME", "STATUS", "SOURCE");
    println!("{}", "-".repeat(90));
    for item in agents {
        let status = match item.status {
            AgentStatus::Discovered => "discovered",
            AgentStatus::Recognized => "recognized",
            AgentStatus::Supported => "supported",
            AgentStatus::Launchable => "launchable",
        };
        println!("{:<24} {:<14} {:<20} {}", item.name, status, item.source, item.path.display());
    }
}
