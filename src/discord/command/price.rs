use crate::{
    catboard::consumer::{
        get_formatted_price_from_pyth, get_price_account_from_kv_by_symbol, get_supported_symbols,
    },
    cloudflare::worker::WorkersKv,
    discord::interaction::{
        InteractionApplicationCommandCallbackData, InteractionResponse, InteractionResponseType,
    },
    solana::web3::Cluster,
};

pub(crate) async fn price(kv: &WorkersKv, symbol: &str) -> InteractionResponse {
    // Get price_account from kv
    let price_account = get_price_account_from_kv_by_symbol(&kv, &symbol).await;
    let embeds = None;

    // Guard not support symbol
    match price_account.as_str() {
        "" => {
            let supported_symbols: Vec<String> = get_supported_symbols()
                .iter()
                .map(|e| format!("`{}`", e))
                .collect();
            let supported_symbols_str = supported_symbols.join(",");
            let content = format!(
                "Sorry, only {} are support at the moment.",
                supported_symbols_str
            );
            return InteractionResponse {
                ty: InteractionResponseType::ChannelMessageWithSource,
                data: Some(InteractionApplicationCommandCallbackData {
                    content: content,
                    embeds: embeds,
                }),
            };
        }
        _ => {}
    }

    // Get formatted price
    let content =
        get_formatted_price_from_pyth(&Cluster::Devnet, symbol, &price_account.as_str()).await;

    InteractionResponse {
        ty: InteractionResponseType::ChannelMessageWithSource,
        data: Some(InteractionApplicationCommandCallbackData {
            content: content,
            embeds: embeds,
        }),
    }
}
