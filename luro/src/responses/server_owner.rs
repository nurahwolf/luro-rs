use twilight_util::builder::embed::EmbedBuilder;

use crate::{interactions::InteractionResponse, COLOUR_DANGER};

/// Member is the guild owner...
pub fn server_owner() -> InteractionResponse {
    let embed = EmbedBuilder::new()
        .color(COLOUR_DANGER)
        .title("That's the server owner you idiot")
        .description("Congratulations moron, that's the server owner. Do you really think I'm gonna try to kick OR ban them? Holy shit, no.")
        .build();

    InteractionResponse::Embed {
        embeds: vec![embed],
        ephemeral: true,
        deferred: true
    }
}
