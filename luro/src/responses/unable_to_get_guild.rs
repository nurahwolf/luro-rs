use twilight_util::builder::embed::EmbedBuilder;

use crate::{interactions::InteractionResponse, COLOUR_DANGER};

/// Unable to fetch the guild
pub fn unable_to_get_guild(reason: String) -> InteractionResponse {
    let embed = EmbedBuilder::new()
        .color(COLOUR_DANGER)
        .description(format!("Can't fetch information for the guild you are in, sorry. Most likely the Discord API is having a certified `fucky wucky` moment.\n\n**Reason:**\n```{}```", reason))
        .build();

    InteractionResponse::Embed {
        embeds: vec![embed],
        ephemeral: true
    }
}
