use std::env;
use std::str::FromStr;

use anyhow::Result;
use secp256k1::SecretKey;
use tracing::info;
use web3::transports::Http;
use web3::types::{Address, TransactionParameters, H256, U256};
use web3::Web3;

use crate::types::SendTx;

pub(crate) async fn build_sign_and_send(info: &SendTx, web3: &Web3<Http>) -> Result<H256> {
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

fn private_key() -> Result<SecretKey> {
    let secret = env::var("SECRET")?;
    Ok(SecretKey::from_str(&secret)?)
}
