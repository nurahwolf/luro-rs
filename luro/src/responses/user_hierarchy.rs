use tracing::warn;
use twilight_util::builder::embed::EmbedBuilder;

use crate::{interactions::InteractionResponse, models::LuroResponse, COLOUR_DANGER};

/// Author is below the user in the role hierarchy.
fn user_hierarchy_embed(username: &String) -> EmbedBuilder {
    warn!("The user {username} tried to abuse the bot's perms to do something they can't do");
    EmbedBuilder::new()
        .color(COLOUR_DANGER)
        .title("You don't have permission to modify this member")
        .description(format!("Well done you fucking moron, {username} is above you in the role hierarchy. That means you can't do shit to them, and no I'm not going to bypass your position because you bitch and whine.\n***Get Bent.***"))
}

pub fn user_hierarchy_response(username: &String, luro_response: LuroResponse) -> InteractionResponse {
    InteractionResponse::Embed {
        embeds: vec![user_hierarchy_embed(username).build()],
        luro_response
    }
}
