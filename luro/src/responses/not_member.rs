use twilight_util::builder::embed::EmbedBuilder;

use crate::{interactions::InteractionResponse, COLOUR_DANGER};

/// User is not a server member.
pub fn not_member(username: String) -> InteractionResponse {
    let embed = EmbedBuilder::new()
        .color(COLOUR_DANGER)
        .description(format!("I'm afraid {username} is no longer a member of the server."))
        .build();

    InteractionResponse::Embed {
        embeds: vec![embed],
        ephemeral: true,
        deferred: true
    }
}
