use std::collections::HashMap;
use std::env;
use std::str::FromStr;

use actix_web::web::{Data, Json};
use actix_web::{post, HttpResponse};
use anyhow::Result;
use secp256k1::SecretKey;
use tokio::sync::{Mutex, MutexGuard};
use tracing::{debug, info};
use web3::transports::Http;
use web3::types::{Address, TransactionParameters, H160, H256, U256};
use web3::Web3;

use crate::types::{Currency, EvmConfig, SendTx};

#[post("/blockchain/send")]
pub async fn send_tx(
    evm_map: Data<Mutex<HashMap<Currency, EvmConfig>>>,
    send_info: Json<SendTx>,
) -> HttpResponse {
    let info = send_info.into_inner();
    let map = evm_map.lock().await;
    info!("POST \"/blockchain/send\": {info:?}");
    debug!("{map:?}");
    let web3 = match get_web3(map, &info.currency) {
        Ok(web3) => web3,
        Err(response) => return response,
    };
    match build_sign_and_send(&info, &web3).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

#[post("/blockchain/balance")]
pub async fn get_balance(
    evm_map: Data<Mutex<HashMap<Currency, EvmConfig>>>,
    currency: Json<Currency>,
) -> HttpResponse {
    let map = evm_map.lock().await;
    let currency = currency.into_inner();
    info!("GET \"/blockchain/balance\": {currency:?}");
    let web3 = match get_web3(map, &currency) {
        Ok(web3) => web3,
        Err(response) => return response,
    };
    let account = match account() {
        Ok(account) => account,
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
    };
    match web3.eth().balance(account, None).await {
        Ok(bal) => {
            let bal = wei_to_eth(bal);
            debug!("Balance for {account:?} = {bal:?}");
            HttpResponse::Ok().json(format!("{bal} {currency:?}"))
        }
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

fn wei_to_eth(wei_val: U256) -> f64 {
    let res = wei_val.as_u128() as f64;
    res / 1_000_000_000_000_000_000.0
}

fn get_web3(
    map: MutexGuard<HashMap<Currency, EvmConfig>>,
    currency: &Currency,
) -> anyhow::Result<Web3<Http>, HttpResponse> {
    match map.get(currency) {
        Some(config) => {
            debug!("{config:?}");
            match web3_transport(config) {
                Ok(web3) => Ok(web3),
                Err(e) => Err(HttpResponse::InternalServerError().json(e.to_string())),
            }
        }
        None => {
            Err(HttpResponse::InternalServerError().json(format!("{:?} not supported", currency)))
        }
    }
}

async fn build_sign_and_send(info: &SendTx, web3: &Web3<Http>) -> anyhow::Result<H256> {
    let prvk = private_key()?;
    let to = Address::from_str(&info.to)?;
    // Build the tx object
    let tx_object = TransactionParameters {
        to: Some(to),
        value: U256::from(info.amount),
        ..Default::default()
    };

    // Sign the tx
    let signed = web3.accounts().sign_transaction(tx_object, &prvk).await?;

    info!("Submitting signed tx: {signed:?}");
    // Send the tx
    Ok(web3
        .eth()
        .send_raw_transaction(signed.raw_transaction)
        .await?)
}

fn web3_transport(config: &EvmConfig) -> Result<Web3<Http>> {
    let transport = Http::new(&config.node_url)?;
    Ok(Web3::new(transport))
}

fn private_key() -> Result<SecretKey> {
    let secret = env::var("SECRET")?;
    Ok(SecretKey::from_str(&secret)?)
}

fn account() -> Result<H160> {
    let account = env::var("ACCOUNT")?;
    Ok(H160::from_str(&account)?)
}
