use actix_web::HttpResponse;
use anyhow::Result;
use tracing::info;
use web3::transports::Http;
use web3::Web3;

use crate::types::EvmConfig;

mod balance;
mod sign;

pub(crate) use balance::get_balance_internal;
pub(crate) use sign::build_sign_and_send;

pub(crate) fn get_web3(config: &EvmConfig) -> Result<Web3<Http>, HttpResponse> {
    info!("Getting web3 for {config:?}");
    match web3_transport(config) {
        Ok(web3) => Ok(web3),
        Err(e) => Err(HttpResponse::InternalServerError().json(e.to_string())),
    }
}

fn web3_transport(config: &EvmConfig) -> Result<Web3<Http>> {
    let transport = Http::new(&config.node_url)?;
    Ok(Web3::new(transport))
}
