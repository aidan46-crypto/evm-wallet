use std::collections::HashMap;

use actix_web::web::{Data, Json};
use actix_web::{get, post, HttpResponse};
use tokio::sync::Mutex;
use tracing::{debug, error, info};

use crate::blockchain::{build_sign_and_send, get_balance_internal, get_web3};
use crate::types::{Currency, EvmConfig, SendTx};

#[post("/blockchain/send")]
pub async fn send_tx(
    evm_map: Data<Mutex<HashMap<Currency, EvmConfig>>>,
    send_info: Json<SendTx>,
) -> HttpResponse {
    let info = send_info.into_inner();
    let map = evm_map.lock().await;
    info!("POST \"/blockchain/send\": {info:#?}");
    debug!("{map:?}");
    let config = match map.get(&info.currency) {
        Some(config) => config,
        None => {
            let e = format!("Config for {:?} not found", info.currency);
            error!(e);
            return HttpResponse::InternalServerError().json(e);
        }
    };
    let web3 = match get_web3(config) {
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
    let config = match map.get(&currency) {
        Some(config) => config,
        None => {
            let e = format!("Config for {currency:?} not found");
            error!(e);
            return HttpResponse::InternalServerError().json(e);
        }
    };
    match get_balance_internal(config).await {
        Ok(json) => HttpResponse::Ok().json(json),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

#[get("/blockchain/balance_all")]
pub async fn get_balance_all(evm_map: Data<Mutex<HashMap<Currency, EvmConfig>>>) -> HttpResponse {
    let map = evm_map.lock().await;
    info!("GET: \"/blockchain/balance_all\"");
    let mut balances = vec![];
    let keys = map.keys().clone();
    for currency in keys {
        // Unwrap safe cause we loop over hashmap keys
        let config = map.get(currency).unwrap();
        match get_balance_internal(config).await {
            Ok(json) => balances.push(json),
            Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
        }
    }
    HttpResponse::Ok().json(balances)
}
