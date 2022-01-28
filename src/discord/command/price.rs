use crate::{
    catboard::consumer::{get_formatted_price_from_pyth, get_price_account_from_kv_by_symbol},
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
            let content = format!("😱 Sorry! `{}` is not support at the moment.", symbol);
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
