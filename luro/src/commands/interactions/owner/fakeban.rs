use luro_model::response::{Punishment, PunishmentData};
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::id::{marker::UserMarker, Id};

use crate::{
    commands::interactions::{PunishmentPurgeAmount, PunishmentReason},
    models::interaction::{InteractionContext, InteractionResult},
};

#[derive(CommandModel, CreateCommand)]
#[command(name = "fakeban", desc = "Ban a user (not really)", dm_permission = false)]
pub struct Fakeban {
    /// The user to ban
    pub user_id: Id<UserMarker>,
    /// Message history to purge in seconds. Defaults to 1 day. Max is 604800.
    pub purge: PunishmentPurgeAmount,
    /// The reason they should be banned.
    pub reason: PunishmentReason,
    /// Some added description to why they should be banned
    pub details: Option<String>,
}

impl crate::models::CreateCommand for Fakeban {
    async fn handle_command(self, framework: &mut InteractionContext) -> InteractionResult<()> {
        framework.ack_interaction(false).await?;

        let twilight_client = &framework.gateway.twilight_client;
        let guild = framework.guild().await?;
        let author = framework.author_member(guild.twilight_guild.id).await?;
        let target = framework.fetch_user(self.user_id).await?;

        let reason = self.reason.fmt(self.details);
        let mut punishment = Punishment::Banned(
            PunishmentData {
                author: &author,
                target: &target,
                reason: &reason,
                guild: &guild,
                dm_successful: None,
            },
            self.purge.value(),
        );

        let target_dm = twilight_client.create_private_channel(target.user_id()).await;
        match target_dm {
            Ok(channel) => {
                let channel_id = channel.model().await?.id;
                let success = twilight_client
                    .create_message(channel_id)
                    .embeds(&[punishment.embed().into()])
                    .await;
                punishment.data().dm_successful = Some(success.is_ok())
            }
            Err(_) => punishment.data().dm_successful = Some(false),
        }

        framework.respond(|r| r.add_embed(punishment.embed())).await
    }
}
