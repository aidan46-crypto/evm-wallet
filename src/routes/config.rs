use std::collections::HashMap;

use actix_web::web::{Data, Json};
use actix_web::{get, post, HttpResponse};
use tokio::sync::Mutex;
use tracing::{debug, error, info};

use crate::config::add_config_to_toml;
use crate::types::{Currency, EvmConfig};

#[post("/config/add")]
pub async fn add_config(
    evm_map: Data<Mutex<HashMap<Currency, EvmConfig>>>,
    config: Json<EvmConfig>,
) -> HttpResponse {
    let evm = config.into_inner();
    info!("POST \"/config/add\": {evm:#?}");
    let mut map = evm_map.lock().await;
    let currency = evm.currency;
    if let Some(..) = map.get(&currency) {
        let e = format!("Config for {currency:?} already present");
        error!("{e}");
        return HttpResponse::InternalServerError().json(e);
    }
    debug!("Adding {evm:#?} to configs");
    match add_config_to_toml("toml/config.toml", &evm) {
        Ok(()) => {
            map.insert(currency, evm);
            debug!("{map:#?}");
            HttpResponse::Ok().json(format!("Currency {currency:?} added"))
        }
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

#[get("/config/list")]
pub async fn list_config(evm_map: Data<Mutex<HashMap<Currency, EvmConfig>>>) -> HttpResponse {
    info!("GET \"/config/list\"");
    let map = evm_map.lock().await;
    let mut list = vec![];
    for item in map.iter() {
        debug!("{item:#?}");
        list.push(item);
    }
    HttpResponse::Ok().json(list)
}
