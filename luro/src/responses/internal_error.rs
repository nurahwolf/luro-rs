use tracing::error;
use twilight_util::builder::embed::{EmbedBuilder, EmbedFooterBuilder};

use crate::{interactions::InteractionResponse, COLOUR_DANGER};

/// Internal error embed
pub fn internal_error(error: String) -> InteractionResponse {
    error!("Generic error raised");
    let embed = EmbedBuilder::new()
        .title("It's fucked")
        .color(COLOUR_DANGER)
        .description(error)
        .footer(EmbedFooterBuilder::new(
            "Okay, Houston, I believe we've had a problem here ..."
        ))
        .build();

    InteractionResponse::Embed {
        embeds: vec![embed],
        ephemeral: true,
        deferred: true
    }
}
