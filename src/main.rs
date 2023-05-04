use std::collections::HashMap;

use actix_web::web::Data;
use actix_web::{App, HttpServer};
use anyhow::Result;
use dotenv::dotenv;
use routes::{add_config, get_balance, get_balance_all, list_config, send_tx};
use tokio::sync::Mutex;
use tracing::{info, Level};

use crate::types::{Currency, EvmConfig};

mod blockchain;
mod routes;
mod types;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv()?;
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();
    let addr = "127.0.0.1";
    let port = 3000;
    info!("Starting server at {addr}:{port}");
    HttpServer::new(move || {
        let evm_map = Data::new(Mutex::new(HashMap::<Currency, EvmConfig>::new()));
        App::new()
            .app_data(evm_map)
            .service(add_config)
            .service(list_config)
            .service(send_tx)
            .service(get_balance)
            .service(get_balance_all)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await?;
    Ok(())
}
