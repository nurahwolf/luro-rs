use tracing::error;
use twilight_util::builder::embed::EmbedBuilder;

use crate::{interactions::InteractionResponse, models::LuroResponse, COLOUR_DANGER};

/// Unknown command received
fn unknown_command_embed() -> EmbedBuilder {
    error!("Unknown command received, most likely its not registered in the event handler");
    EmbedBuilder::new()
        .title("Unknown Command Received")
        .color(COLOUR_DANGER)
        .description("We had a fucky wucky!")
}

pub fn unknown_command_response(luro_response: LuroResponse) -> InteractionResponse {
    InteractionResponse::Embed {
        embeds: vec![unknown_command_embed().build()],
        luro_response
    }
}
