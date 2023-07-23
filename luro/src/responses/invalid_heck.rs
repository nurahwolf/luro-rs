use tracing::warn;
use twilight_util::builder::embed::{EmbedBuilder, EmbedFieldBuilder};

use crate::{interactions::InteractionResponse, models::LuroResponse, COLOUR_DANGER};

/// Invalid heck embed
fn invalid_heck_embed(heck_message: &str) -> EmbedBuilder {
    warn!("User attempted to make an invalid heck");
    EmbedBuilder::new()
        .color(COLOUR_DANGER)
        .title("Can you read?")
        .description(format!("You need to include both `<user>` and `<author>` in your message at least once! For example:\n```<author> exploded <user>!```\nYour heck was:\n```{}```", heck_message))
}

/// Repond with an invalid heck error
pub fn invalid_heck_response(
    missing_user: bool,
    missing_author: bool,
    heck_message: &str,
    response: LuroResponse
) -> InteractionResponse {
    let mut embed = invalid_heck_embed(heck_message);

    if missing_user {
        embed = embed.field(EmbedFieldBuilder::new("Missing Value", "`<user>`").inline())
    };
    if missing_author {
        embed = embed.field(EmbedFieldBuilder::new("Missing Value", "`<author>`").inline())
    };

    InteractionResponse::Embed {
        embeds: vec![embed.build()],
        luro_response: response
    }
}
