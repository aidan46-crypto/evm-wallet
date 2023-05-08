use std::fs;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::types::EvmConfig;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConfigVec {
    pub config: Vec<EvmConfig>,
}

pub fn add_config_to_toml(path: &str, config: &EvmConfig) -> Result<()> {
    let mut toml = from_toml(path)?;
    toml.config.push(config.clone());
    let toml_string = toml::to_string(&toml)?;
    fs::write(path, toml_string)?;
    Ok(())
}

pub fn from_toml(path: &str) -> Result<ConfigVec> {
    let file = fs::read_to_string(path)?;
    let config: ConfigVec = toml::from_str(&file)?;
    Ok(config)
}
