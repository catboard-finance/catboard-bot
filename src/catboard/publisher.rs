use std::collections::HashMap;

use crate::{
    cloudflare::worker::WorkersKv,
    error::Error,
    pyth::adaptor::{fetch_pyth_price_by_pubkey, fetch_pyth_product_account_by_symbol},
    solana::{pubkey::Pubkey, web3::Cluster},
};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::str::FromStr;

#[derive(PartialEq, Debug, Clone, Deserialize, Serialize)]
struct RecordPrice {
    pub symbol: String,
    pub px_pkey: String,
}

impl RecordPrice {
    #[allow(dead_code)]
    fn new(symbol: &str, px_pkey: &str) -> RecordPrice {
        RecordPrice {
            symbol: symbol.to_string(),
            px_pkey: px_pkey.to_string(),
        }
    }
}

// TODO: finish this
pub(crate) async fn fetch_pyth_prices_and_record(
    kv: &WorkersKv,
    _cluster: &Cluster,
    _symbols: Vec<&str>,
) -> Result<String, Error> {
    // BTC, SOL, ETH, BNB
    let price_account_map = HashMap::from([
        (
            "Crypto.BTC/USD".to_string(),
            "HovQMDrbAgAYPCmHVSrezcSmkMtXSSUsLDFANExrZh2J".to_string(),
        ),
        (
            "Crypto.ETH/USD".to_string(),
            "EdVCmQ9FSPcVe5YySXDPCRmc8aDQLKJ9xvYBMZPie1Vw".to_string(),
        ),
    ]);

    // loop fetch
    // let mut prices: Vec<RecordPrice> = Vec::new();
    for (symbol, px_pkey) in price_account_map.iter() {
        // pubkey
        let pubkey = Pubkey::from_str(px_pkey).unwrap();

        // fetch
        let price_conf = fetch_pyth_price_by_pubkey(&Cluster::Devnet, &pubkey).await;

        let price = price_conf.unwrap().price.to_string();
        // prices.push(RecordPrice::new(symbol, px_pkey));

        // kv
        // `price:Crypto.SOL/USD:2022-01-01` = `{index,low,open,close,average,high}`
        let utc = Utc::now().to_string();
        let today: Vec<&str> = utc.split('T').collect();
        let key = format!("{}:{}", symbol, today[0]);
        let value = json!({
            "close": price,
        });

        kv.put_text(&key, value.as_str().unwrap(), 60 * 60 * 24 * 365)
            .await
            .unwrap_or_default();
    }

    Ok("ok".to_string())
}

pub(crate) async fn fetch_pyth_product_and_record(
    kv: &WorkersKv,
    cluster: &Cluster,
    symbols: Vec<&str>,
) -> Result<String, Error> {
    // TOFIX : script exceeded time limit when symbols > 3
    // let symbols = ["Crypto.BTC/USD", "Crypto.ETH/USD", "Crypto.SOL/USD"];
    let mut product_fetched = 0;
    for symbol in symbols.iter() {
        // Get product account from Pyth
        let product_account = fetch_pyth_product_account_by_symbol(cluster, symbol).await;

        // Write to KV
        let key = format!("{}:price_account", symbol);
        let product_account = product_account.unwrap().to_string();
        kv.put_text(&key, product_account.as_str(), 60 * 60 * 24 * 365)
            .await
            .unwrap_or_default();

        product_fetched += 1;
    }

    // Result
    Ok(json!({ "completed": product_fetched }).to_string())
}
