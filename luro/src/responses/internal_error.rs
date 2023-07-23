use twilight_util::builder::embed::{EmbedBuilder, EmbedFooterBuilder};

use crate::{interactions::InteractionResponse, models::LuroResponse, COLOUR_DANGER};

fn internal_error_embed(error: &String) -> EmbedBuilder {
    EmbedBuilder::new()
        .title("It's fucked")
        .color(COLOUR_DANGER)
        .description(format!("```{error}```"))
        .footer(EmbedFooterBuilder::new(
            "Okay, Houston, I believe we've had a problem here ..."
        ))
}

/// Internal error embed
pub fn internal_error_response(error: &String, response: LuroResponse) -> InteractionResponse {
    InteractionResponse::Embed {
        embeds: vec![internal_error_embed(error).build()],
        luro_response: response
    }
}
