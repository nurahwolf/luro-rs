use tracing::warn;
use twilight_util::builder::embed::EmbedBuilder;

use crate::COLOUR_DANGER;

use crate::interaction::LuroSlash;

impl LuroSlash {
    pub async fn user_hierarchy_response(&self, username: &String) -> anyhow::Result<()> {
        self.respond(|r| r.add_embed(user_hierarchy_embed(username).build())).await
    }
}

/// Returns an embed containing a standardised error message that we were unable to get the channel that an interaction took place in.
fn user_hierarchy_embed(username: &String) -> EmbedBuilder {
    warn!("The user {username} tried to abuse the bot's perms to do something they can't do");
    EmbedBuilder::new()
        .color(COLOUR_DANGER)
        .title("You don't have permission to modify this member")
        .description(format!("Well done you fucking moron, {username} is above you in the role hierarchy. That means you can't do shit to them, and no I'm not going to bypass your position because you bitch and whine.\n***Get Bent.***"))
}
