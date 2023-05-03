use std::collections::HashMap;
use std::env;
use std::str::FromStr;

use actix_web::web::{Data, Json};
use actix_web::{post, HttpResponse};
use anyhow::Result;
use secp256k1::SecretKey;
use tokio::sync::Mutex;
use tracing::{debug, info};
use web3::transports::Http;
use web3::types::{Address, TransactionParameters, H256, U256};
use web3::Web3;

use crate::types::{Currency, EvmConfig, SendTx};

#[post("/send")]
pub async fn send_tx(
    evm_map: Data<Mutex<HashMap<Currency, EvmConfig>>>,
    send_info: Json<SendTx>,
) -> HttpResponse {
    let info = send_info.into_inner();
    let map = evm_map.lock().await;
    info!("POST \"/send\": {info:?}");
    debug!("{map:?}");
    let web3 = match map.get(&info.currency) {
        Some(config) => get_web3(config).unwrap(),
        None => {
            return HttpResponse::InternalServerError()
                .json(format!("{:?} not supported", info.currency))
        }
    };
    match build_sign_and_send(&info, &web3).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
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

fn get_web3(config: &EvmConfig) -> Result<Web3<Http>> {
    let transport = Http::new(&config.node_url)?;
    Ok(Web3::new(transport))
}

fn private_key() -> Result<SecretKey> {
    let secret = env::var("SECRET")?;
    Ok(SecretKey::from_str(&secret)?)
}
