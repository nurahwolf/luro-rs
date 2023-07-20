use tracing::error;
use twilight_util::builder::embed::EmbedBuilder;

use crate::{interactions::InteractionResponse, COLOUR_DANGER};

/// Unknown command received
pub fn unknown_command() -> InteractionResponse {
    error!("Unknown command received, most likely its not registered in the event handler");
    let embed = EmbedBuilder::new()
        .title("Unknown Command Received")
        .color(COLOUR_DANGER)
        .description("We had a fucky wucky!")
        .build();

    InteractionResponse::Embed {
        embeds: vec![embed],
        ephemeral: true,
        deferred: true
    }
}
