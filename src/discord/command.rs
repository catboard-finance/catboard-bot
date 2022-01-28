mod hello;
mod price;

use crate::{
    cloudflare::worker::WorkersKv,
    discord::interaction::{
        ApplicationCommandInteractionData, InteractionResponse, InteractionResponseType,
    },
};

pub(crate) async fn handle_command(
    data: &ApplicationCommandInteractionData,
    kv: &WorkersKv,
) -> InteractionResponse {
    match data.name.as_str() {
        "hello" => hello::hello().await,
        "price" => {
            // Params?
            let options = data.options.as_ref().unwrap();
            let params = options[0].value.as_str();

            price::price(kv, params).await
        }
        _ => InteractionResponse {
            ty: InteractionResponseType::ACKWithSource,
            data: None,
        },
    }
}
