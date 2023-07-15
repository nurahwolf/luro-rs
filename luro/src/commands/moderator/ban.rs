use twilight_interactions::command::{
    CommandModel, CommandOption, CreateCommand, CreateOption, ResolvedUser,
};
use twilight_model::{application::interaction::Interaction, guild::Permissions};

use crate::{
    framework::LuroFramework,
    interactions::InteractionResponse,
    permissions::GuildPermissions,
    responses::embeds::{
        ban::{embed, interaction_response},
        bot_hierarchy::bot_hierarchy,
        bot_missing_permissions::bot_missing_permission,
        not_member::not_member,
        server_owner::server_owner,
        unable_to_get_guild::unable_to_get_guild,
        user_hierarchy::user_hierarchy,
    },
};

#[derive(CommandModel, CreateCommand, Clone, Debug, PartialEq, Eq)]
#[command(
    name = "ban",
    desc = "Ban a user",
    dm_permission = false,
    default_permissions = "BanCommand::default_permissions"
)]
pub struct BanCommand {
    /// The user to ban
    pub user: ResolvedUser,
    /// Message history to purge in seconds. Defaults to 1 day. Max is 604800.
    pub purge: TimeToBan,
    /// The reason they should be banned
    pub reason: Option<String>,
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

impl BanCommand {
    fn default_permissions() -> Permissions {
        Permissions::BAN_MEMBERS
    }

    pub async fn run(
        self,
        ctx: &LuroFramework,
        interaction: Interaction,
    ) -> Result<InteractionResponse, anyhow::Error> {
        let reason = match self.reason {
            Some(reason) => reason,
            None => String::new(),
        };
        let period_string = match self.purge {
            TimeToBan::None => "Don't Delete Any".to_string(),
            TimeToBan::Hour => "Previous Hour".to_string(),
            TimeToBan::SixHours => "Previous 6 Hours".to_string(),
            TimeToBan::TwelveHours => "Previous 12 Hours".to_string(),
            TimeToBan::TwentyFourHours => "Previous 24 Hours".to_string(),
            TimeToBan::ThreeDays => "Previous 3 Days".to_string(),
            TimeToBan::SevenDays => "Previous 7 Days".to_string(),
        };
        let user_to_remove = self.user.resolved;
        let member_to_remove = match self.user.member {
            Some(member) => member,
            None => return Ok(not_member(user_to_remove.name)),
        };

        // Fetch the author and the bot permissions.
        let guild_id = match interaction.guild_id {
            Some(guild_id) => guild_id,
            None => return Ok(unable_to_get_guild("Failed to get guild ID".to_string())),
        };
        let guild = ctx.twilight_client.guild(guild_id).await?.model().await?;
        let author_user = match interaction.member {
            Some(member) => match member.user {
                Some(user) => user,
                None => return Ok(unable_to_get_guild("Failed to get author user".to_string())),
            },
            None => {
                return Ok(unable_to_get_guild(
                    "Failed to get author member".to_string(),
                ))
            }
        };
        let author = ctx
            .twilight_client
            .guild_member(guild_id, author_user.id)
            .await?
            .model()
            .await?;
        let permissions = GuildPermissions::new(&ctx.twilight_client, &guild_id).await?;
        let author_permissions = permissions.member(author.user.id, &author.roles).await?;
        let member_permissions = permissions
            .member(user_to_remove.id, &member_to_remove.roles)
            .await?;
        let bot_permissions = permissions.current_member().await?;

        // Check if the author and the bot have required permissions.
        if member_permissions.is_owner() {
            return Ok(server_owner());
        }

        if !bot_permissions.guild().contains(Permissions::BAN_MEMBERS) {
            return Ok(bot_missing_permission("BAN_MEMBERS".to_string()));
        }

        // Check if the role hierarchy allow the author and the bot to perform
        // the ban.
        let member_highest_role = member_permissions.highest_role();

        if member_highest_role >= author_permissions.highest_role() {
            return Ok(user_hierarchy(
                member_to_remove
                    .nick
                    .unwrap_or(user_to_remove.name.to_string()),
            ));
        }

        if member_highest_role >= bot_permissions.highest_role() {
            return Ok(bot_hierarchy(&ctx.application.name));
        }

        // Checks passed, now let's action the user
        let user_to_ban_dm = match ctx
            .twilight_client
            .create_private_channel(user_to_remove.id)
            .await
        {
            Ok(channel) => channel.model().await?,
            Err(_) => {
                return interaction_response(
                    guild,
                    author,
                    user_to_remove,
                    guild_id,
                    &reason,
                    &period_string,
                    false,
                )
            }
        };

        let embeds = [embed(
            guild.clone(),
            author.clone(),
            user_to_remove.clone(),
            guild_id,
            &reason,
            &period_string,
        )?
        .build()];
        match ctx
            .twilight_client
            .create_message(user_to_ban_dm.id)
            .embeds(&embeds)?
            .await
        {
            Ok(_) => interaction_response(
                guild,
                author,
                user_to_remove,
                guild_id,
                &reason,
                &period_string,
                true,
            ),
            Err(_) => interaction_response(
                guild,
                author,
                user_to_remove,
                guild_id,
                &reason,
                &period_string,
                false,
            ),
        }
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
