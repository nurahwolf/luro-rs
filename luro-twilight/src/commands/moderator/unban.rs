use anyhow::Context;
use luro_framework::{CommandInteraction, Luro, LuroCommand, PunishmentType, Response, StandardResponse};
use twilight_http::request::AuditLogReason;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    guild::Permissions,
    id::{marker::UserMarker, Id},
};

#[derive(CommandModel, CreateCommand, Clone, Debug, PartialEq, Eq)]
#[command(name = "unban", desc = "Unban a user", dm_permission = false)]
pub struct Unban {
    /// The user to ban
    pub user: Id<UserMarker>,
    /// The reason they should be unbanned.
    pub reason: String,
}

impl LuroCommand for Unban {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        let mut response = ctx.acknowledge_interaction(false).await?;
        let guild = ctx.guild.as_ref().context("Expected this to be a guild")?;
        let luro = ctx.database.user_fetch_current_user(ctx.guild_id()).await?;
        let moderator = &ctx.author;
        let target = ctx.fetch_user(self.user).await?;
        let luro_data = match &luro.member.as_ref().context("Expected member context")?.data {
            Some(data) => data,
            None => {
                return ctx
                    .respond(|r| {
                        r.content("Sorry, could not fetch my permissions to check if I can do this!")
                            .ephemeral()
                    })
                    .await
            }
        };
        let moderator_data = match &moderator.member.as_ref().context("Expected member context")?.data {
            Some(data) => data,
            None => {
                return ctx
                    .respond(|r| {
                        r.content("Sorry, could not fetch your permissions to check if you can do this!")
                            .ephemeral()
                    })
                    .await
            }
        };

        let luro_permissions = luro_data.permission_calculator(&luro_data.role_permissions()).root();
        let moderator_permissions = moderator_data.permission_calculator(&moderator_data.role_permissions()).root();

        if !luro_permissions.contains(Permissions::BAN_MEMBERS) {
            return ctx.response_simple(Response::BotMissingPermission(Permissions::BAN_MEMBERS)).await;
        }

        if !moderator_permissions.contains(Permissions::BAN_MEMBERS) {
            return ctx.response_simple(Response::MissingPermission(Permissions::BAN_MEMBERS)).await;
        }

        // Checks passed, now let's action the user
        let mut embed =
            StandardResponse::new_punishment(PunishmentType::Unbanned, &guild.name, &guild.guild_id, &target, &ctx.author.clone());
        embed.punishment_reason(Some(&self.reason), &target);
        match ctx.twilight_client.create_private_channel(target.user_id).await {
            Ok(channel) => {
                let victim_dm = ctx
                    .twilight_client
                    .create_message(channel.model().await?.id)
                    .embeds(&[embed.embed().0])
                    .await;

                match victim_dm {
                    Ok(_) => embed.dm_sent(true),
                    Err(_) => embed.dm_sent(false),
                }
            }
            Err(_) => embed.dm_sent(false),
        };

        let unban = ctx
            .twilight_client
            .delete_ban(guild.guild_id, target.user_id)
            .reason(&self.reason)
            .await;
        match unban {
            Ok(_) => embed.create_field("Unban", "Successful", true),
            Err(_) => embed.create_field("Unban", "Failed", true),
        };

        response.add_embed(embed.embed().0);
        ctx.response_send(response).await?;

        // moderator.moderation_actions_performed += 1;
        // ctx.database.modify_user(&moderator.id, &moderator).await?;

        // If an alert channel is defined, send a message there
        // ctx.send_log_channel(&guild.guild_id, LuroLogChannel::Moderator, |r| r.add_embed(embed.embed))
        //     .await?;

        Ok(())
    }
}
