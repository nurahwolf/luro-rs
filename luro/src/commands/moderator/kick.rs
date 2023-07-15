use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::{application::interaction::Interaction, guild::Permissions};

use crate::{
    framework::LuroFramework,
    interactions::InteractionResponse,
    permissions::GuildPermissions,
    responses::embeds::{
        bot_hierarchy::bot_hierarchy,
        bot_missing_permissions::bot_missing_permission,
        kick::{embed, interaction_response},
        not_member::not_member,
        server_owner::server_owner,
        unable_to_get_guild::unable_to_get_guild,
        user_hierarchy::user_hierarchy,
    },
};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "kick",
    desc = "Kick a user",
    dm_permission = false,
    default_permissions = "KickCommand::default_permissions"
)]
pub struct KickCommand {
    /// The user to ban
    pub user: ResolvedUser,
    /// The reason they should be banned
    pub reason: Option<String>,
}

impl KickCommand {
    fn default_permissions() -> Permissions {
        Permissions::KICK_MEMBERS
    }

    pub async fn run(
        self,
        ctx: &LuroFramework,
        interaction: &Interaction,
    ) -> Result<InteractionResponse, anyhow::Error> {
        let reason = match self.reason {
            Some(reason) => reason,
            None => String::new(),
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
        let author_user = match &interaction.member {
            Some(member) => match &member.user {
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
        )?
        .build()];
        match ctx
            .twilight_client
            .create_message(user_to_ban_dm.id)
            .embeds(&embeds)?
            .await
        {
            Ok(_) => interaction_response(guild, author, user_to_remove, guild_id, &reason, true),
            Err(_) => interaction_response(guild, author, user_to_remove, guild_id, &reason, false),
        }
    }
}
