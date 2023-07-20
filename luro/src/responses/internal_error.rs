use twilight_util::builder::embed::{EmbedBuilder, EmbedFooterBuilder};

use crate::{interactions::InteractionResponse, COLOUR_DANGER};

pub fn embed(error: String) -> EmbedBuilder {
    EmbedBuilder::new()
        .title("It's fucked")
        .color(COLOUR_DANGER)
        .description(error)
        .footer(EmbedFooterBuilder::new(
            "Okay, Houston, I believe we've had a problem here ..."
        ))
}

/// Internal error embed
pub fn internal_error(error: String, ephemeral: bool, deferred: bool) -> InteractionResponse {
    InteractionResponse::Embed {
        embeds: vec![embed(error).build()],
        ephemeral,
        deferred
    }
}
