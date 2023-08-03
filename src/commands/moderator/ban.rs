use std::convert::TryInto;

use async_trait::async_trait;
use tracing::debug;

use twilight_http::request::AuditLogReason;
use twilight_interactions::command::{CommandModel, CommandOption, CreateCommand, CreateOption, ResolvedUser};
use twilight_model::guild::Permissions;
use twilight_util::builder::embed::EmbedFieldBuilder;

use crate::{
    models::{GuildPermissions, LuroSlash},
    traits::luro_command::LuroCommand
};

#[derive(CommandModel, CreateCommand, Clone, Debug, PartialEq, Eq)]
#[command(name = "ban", desc = "Ban a user", dm_permission = false)]
pub struct BanCommand {
    /// The user to ban
    pub user: ResolvedUser,
    /// Message history to purge in seconds. Defaults to 1 day. Max is 604800.
    pub purge: TimeToBan,
    /// The reason they should be banned
    pub reason: Option<String>
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
    SevenDays
}

#[async_trait]
impl LuroCommand for BanCommand {
    async fn run_command(self, mut ctx: LuroSlash) -> anyhow::Result<()> {
        ctx.deferred().await?;

        let reason = match self.reason {
            Some(reason) => reason,
            None => String::new()
        };
        let period_string = match self.purge {
            TimeToBan::None => "Don't Delete Any".to_string(),
            TimeToBan::Hour => "Previous Hour".to_string(),
            TimeToBan::SixHours => "Previous 6 Hours".to_string(),
            TimeToBan::TwelveHours => "Previous 12 Hours".to_string(),
            TimeToBan::TwentyFourHours => "Previous 24 Hours".to_string(),
            TimeToBan::ThreeDays => "Previous 3 Days".to_string(),
            TimeToBan::SevenDays => "Previous 7 Days".to_string()
        };
        // Fetch the author and the bot permissions.
        let guild_id = match ctx.interaction.guild_id {
            Some(guild_id) => guild_id,
            None => return ctx.not_guild_response().await
        };
        let guild = ctx.luro.twilight_client.guild(guild_id).await?.model().await?;
        let author_user = match &ctx.interaction.member {
            Some(member) => match &member.user {
                Some(user) => user,
                None => return ctx.not_guild_response().await
            },
            None => return ctx.not_guild_response().await
        };
        let author = ctx
            .luro
            .twilight_client
            .guild_member(guild_id, author_user.id)
            .await?
            .model()
            .await?;
        debug!("Getting permissions of the guild");
        let permissions = GuildPermissions::new(&ctx.luro.twilight_client, &guild_id).await?;
        debug!("Getting author permissions");
        let author_permissions = permissions.member(author.user.id, &author.roles).await?;
        let user_to_remove = self.user.resolved;
        debug!("Getting bot permissions");
        let bot_permissions = permissions.current_member().await?;

        if !bot_permissions.guild().contains(Permissions::BAN_MEMBERS) {
            return ctx.bot_missing_permission_response(&"BAN_MEMBERS".to_owned()).await;
        }

        if let Some(member_to_remove) = self.user.member {
            // The user is a member of the server, so carry out some additional checks.
            debug!("Getting banned user's permissions");
            let member_permissions = permissions.member(user_to_remove.id, &member_to_remove.roles).await?;

            // Check if the author and the bot have required permissions.
            if member_permissions.is_owner() {
                return ctx.server_owner_response().await;
            }

            // Check if the role hierarchy allow the author and the bot to perform
            // the ban.
            let member_highest_role = member_permissions.highest_role();

            if member_highest_role >= author_permissions.highest_role() {
                return ctx
                    .user_hierarchy_response(&member_to_remove.nick.unwrap_or(user_to_remove.name.to_owned()))
                    .await;
            }

            if member_highest_role >= bot_permissions.highest_role() {
                let global_data = ctx.luro.global_data.read().clone();
                return ctx.bot_hierarchy_response(&global_data.current_user.name).await;
            }
        };

        // Checks passed, now let's action the user
        let user_to_ban_dm = match ctx.luro.twilight_client.create_private_channel(user_to_remove.id).await {
            Ok(channel) => channel.model().await?,
            Err(_) => {
                return ctx
                    .ban_response(guild, author, user_to_remove, &reason, &period_string, false)
                    .await
            }
        };

        let mut embed = ctx
            .ban_embed(guild.clone(), author.clone(), user_to_remove.clone(), &reason, &period_string)
            .await?;

        let victim_dm = ctx
            .luro
            .twilight_client
            .create_message(user_to_ban_dm.id)
            .embeds(&[embed.clone().build()])?
            .await;

        match victim_dm {
            Ok(_) => embed = embed.field(EmbedFieldBuilder::new("DM Sent", "Successful").inline()),
            Err(_) => embed = embed.field(EmbedFieldBuilder::new("DM Sent", "Failed").inline())
        }

        let mut ban = ctx
            .luro
            .twilight_client
            .create_ban(guild_id, user_to_remove.id)
            .delete_message_seconds(self.purge.value().try_into()?)?;
        if !reason.is_empty() {
            ban = ban.reason(&reason)?
        }
        ban.await?;

        // If an alert channel is defined, send a message there
        ctx.luro
            .send_log_channel(&Some(guild_id), embed.clone(), crate::models::LuroLogChannel::Moderator)
            .await?;

        ctx.embed(embed.build())?.respond().await
    }
}

// pub async fn ban(
//     ctx: &LuroFramework,
//     interaction: &Interaction,
//     data: BanCommand,
// ) -> Result<(), Error> {
//     let embed;

//     if let Some(author) = &interaction.member && let Some(permissions) = &author.permissions && let Some(guild_id) = interaction.guild_id {
//         let author_user = &author.user.clone().unwrap();

//         if permissions.contains(Permissions::BAN_MEMBERS) || author_user.id != Id::new(97003404601094144) {
//             let user_avatar = match interaction.guild_id {
//                 Some(guild_id) => get_member_avatar(&data.user.member, &Some(guild_id), &data.user.resolved),
//                 None => get_user_avatar(&data.user.resolved),
//             };

//             let embed_author = EmbedAuthorBuilder::new(format!("{}#{} - {}", &data.user.resolved.name, &data.user.resolved.discriminator, &data.user.resolved.id)).icon_url(ImageSource::url(user_avatar)?).build();

//             embed = EmbedBuilder::default().description(format!("**BANNED!**\n Looks like <@{}> was banned! How unfortunate.", &data.user.resolved.id)).author(embed_author).footer(EmbedFooterBuilder::new(format!("Banned by {}#{}", author_user.name, author_user.discriminator))).field(EmbedFieldBuilder::new("Reason", data.reason)).color(ACCENT_COLOUR).build();

//             match data.purge {
//                 Some(seconds) => luro.twilight_client.create_ban(guild_id, data.user.resolved.id).delete_message_seconds(seconds.try_into().unwrap())?.await?,
//                 None => luro.twilight_client.create_ban(guild_id, data.user.resolved.id).delete_message_seconds(86400)?.await?,
//             };

//     let response = InteractionResponseDataBuilder::new().embeds(vec![embed]);
//     create_response(luro, interaction, response.build()).await?;

//     Ok(())
// }
