use twilight_model::{
    channel::message::Embed,
    guild::Permissions,
    id::{marker::UserMarker, Id},
};

use crate::{
    builders::EmbedBuilder,
    embeds::{ban_logged, ban_user},
    types::{Guild, PunishmentType, User},
};

mod bot_heirarchy;
mod bot_missing_permission;
mod internal_error;
mod missing_permissions;
mod not_guild;
mod not_owner;
mod permission_modify_server_owner;
mod permission_not_bot_staff;
mod unknown_command;
mod user_action;
mod user_heirarchy;

pub struct BannedResponse<'a> {
    pub target: &'a User,
    pub moderator: &'a User,
    pub reason: Option<&'a str>,
    pub purged_messages: i64,
}

pub enum SimpleResponse<'a> {
    InternalError(&'a anyhow::Error),
    PermissionNotBotStaff,
    PermissionModifyServerOwner(&'a Id<UserMarker>),
    UnknownCommand(&'a str),
    NotGuild,
    BotMissingPermission(&'a Permissions),
    UserHeirarchy(&'a User, &'a User),
    BotHeirarchy(&'a User, &'a User),
    MissingPermission(&'a Permissions),
    NotOwner(&'a Id<UserMarker>, &'a str),
    /// A punishment applied to a user, such as a ban or kick. First user paramater is the moderator, second is the target
    Punishment(&'a Guild, PunishmentType, &'a User, &'a User),
    /// A ban response sent to the user banned. String parameter is the guild name.
    BannedUserResponse(BannedResponse<'a>, &'a str),
    /// A ban response sent to the moderator, or log channel. bool is if the user was DMed successfully.
    BannedModeratorResponse(BannedResponse<'a>, bool),
}

impl<'a> SimpleResponse<'a> {
    /// Convert the response to an [EmbedBuilder]
    pub fn builder(&self) -> EmbedBuilder {
        match self {
            Self::InternalError(error) => internal_error::internal_error(error),
            Self::PermissionNotBotStaff => permission_not_bot_staff::permission_not_bot_staff(),
            Self::PermissionModifyServerOwner(user_id) => permission_modify_server_owner::permission_server_owner(user_id),
            Self::UnknownCommand(name) => unknown_command::unknown_command(name),
            Self::NotGuild => not_guild::not_guild(),
            Self::BotMissingPermission(permission) => bot_missing_permission::bot_missing_permission_embed(permission),
            Self::UserHeirarchy(user, target) => user_heirarchy::user_hierarchy_embed(user, target),
            Self::BotHeirarchy(user, bot) => bot_heirarchy::bot_hierarchy_embed(user, bot),
            Self::MissingPermission(permission) => missing_permissions::missing_permission_embed(permission),
            Self::NotOwner(user_id, command_name) => not_owner::not_owner_embed(user_id, command_name),
            Self::BannedUserResponse(data, guild_name) => ban_user(data, guild_name),
            Self::BannedModeratorResponse(data, dm_success) => ban_logged(data, dm_success),
            Self::Punishment(guild, kind, moderator, target) => {
                user_action::new_punishment_embed(guild, kind, moderator, target).unwrap()
            }
        }
    }

    /// Convert the response to a twilight [Embed]
    pub fn embed(&self) -> Embed {
        self.builder().into()
    }

    /// Append a field to state if the response was successfully sent in a DM
    pub fn dm_sent(&self, success: bool) -> EmbedBuilder {
        let mut embed = self.builder();
        let success = match success {
            true => "<:mail:1175136204648349756> **Direct Message:** `Success!` <:join:1175114514216259615>",
            false => "<:mail:1175136204648349756> **Direct Message:** `Failed!` <:leave:1175114521652756641>",
        };

        match embed.0.description {
            Some(ref mut description) => description.push_str(success),
            None => embed.0.description = Some(success.to_owned()),
        };

        embed
    }

    /// Append the guild information as the footer
    pub fn guild_info(&self, guild: &Guild) -> EmbedBuilder {
        let mut embed = self.builder();
        embed.footer(|footer| footer.text(format!("Guild: {}", guild.name)));
        embed
    }
}
