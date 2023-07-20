use tracing::warn;
use twilight_util::builder::embed::EmbedBuilder;

use crate::{interactions::InteractionResponse, COLOUR_DANGER};

/// Author is below the user in the role hierarchy.
pub fn user_hierarchy(username: String) -> InteractionResponse {
    warn!("A moderator tried to abuse the bot's perms to do something they can't do");
    let embed = EmbedBuilder::new()
        .color(COLOUR_DANGER)
        .title("You don't have permission to modify this member")
        .description(format!("Well done you fucking moron, {username} is above you in the role hierarchy. That means you can't do shit to them, and no I'm not going to bypass your position because you bitch and whine.\n***Get Bent.***"))
        .build();

    InteractionResponse::Embed {
        embeds: vec![embed],
        ephemeral: true,
        deferred: true
    }
}
