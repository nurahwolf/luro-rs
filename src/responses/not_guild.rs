use tracing::warn;
use twilight_util::builder::embed::EmbedBuilder;

use crate::{interactions::InteractionResponse, models::LuroResponse, COLOUR_DANGER};

/// Returns an embed containing a standardised error message that we were unable to get the channel that an interaction took place in.
pub fn embed() -> EmbedBuilder {
    warn!("Guild Command was ran but I could not work out what guild they were in");
    EmbedBuilder::new()
        .title("Unable to find this guild!")
        .color(COLOUR_DANGER)
        .description("If you ran this command in a guild, I am unable to find it. If this is a DM... Please tell Nurah to limit this command to guilds only.")
}

pub fn not_guild_response(luro_response: LuroResponse) -> InteractionResponse {
    InteractionResponse::Embed {
        embeds: vec![embed().build()],
        luro_response
    }
}
