use serde_json::json;

use crate::discord::interaction::{
    InteractionApplicationCommandCallbackData, InteractionResponse, InteractionResponseType,
};

pub(crate) async fn hello() -> InteractionResponse {
    let content = "hello there!".to_string();
    let embed = json!({
      "type": "rich",
      "title": "Solana",
      "description": "foo",
      "color": 0x8400ff,
    })
    .to_string();
    let embeds: Vec<String> = [embed].to_vec();

    InteractionResponse {
        ty: InteractionResponseType::ChannelMessageWithSource,
        data: Some(InteractionApplicationCommandCallbackData {
            content: content,
            embeds: Some(embeds),
        }),
    }
}
