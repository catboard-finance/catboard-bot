use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::cloudflare::worker::WorkersKv;
use crate::discord::command::handle_command;
use crate::error::Error;

#[derive(Deserialize_repr)]
#[repr(u8)]
enum InteractionType {
    Ping = 1,
    ApplicationCommand = 2,
}

#[allow(dead_code)]
#[derive(Serialize_repr)]
#[repr(u8)]
pub(crate) enum InteractionResponseType {
    Pong = 1,
    Acknowledge = 2,
    ChannelMessage = 3,
    ChannelMessageWithSource = 4,
    ACKWithSource = 5,
}

#[derive(Deserialize)]
pub(crate) struct ApplicationCommandInteractionDataOptions {
    pub(crate) value: String,
}

#[derive(Deserialize)]
pub(crate) struct ApplicationCommandInteractionData {
    pub(crate) name: String,
    pub(crate) options: Option<Vec<ApplicationCommandInteractionDataOptions>>,
}

#[derive(Serialize)]
pub(crate) struct InteractionApplicationCommandCallbackData {
    pub(crate) content: String,
    pub(crate) embeds: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub(crate) struct Interaction {
    #[serde(rename = "type")]
    ty: InteractionType,
    data: Option<ApplicationCommandInteractionData>,
}

impl Interaction {
    fn data(&self) -> Result<&ApplicationCommandInteractionData, Error> {
        Ok(self
            .data
            .as_ref()
            .ok_or_else(|| Error::InvalidPayload("data not found".to_string()))?)
    }
}

#[derive(Serialize)]
pub(crate) struct InteractionResponse {
    #[serde(rename = "type")]
    pub(crate) ty: InteractionResponseType,
    pub(crate) data: Option<InteractionApplicationCommandCallbackData>,
}

impl Interaction {
    pub(crate) async fn perform(&self, kv: &WorkersKv) -> Result<InteractionResponse, Error> {
        Ok(match self.ty {
            InteractionType::Ping => InteractionResponse {
                ty: InteractionResponseType::Pong,
                data: None,
            },
            InteractionType::ApplicationCommand => handle_command(self.data()?, kv).await,
        })
    }
}
