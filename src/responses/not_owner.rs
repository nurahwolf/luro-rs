use crate::traits::toml::LuroTOML;
use crate::USERDATA_FILE_PATH;
use std::path::Path;

use anyhow::Context;
use tracing::warn;
use twilight_model::id::marker::{GuildMarker, UserMarker};
use twilight_model::id::Id;
use twilight_util::builder::embed::{EmbedBuilder, EmbedFooterBuilder};

use crate::COLOUR_DANGER;

use crate::models::{LuroSlash, UserActionType, UserActions, UserData};

impl LuroSlash {
    pub async fn not_owner_response(
        mut self,
        user_id: &Id<UserMarker>,
        guild_id: &Option<Id<GuildMarker>>,
        command_name: impl Into<String>
    ) -> anyhow::Result<()> {
        let command = command_name.into();
        {
            let _ = UserData::get_user_settings(&self.luro, user_id).await?;
            let path = format!("{0}/{1}/user_settings.toml", USERDATA_FILE_PATH, &user_id);
            let data = &mut self
                .luro
                .user_data
                .get_mut(user_id)
                .context("Expected to find user's data in the cache")?;
            data.moderation_actions.push(UserActions {
                action_type: vec![UserActionType::Kick],
                guild_id: *guild_id,
                reason: format!("Attempted to run the {} command", &command),
                responsible_user: *user_id
            });
            data.write(Path::new(&path)).await?;
        }
        self.embed(not_owner_embed(user_id, &command).build())?.respond().await
    }
}

/// Returns an embed containing a standardised error message that we were unable to get the channel that an interaction took place in.
fn not_owner_embed(user_id: &Id<UserMarker>, command_name: &String) -> EmbedBuilder {
    warn!("User {user_id} attempted to run the command {command_name} without being the bot owner...");
    EmbedBuilder::new()
    .title("You are not the bot owner!")
    .color(COLOUR_DANGER)
    .description("Great job motherfucker, you are not the bot owner and do not have permission to use that command.\n\n**THE COMMAND IS LITERALLY NAMED OWNER ONLY! WHAT THE HECK DID YOU THINK WOULD HAPPEN!?**")
    .footer(EmbedFooterBuilder::new("FYI, I'm reporting you to Nurah."))
}
