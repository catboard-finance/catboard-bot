use pyth_client::{load_mapping, load_price, load_product, PriceConf, PriceStatus};

use crate::solana::{
    pubkey::Pubkey,
    web3::{get_account_data, Cluster},
};
use std::collections::HashMap;
use std::str::FromStr;

#[allow(dead_code)]
fn get_pyth_mapping_account(target: &Cluster) -> &'static str {
    match target {
        Cluster::MainnetBeta => "AHtgzX45WTKfkPG53L6WYhGEXwQkN1BVknET3sVsLL8J",
        Cluster::Testnet => "AFmdnt9ng1uVxqCmqwQJDAYC5cKTkw8gJKSM5PnzuF6z",
        Cluster::Devnet => "BmA9Z6FjioHJPpjT39QazZyhDRUdZy2ezwx4GiDdE2u2",
        _ => panic!("Unsupported cluster"),
    }
}

#[allow(dead_code)]
fn get_attr_str<'a, T>(ite: &mut T) -> String
where
    T: Iterator<Item = &'a u8>,
{
    let mut len = *ite.next().unwrap() as usize;
    let mut val = String::with_capacity(len);
    while len > 0 {
        val.push(*ite.next().unwrap() as char);
        len -= 1;
    }
    return val;
}

#[allow(dead_code)]
pub(crate) fn get_status(st: &PriceStatus) -> &'static str {
    match st {
        PriceStatus::Unknown => "unknown",
        PriceStatus::Trading => "trading",
        PriceStatus::Halted => "halted",
        PriceStatus::Auction => "auction",
    }
}

#[allow(dead_code)]
pub(crate) async fn fetch_pyth_product_account_by_symbol(
    cluster: &Cluster,
    symbol: &str,
) -> Option<Pubkey> {
    let product_accounts = fetch_pyth_product_accounts(cluster, Some(symbol)).await;
    Some(*product_accounts.get(symbol).unwrap())
}

#[allow(dead_code)]
pub(crate) async fn fetch_pyth_product_accounts(
    cluster: &Cluster,
    symbol: Option<&str>,
) -> HashMap<String, Pubkey> {
    let addr = get_pyth_mapping_account(cluster);
    let mut akey = Pubkey::from_str(&addr).unwrap();

    let mut product_accounts = HashMap::new();

    loop {
        // get Mapping account from key
        let map_data: &[u8] = &get_account_data(&cluster, &akey).await;
        let map_acct = load_mapping(&map_data).unwrap();

        for prod_akey in &map_acct.products {
            let prod_pkey = Pubkey::new(&prod_akey.val);
            let prod_data: &[u8] = &get_account_data(&cluster, &prod_pkey).await;
            let prod_acct = match load_product(&prod_data) {
                Ok(prod_acct) => prod_acct,
                Err(_) => break,
            };

            // print key and reference data for this Product
            // println!("prod_pkey .. {:?}", prod_pkey);
            let mut pit = (&prod_acct.attr[..]).iter();

            let _ = get_attr_str(&mut pit);
            let val = get_attr_str(&mut pit);
            // println!("  {:.<16} {}", key, val);

            // Valid?
            if prod_acct.px_acc.is_valid() {
                // Then keep it
                product_accounts.insert(val.clone(), Pubkey::new(&prod_acct.px_acc.val));
            }

            // Found specific symbol?
            if symbol.is_some() && val == symbol.unwrap().to_string() {
                // Found specific symbol
                break;
            }
        }

        // go to next Mapping account in list
        if !map_acct.next.is_valid() {
            break;
        }
        akey = Pubkey::new(&map_acct.next.val);
    }

    product_accounts
}

#[allow(dead_code)]
pub(crate) async fn fetch_pyth_price_by_symbol(
    cluster: &Cluster,
    symbol: &str,
) -> Option<PriceConf> {
    // Get product account
    let px_pkeys = fetch_pyth_product_accounts(&cluster, Some(symbol)).await;

    // Guard none px_pkey
    if !px_pkeys.contains_key(symbol) {
        return None;
    }

    let px_pkey = *px_pkeys.get(symbol).unwrap();

    // Get price
    fetch_pyth_price_by_pubkey(&cluster, &px_pkey).await
}

pub(crate) async fn fetch_pyth_price_by_pubkey(
    cluster: &Cluster,
    px_pkey: &Pubkey,
) -> Option<PriceConf> {
    let mut maybe_price;
    let mut px_pkey = *px_pkey;
    loop {
        let pd: &[u8] = &get_account_data(&cluster, &px_pkey).await;
        let pa = load_price(&pd).unwrap();

        maybe_price = pa.get_current_price();
        let maybe_price = pa.get_current_price();
        match maybe_price {
            Some(p) => {
                println!("price  : {} x 10^{}", p.price, p.expo);
                println!("conf   : {} x 10^{}", p.conf, p.expo);
            }
            None => {
                println!("price  : unavailable");
                println!("conf   : unavailable");
            }
        }

        println!("status : {}", get_status(&pa.agg.status));

        // go to next price account in list
        if pa.next.is_valid() {
            px_pkey = Pubkey::new(&pa.next.val);
        } else {
            break;
        }
    }

    maybe_price
}

#[cfg(test)]
#[tokio::test]
async fn test_fetch_pyth_product_accounts() {
    let cluster = Cluster::Devnet;
    let product_accounts = fetch_pyth_product_accounts(&cluster, None).await;

    println!("product_accounts: {:?}", product_accounts);
    assert_eq!(product_accounts.is_empty(), false);
    assert!(product_accounts.capacity() > 1);
}

#[cfg(test)]
#[tokio::test]
async fn test_fetch_pyth_product_account_by_symbol() {
    let cluster = Cluster::Devnet;
    let symbol = "Crypto.ORCA/USD";
    let product_account = fetch_pyth_product_account_by_symbol(&cluster, symbol).await;

    println!("product_account: {:?}", product_account);
    assert_eq!(
        product_account.unwrap().to_string(),
        "A1WttWF7X3Rg6ZRpB2YQUFHCRh1kiXV8sKKLV3S9neJV".to_string()
    );
}

#[cfg(test)]
#[tokio::test]
async fn test_fetch_pyth_price_by_symbol() {
    let cluster = Cluster::Devnet;
    let symbol = "Crypto.ORCA/USD";
    let current_price = fetch_pyth_price_by_symbol(&cluster, symbol).await;

    println!("current_price: {:?}", current_price);
    assert_ne!(current_price, None);
}

#[cfg(test)]
#[tokio::test]
async fn test_fetch_pyth_price_by_pubkey() {
    let cluster = Cluster::Devnet;
    // Mocked SOL/USD
    let address = Pubkey::from_str("A1WttWF7X3Rg6ZRpB2YQUFHCRh1kiXV8sKKLV3S9neJV").unwrap();

    // Fetch price from pyth
    let price_conf = fetch_pyth_price_by_pubkey(&cluster, &address).await;

    println!("price_conf: {:?}", price_conf);
    assert_ne!(price_conf, None);
}
