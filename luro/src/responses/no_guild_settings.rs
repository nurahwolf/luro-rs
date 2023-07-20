use tracing::error;
use twilight_util::builder::embed::{EmbedBuilder, EmbedFooterBuilder};

use crate::{interactions::InteractionResponse, COLOUR_DANGER};

/// Returns an embed containing a standardised error message that we were unable to get the channel that an interaction took place in.
pub fn embed() -> EmbedBuilder {
    error!("No guild settings present for guild");
    EmbedBuilder::new()
        .title("No Guild Settings")
        .color(COLOUR_DANGER)
        .description("Looks like this guild has not been enrolled into my guild settings. That's not ideal. Try running the command again?")
        .footer(EmbedFooterBuilder::new(
            "Okay, Houston, I believe we've had a problem here ...",
        ))
}

pub fn no_guild_settings(ephemeral: bool, deferred: bool) -> InteractionResponse {
    InteractionResponse::Embed {
        embeds: vec![embed().build()],
        ephemeral,
        deferred
    }
}
