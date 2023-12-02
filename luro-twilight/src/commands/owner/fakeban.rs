use anyhow::Context;
use luro_framework::{CommandInteraction, Luro, LuroCommand};
use luro_model::response::{BannedResponse, SimpleResponse};
use twilight_interactions::command::{CommandModel, CommandOption, CreateCommand, CreateOption, ResolvedUser};

use crate::commands::moderator::{reason, Reason};

#[derive(CommandModel, CreateCommand, Clone, Debug, PartialEq, Eq)]
#[command(name = "ban", desc = "Ban a user (not really)", dm_permission = false)]
pub struct Ban {
    /// The user to ban
    pub user: ResolvedUser,
    /// Message history to purge in seconds. Defaults to 1 day. Max is 604800.
    pub purge: TimeToBan,
    /// The reason they should be banned.
    pub reason: Reason,
    /// Some added description to why they should be banned
    pub details: Option<String>,
}

#[derive(CommandOption, CreateOption, Clone, Debug, PartialEq, Eq)]
pub enum TimeToBan {
    #[option(name = "Don't Delete Any", value = 0)]
    None,
    #[option(name = "Previous Hour", value = 3_600)]
    Hour,
    #[option(name = "Previous 6 Hours", value = 21_600)]
    SixHours,
    #[option(name = "Previous 12 Hours", value = 43_200)]
    TwelveHours,
    #[option(name = "Previous 24 Hours", value = 86_400)]
    TwentyFourHours,
    #[option(name = "Previous 3 Days", value = 259_200)]
    ThreeDays,
    #[option(name = "Previous 7 Days", value = 604_800)]
    SevenDays,
}

impl LuroCommand for Ban {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<luro_model::types::CommandResponse> {
        let guild = ctx.guild.as_ref().context("Expected guild")?;
        let target = ctx.fetch_user(self.user.resolved.id).await?;
        let reason = reason(self.reason, self.details);

        let target_dm = match ctx.twilight_client.create_private_channel(target.user_id).await {
            Ok(channel) => channel.model().await?,
            Err(_) => return ctx.respond(|r| r.content("Could not create DM with the user!")).await,
        };

        let victim_dm = ctx
            .twilight_client
            .create_message(target_dm.id)
            .embeds(&[SimpleResponse::BannedUserResponse(
                BannedResponse {
                    target: &target,
                    moderator: &ctx.author,
                    reason: reason.as_deref(),
                    purged_messages: self.purge.value(),
                },
                &guild.name,
            )
            .embed()])
            .await;

        let embed = match victim_dm {
            Ok(_) => SimpleResponse::BannedModeratorResponse(
                BannedResponse {
                    target: &target,
                    moderator: &ctx.author,
                    reason: reason.as_deref(),
                    purged_messages: self.purge.value(),
                },
                true,
            )
            .embed(),
            Err(_) => SimpleResponse::BannedModeratorResponse(
                BannedResponse {
                    target: &target,
                    moderator: &ctx.author,
                    reason: reason.as_deref(),
                    purged_messages: self.purge.value(),
                },
                false,
            )
            .embed(),
        };

        ctx.respond(|r| r.add_embed(embed)).await
    }
}
