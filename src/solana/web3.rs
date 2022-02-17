use serde_json::json;

use crate::solana::pubkey::Pubkey;

#[allow(dead_code)]
pub(crate) enum Cluster {
    Development,
    Devnet,
    Testnet,
    MainnetBeta,
}

fn get_cluster_api_url(cluster: &Cluster) -> &'static str {
    match cluster {
        // *self has type Direction
        Cluster::Development => "https://api.devnet.solana.com",
        Cluster::Devnet => "https://api.devnet.solana.com",
        Cluster::Testnet => "https://api.testnet.solana.com",
        Cluster::MainnetBeta => "https://api.mainnet-beta.solana.com",
    }
}

fn build_request_json(id: u64, method: &str, params: serde_json::Value) -> serde_json::Value {
    let jsonrpc = "2.0";
    json!({
    "jsonrpc": jsonrpc,
    "id": id,
    "method": method,
    "params": params,
    })
}

pub(crate) async fn get_account_info(cluster: &Cluster, pubkey_str: &str) -> serde_json::Value {
    let client = reqwest::Client::new();
    let request_id: u64 = 0;
    let method = "getAccountInfo";
    let params = json!([pubkey_str,{
    "encoding": "base64"
    }]);
    let request_json = build_request_json(request_id, method, params).to_string();

    let response = client
        .post(get_cluster_api_url(&cluster))
        .header("Content-Type", "application/json")
        .body(request_json)
        .send()
        .await
        .unwrap();

    response.json().await.unwrap()
}

pub(crate) async fn get_account_data(cluster: &Cluster, pubkey: &Pubkey) -> Vec<u8> {
    let map_data = get_account_info(&cluster, &pubkey.to_string()).await;
    let result = &map_data["result"]["value"]["data"][0];
    let foo_str = result.as_str().unwrap();
    let b64 = base64::decode(foo_str);

    b64.unwrap()
}

#[cfg(test)]
#[tokio::test]
async fn test_get_account_info() {
    let cluster = Cluster::Devnet;
    let pubkey = "testbF8AJQDHg7ZqEZejXJTXrjff4516AHD48xTPnBK";
    let account_info = get_account_info(&cluster, pubkey).await;
    let result = &account_info["result"];

    println!("result: {:?}", result);
    assert_ne!(Some(result), None);
}
