use crate::{
    cloudflare::worker::WorkersKv,
    pyth::adaptor::fetch_pyth_price_by_pubkey,
    solana::{pubkey::Pubkey, web3::Cluster},
};
use rusty_money::{define_currency_set, Money, Round};
use std::str::FromStr;

// TODO: cached this
define_currency_set!(
  crypto {
    BTC: {
        code: "BTC",
        exponent: 8,
        locale: EnUs,
        minor_units: 100_000_000,
        name: "Bitcoin",
        symbol: "",
        symbol_first: false,
    },
    ETH: {
        code: "ETH",
        exponent: 18,
        locale: EnUs,
        minor_units: 1_000_000_000_000_000_000,
        name: "Ethereum",
        symbol: "",
        symbol_first: false,
    },
    SOL: {
      code: "SOL",
      exponent: 8,
      locale: EnUs,
      minor_units: 100_000_000,
      name: "Solana",
      symbol: "",
      symbol_first: false,
    }
  }
);

#[allow(dead_code)]
async fn get_kv_text(kv: &WorkersKv, key: &str) -> String {
    kv.get_text(&key)
        .await
        .unwrap_or_default()
        .unwrap_or_default()
}

#[allow(dead_code)]
pub(crate) fn get_supported_symbols() -> Vec<&'static str> {
    // TODO: read from kv
    let text = "Crypto.BTC/USD,Crypto.ETH/USD,Crypto.SOL/USD";
    let texts: Vec<&str> = text.split(",").collect();
    let pairs: Vec<&str> = texts.iter().flat_map(|p| p.split(".").nth(1)).collect();
    let symbols: Vec<&str> = pairs.iter().flat_map(|p| p.split("/").nth(0)).collect();
    symbols
}

pub(crate) async fn get_price_account_from_kv_by_symbol(kv: &WorkersKv, symbol: &str) -> String {
    let symbol = format!("{}", symbol.to_uppercase());
    let pair = format!("{}/USD", symbol);
    let key = format!("Crypto.{}:price_account", pair);

    // Get price_account from kv
    let price_account = get_kv_text(&kv, &key).await;

    price_account
}

pub(crate) async fn get_formatted_price_from_pyth(
    cluster: &Cluster,
    symbol: &str,
    price_account: &str,
) -> String {
    // Fetch price from pyth
    let pubkey = Pubkey::from_str(price_account).unwrap();
    let price_conf = fetch_pyth_price_by_pubkey(cluster, &pubkey).await.unwrap();

    let price = Money::from_minor(price_conf.price, crypto::SOL);
    let price_round_up = price.round(2, Round::HalfUp);
    let conf = (price_conf.conf as f64) / (crypto::SOL.minor_units as f64);
    let content = format!(
        "`{}` = `${}` 🎯`±{:.2}`",
        symbol.to_uppercase(),
        price_round_up,
        conf
    );

    content
}

// static mut STATE: &'static str = "";

// #[no_mangle]
// pub extern "C" fn add_product_list(product_list: &'static str) {
//     unsafe { STATE = &product_list.clone() };
// }

// #[no_mangle]
// pub extern "C" fn get_product_list() -> &'static str {
//     unsafe { STATE }
// }

// pub(crate) async fn feed(kv: &WorkersKv) -> Result<Response, Error> {
//     let result = fetch_pyth_prices_and_record(kv).await;
//     Ok(())
// }
