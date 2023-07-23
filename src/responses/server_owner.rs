use tracing::warn;
use twilight_util::builder::embed::EmbedBuilder;

use crate::{interactions::InteractionResponse, models::LuroResponse, COLOUR_DANGER};

/// Member is the guild owner...
fn server_owner_embed() -> EmbedBuilder {
    warn!("Someone tried to fuck with the server owner using the bot");
    EmbedBuilder::new()
        .color(COLOUR_DANGER)
        .title("That's the server owner you idiot")
        .description("Congratulations moron, that's the server owner. Do you really think I'm gonna try to kick OR ban them? Holy shit, no.")
}

pub fn server_owner_response(luro_response: LuroResponse) -> InteractionResponse {
    InteractionResponse::Embed {
        embeds: vec![server_owner_embed().build()],
        luro_response
    }
}
