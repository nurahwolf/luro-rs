use tracing::warn;
use twilight_util::builder::embed::EmbedBuilder;

use crate::{interactions::InteractionResponse, models::LuroResponse, COLOUR_DANGER};

/// Unable to fetch the guild
fn unable_to_get_guild_embed(reason: &String) -> EmbedBuilder {
    warn!("Unable to get guild data");
    EmbedBuilder::new()
        .color(COLOUR_DANGER)
        .description(format!("Can't fetch information for the guild you are in, sorry. Most likely the Discord API is having a certified `fucky wucky` moment.\n\n**Reason:**\n```{}```", reason))
}

pub fn unable_to_get_guild_response(reason: &String, luro_response: LuroResponse) -> InteractionResponse {
    InteractionResponse::Embed {
        embeds: vec![unable_to_get_guild_embed(reason).build()],
        luro_response
    }
}
