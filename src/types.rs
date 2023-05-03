use std::str::FromStr;

use anyhow::bail;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct EvmConfig {
    pub node_url: String,
    pub denom: String,
    pub ticker: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SendTx {
    pub to: String,
    pub amount: usize,
    pub currency: Currency,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum Currency {
    Eth,
    Matic,
}

impl FromStr for Currency {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "Eth" => Ok(Self::Eth),
            "Matic" => Ok(Self::Matic),
            _ => bail!("Not supported"),
        }
    }
}
