use std::path::Path;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub(crate) prefix: String,
    pub(crate) minimum_commit_size: i32,
    pub(crate) csv_file: String,
}

impl Config {}

pub fn read_config<P: AsRef<Path>>(path: P) -> anyhow::Result<Config> {
    let content = std::fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}
