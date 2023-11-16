use twilight_model::{id::{Id, marker::UserMarker}, guild::Permissions};

use crate::{builders::EmbedBuilder, types::{PunishmentType, Guild, User}};

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

pub enum SimpleResponse<'a> {
    InternalError(anyhow::Error),
    PermissionNotBotStaff(),
    PermissionModifyServerOwner(&'a Id<UserMarker>),
    UnknownCommand(&'a str),
    NotGuild,
    BotMissingPermission(Permissions),
    UserHeirarchy(&'a str),
    BotHeirarchy(&'a str),
    MissingPermission(Permissions),
    NotOwner(&'a Id<UserMarker>, &'a str),
    /// A punishment applied to a user, such as a ban or kick. First user paramater is the moderator, second is the target
    Punishment(
        &'a Guild,
        PunishmentType,
        &'a User,
        &'a User,
    )
}

impl<'a> SimpleResponse<'a> {
    /// Convert the response to an embed
    pub fn embed(self) -> EmbedBuilder {
        match self {
            Self::InternalError(error) => internal_error::internal_error(error),
            Self::PermissionNotBotStaff() => permission_not_bot_staff::permission_not_bot_staff(),
            Self::PermissionModifyServerOwner(user_id) => permission_modify_server_owner::permission_server_owner(user_id),
            Self::UnknownCommand(name) => unknown_command::unknown_command(name),
            Self::NotGuild => not_guild::not_guild(),
            Self::BotMissingPermission(permission) => bot_missing_permission::bot_missing_permission_embed(permission),
            Self::UserHeirarchy(username) => user_heirarchy::user_hierarchy_embed(username),
            Self::BotHeirarchy(username) => bot_heirarchy::bot_hierarchy_embed(username),
            Self::MissingPermission(permission) => missing_permissions::missing_permission_embed(permission),
            Self::NotOwner(user_id, command_name) => not_owner::not_owner_embed(user_id, command_name),
            Self::Punishment(guild, kind, moderator, target) => user_action::new_punishment_embed(guild, kind, moderator, target)
        }
    }

    /// Append a field to state if the response was successfully sent in a DM
    pub fn dm_sent(embed: &mut EmbedBuilder, success: bool) -> &mut EmbedBuilder {
        match success {
            true => embed.create_field("DM Sent", "Successful", true),
            false => embed.create_field("DM Sent", "Failed", true),
        };

        embed
    }
}
