use anyhow::Result;
use std::{collections::BTreeMap, env, fs, path::{Path, PathBuf}};
use crate::{agent::Agent, config::Config};

pub struct Discovery { config: Config }

impl Discovery {
    pub fn new(config: Config) -> Self { Self { config } }

    /// Discover executable candidates without maintaining a product catalogue.
    /// Recognition is heuristic; unknown binaries remain visible as discovered candidates.
    pub fn scan(&self) -> Result<Vec<Agent>> {
        let mut found = BTreeMap::<String, Agent>::new();
        if let Some(paths) = env::var_os("PATH") {
            for directory in env::split_paths(&paths) {
                self.scan_directory(&directory, "PATH", &mut found)?;
            }
        }
        for path in &self.config.custom_executables {
            if is_executable_file(path) {
                found.entry(canonical_key(path)).or_insert_with(|| Agent::new(path.clone(), "custom", is_agent_like(path)));
            }
        }
        let mut agents: Vec<_> = found.into_values().collect();
        agents.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        Ok(agents)
    }

    fn scan_directory(&self, directory: &Path, source: &str, found: &mut BTreeMap<String, Agent>) -> Result<()> {
        let entries = match fs::read_dir(directory) { Ok(entries) => entries, Err(_) => return Ok(()) };
        for entry in entries.flatten() {
            let path = entry.path();
            if !is_executable_file(&path) { continue; }
            found.entry(canonical_key(&path)).or_insert_with(|| Agent::new(path.clone(), source, is_agent_like(&path)));
        }
        Ok(())
    }
}

fn canonical_key(path: &Path) -> String {
    fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf()).to_string_lossy().to_lowercase()
}

#[cfg(unix)]
fn is_executable_file(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    fs::metadata(path).map(|m| m.is_file() && m.permissions().mode() & 0o111 != 0).unwrap_or(false)
}

#[cfg(windows)]
fn is_executable_file(path: &Path) -> bool {
    path.is_file() && path.extension().and_then(|e| e.to_str())
        .map(|e| ["exe", "cmd", "bat", "com"].iter().any(|v| e.eq_ignore_ascii_case(v))).unwrap_or(false)
}

#[cfg(not(any(unix, windows)))]
fn is_executable_file(path: &Path) -> bool { path.is_file() }

/// Generic signals only; this intentionally does not enumerate known agent products.
fn is_agent_like(path: &Path) -> bool {
    let name = path.file_name().and_then(|v| v.to_str()).unwrap_or("").to_ascii_lowercase();
    ["agent", "assistant", "copilot", "llm", "code-assist", "coding"]
        .iter().any(|signal| name.contains(signal))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn recognizes_generic_signals() {
        assert!(is_agent_like(Path::new("/usr/bin/my-agent-cli")));
        assert!(is_agent_like(Path::new("/usr/bin/local-coding-assistant")));
        assert!(!is_agent_like(Path::new("/usr/bin/git")));
    }
}
