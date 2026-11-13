use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub custom_executables: Vec<PathBuf>,
}

pub fn config_path() -> Option<PathBuf> {
    dirs::config_dir().map(|root| root.join("agenthub").join("config.json"))
}

pub fn load() -> Result<Config> {
    let Some(path) = config_path() else { return Ok(Config::default()) };
    if !path.exists() { return Ok(Config::default()); }
    Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
}
