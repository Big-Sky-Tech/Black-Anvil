use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Deserialize, Default)]
pub struct Config {
    pub project_path: Option<String>,
    pub build_type: Option<String>,
    pub install_dir: Option<String>,
    pub vendor: Option<bool>,
}

impl Config {
    pub fn from_file(path: &Path) -> Result<Self> {
        let contents = fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file {}", path.display()))?;
        let cfg = toml::from_str(&contents).context("Failed to parse config")?;
        Ok(cfg)
    }
}
