use anyhow::Context;
use luro_framework::{CommandInteraction, Luro, LuroCommand};
use luro_model::{response::SimpleResponse, types::PunishmentType};
use twilight_interactions::command::{CommandModel, CommandOption, CreateCommand, CreateOption, ResolvedUser};

use crate::commands::moderator::{reason, Reason};

#[derive(CommandModel, CreateCommand, Clone, Debug, PartialEq, Eq)]
#[command(name = "fakeban", desc = "Ban a user", dm_permission = false)]
pub struct FakeBan {
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

impl LuroCommand for FakeBan {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<luro_model::types::CommandResponse> {
        let guild = ctx.guild.as_ref().context("Expected guild")?;
        let punished_user = ctx.fetch_user(self.user.resolved.id).await?;
        let reason = reason(self.reason, self.details);

        // Checks passed, now let's action the user
        let embed = SimpleResponse::Punishment(
            guild,
            PunishmentType::Banned(reason.as_deref(), self.purge.value()),
            &ctx.author,
            &punished_user,
        );

        let punished_user_dm = match ctx.twilight_client.create_private_channel(punished_user.user_id).await {
            Ok(channel) => channel.model().await?,
            Err(_) => return ctx.respond(|r| r.content("Could not create DM with the user!")).await,
        };

        let victim_dm = ctx
            .twilight_client
            .create_message(punished_user_dm.id)
            .embeds(&[embed.embed().0])
            .await;

        let embed = match victim_dm {
            Ok(_) => embed.dm_sent(true),
            Err(_) => embed.dm_sent(false),
        };

        ctx.respond(|r| r.add_embed(embed)).await
    }
}
