use tracing::warn;

use twilight_util::builder::embed::EmbedBuilder;

use crate::COLOUR_DANGER;

use crate::framework::LuroFramework;
use crate::models::LuroResponse;

impl LuroFramework {
    pub async fn bot_hierarchy_response(&self, bot_username: &String, slash: &mut LuroResponse) -> anyhow::Result<()> {
        slash.embed(bot_hierarchy_embed(bot_username).build())?;
        self.respond(slash).await
    }
}

/// An embed returned if the user is above the bot in the role hierarchy.
fn bot_hierarchy_embed(bot_username: &String) -> EmbedBuilder {
    warn!("User tried to execute a command in which the bot is too low to function");
    EmbedBuilder::new()
        .color(COLOUR_DANGER)
        .title("Role Hierarchy Error")
        .description(format!("This member has a role above or equivalent to that of {bot_username} in the list of roles, which prevents moderation actions from being performed on them. You can correct this by placing me higher in the list of roles.\nHowever, if you are trying to action someone higher than me on purpose... ***Get fucked ;)***"))
}
