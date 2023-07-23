use tracing::warn;
use twilight_util::builder::embed::EmbedBuilder;

use crate::{interactions::InteractionResponse, models::LuroResponse, COLOUR_DANGER};

/// User is not a server member.
fn not_member_embed(username: &String) -> EmbedBuilder {
    warn!("User is no longer a member of the server");
    EmbedBuilder::new()
        .color(COLOUR_DANGER)
        .description(format!("I'm afraid {username} is no longer a member of the server."))
}

pub fn not_member_response(username: &String, luro_response: LuroResponse) -> InteractionResponse {
    InteractionResponse::Embed {
        embeds: vec![not_member_embed(username).build()],
        luro_response
    }
}
