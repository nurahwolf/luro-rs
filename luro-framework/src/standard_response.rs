use anyhow::Error;
use luro_model::builders::EmbedBuilder;
use twilight_model::{
    channel::message::embed::EmbedField,
    guild::Permissions,
    id::{marker::UserMarker, Id},
};

use self::{
    bot_heirarchy::bot_hierarchy_embed, bot_missing_permission::bot_missing_permission_embed,
    missing_permissions::missing_permission_embed, not_owner::not_owner_embed, permission_modify_server_owner::permission_server_owner,
    user_heirarchy::user_hierarchy_embed,
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

/// A wrapper around [EmbedBuilder] to make easy standardised responses
#[derive(Default, Clone)]
pub struct StandardResponse {
    /// The internal embed, if you wish to manipulate it directly
    pub embed: EmbedBuilder,
}

impl StandardResponse {
    pub fn new() -> Self {
        Self { embed: Default::default() }
    }

    /// Clone the internal embed and return it. Useful for if you don't want to clone it manually.
    ///
    /// Generally used when the response is reused
    pub fn embed(&self) -> EmbedBuilder {
        self.embed.clone()
    }

    /// Append a field to state if the response was successfully sent in a DM
    pub fn dm_sent(&mut self, success: bool) -> &mut Self {
        match success {
            true => self.embed.create_field("DM Sent", "Successful", true),
            false => self.embed.create_field("DM Sent", "Failed", true),
        };
        self
    }

    /// Create and append a filed directly to the embed
    /// NOTE: If the resulting embed is being sent by Luro, it is checked to make sure we are not over 25 fields.
    /// There is NO check for this in the builder itself!
    pub fn create_field<S: ToString>(&mut self, name: S, value: S, inline: bool) -> &mut Self {
        let field = EmbedField {
            inline,
            name: name.to_string(),
            value: value.to_string(),
        };

        self.embed.0.fields.push(field);
        self
    }
}
pub enum Response<'a> {
    InternalError(Error),
    PermissionNotBotStaff(),
    PermissionModifyServerOwner(&'a Id<UserMarker>),
    UnknownCommand(&'a str),
    NotGuild,
    BotMissingPermission(Permissions),
    UserHeirarchy(&'a str),
    BotHeirarchy(&'a str),
    MissingPermission(Permissions),
    NotOwner(&'a Id<UserMarker>, &'a str),
}

impl<'a> Response<'a> {
    /// Convert the response to an embed
    pub fn embed(self) -> EmbedBuilder {
        match self {
            Self::InternalError(error) => internal_error::internal_error(error),
            Self::PermissionNotBotStaff() => permission_not_bot_staff::permission_not_bot_staff(),
            Self::PermissionModifyServerOwner(user_id) => permission_server_owner(user_id),
            Self::UnknownCommand(name) => unknown_command::unknown_command(name),
            Self::NotGuild => not_guild::not_guild(),
            Self::BotMissingPermission(permission) => bot_missing_permission_embed(permission),
            Self::UserHeirarchy(username) => user_hierarchy_embed(username),
            Self::BotHeirarchy(username) => bot_hierarchy_embed(username),
            Self::MissingPermission(permission) => missing_permission_embed(permission),
            Self::NotOwner(user_id, command_name) => not_owner_embed(user_id, command_name),
        }
    }
}

/// The type of punishment
pub enum PunishmentType {
    Kicked,
    Banned,
    Unbanned,
}
