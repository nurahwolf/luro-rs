use luro_builder::embed::EmbedBuilder;
use luro_model::user_actions::UserActions;
use luro_model::user_actions_type::UserActionType;
use tracing::warn;
use twilight_model::id::marker::{GuildMarker, UserMarker};
use twilight_model::id::Id;

use crate::COLOUR_DANGER;

use crate::interaction::LuroSlash;

impl LuroSlash {
    pub async fn not_owner_response(
        &self,
        user_id: &Id<UserMarker>,
        guild_id: &Option<Id<GuildMarker>>,
        command_name: impl Into<String>
    ) -> anyhow::Result<()> {
        let command = command_name.into();
        {
            let mut user_data = self.framework.database.get_user(user_id).await?;
            user_data.moderation_actions.push(UserActions {
                action_type: vec![UserActionType::PrivilegeEscalation],
                guild_id: *guild_id,
                reason: format!("Attempted to run the {} command", &command),
                responsible_user: *user_id
            });
            self.framework.database.modify_user(user_id, &user_data).await?;
        }
        self.respond(|r| r.add_embed(not_owner_embed(user_id, &command))).await
    }
}

/// Returns an embed containing a standardised error message that we were unable to get the channel that an interaction took place in.
fn not_owner_embed(user_id: &Id<UserMarker>, command_name: &String) -> EmbedBuilder {
    warn!("User {user_id} attempted to run the command {command_name} without being the bot owner...");
    let mut embed = EmbedBuilder::default();
    embed.title("You are not the bot owner!")
    .colour(COLOUR_DANGER)
    .description("Great job motherfucker, you are not the bot owner and do not have permission to use that command.\n\n**THE COMMAND IS LITERALLY NAMED OWNER ONLY! WHAT THE HECK DID YOU THINK WOULD HAPPEN!?**")
    .footer(|f|f.text("FYI, I'm reporting you to Nurah."));
    embed
}
