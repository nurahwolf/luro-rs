use luro_model::{COLOUR_DANGER, builders::EmbedBuilder};
use tracing::warn;

/// Returns an embed containing a standardised error message that we were unable to get the channel that an interaction took place in.
pub fn user_hierarchy_embed(username: &str) -> EmbedBuilder {
    warn!("The user {username} tried to abuse the bot's perms to do something they can't do");
    let mut embed = EmbedBuilder::default();
    embed.colour(COLOUR_DANGER)
        .title("You don't have permission to modify this member")
        .description(format!("Well done you fucking moron, {username} is above you in the role hierarchy. That means you can't do shit to them, and no I'm not going to bypass your position because you bitch and whine.\n***Get Bent.***"));
    embed
}
