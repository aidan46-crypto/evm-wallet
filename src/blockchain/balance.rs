use std::env;
use std::str::FromStr;

use actix_web::web::Json;
use anyhow::{bail, Result};
use tracing::{debug, error, info};
use web3::types::{H160, U256};

use super::get_web3;
use crate::types::EvmConfig;

pub(crate) async fn get_balance_internal(config: &EvmConfig) -> Result<Json<String>> {
    let currency = &config.currency;
    info!("Getting balance for {config:?}");
    let web3 = match get_web3(config) {
        Ok(web3) => web3,
        Err(response) => {
            error!("{response:?}");
            bail!("{response:#?}")
        }
    };
    let account = match account() {
        Ok(account) => account,
        Err(e) => {
            error!("{e}");
            return Err(e);
        }
    };
    match web3.eth().balance(account, None).await {
        Ok(bal) => {
            let bal = wei_to_eth(bal);
            debug!("Balance for {account:?} = {bal:?} {currency:?}");
            Ok(Json(format!("{bal} {currency:?}")))
        }
        Err(e) => {
            error!("{e}");
            Err(e.into())
        }
    }
}

fn wei_to_eth(wei_val: U256) -> f64 {
    let res = wei_val.as_u128() as f64;
    res / 1_000_000_000_000_000_000.0
}

fn account() -> Result<H160> {
    let account = env::var("ACCOUNT")?;
    Ok(H160::from_str(&account)?)
}
