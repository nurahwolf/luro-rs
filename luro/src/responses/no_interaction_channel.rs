use twilight_util::builder::embed::{EmbedBuilder, EmbedFooterBuilder};

use crate::{interactions::InteractionResponse, COLOUR_DANGER};

/// Returns an embed containing a standardised error message that we were unable to get the channel that an interaction took place in.
pub fn embed() -> EmbedBuilder {
    EmbedBuilder::new()
        .title("Unable to get interaction channel")
        .color(COLOUR_DANGER)
        .description("I'm afraid I was unable to work out what channel this command was ran in. Try again as it might be the API that's having a moment.")
        .footer(EmbedFooterBuilder::new(
            "Okay, Houston, I believe we've had a problem here ...",
        ))
}

pub fn no_interaction_channel() -> InteractionResponse {
    InteractionResponse::Embed {
        embeds: vec![embed().build()],
        ephemeral: true,
        deferred: true
    }
}
