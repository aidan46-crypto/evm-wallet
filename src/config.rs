use std::fs;
use std::vec::IntoIter;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::types::EvmConfig;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    config: Vec<EvmConfig>,
}

impl IntoIterator for Config {
    type Item = EvmConfig;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.config.into_iter()
    }
}

pub fn add_config_to_toml(path: &str, config: &EvmConfig) -> Result<()> {
    let mut toml = from_toml(path)?;
    toml.config.push(config.clone());
    let toml_string = toml::to_string(&toml)?;
    fs::write(path, toml_string)?;
    Ok(())
}

pub fn from_toml(path: &str) -> Result<Config> {
    let file = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&file)?;
    Ok(config)
}
