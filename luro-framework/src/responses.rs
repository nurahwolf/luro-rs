use anyhow::Error;
use luro_builder::embed::EmbedBuilder;
use luro_model::{
    database::drivers::LuroDatabaseDriver,
    user::{actions::UserActions, actions_type::UserActionType},
};
use twilight_model::{
    channel::message::embed::EmbedField,
    id::{marker::UserMarker, Id},
};

use crate::{Framework, InteractionContext, LuroInteraction};

mod internal_error;
pub mod permission_modify_server_owner; // TODO: Change to private, only needs to be public for old framework
mod permission_not_bot_staff;
mod unknown_command;
mod user_action;

/// A wrapper around [EmbedBuilder] to make easy standardised responses
#[derive(Default, Clone)]
pub struct StandardResponse {
    /// The internal embed, if you wish to manipulate it directly
    pub embed: EmbedBuilder,
}

impl StandardResponse {
    pub fn new() -> Self {
        Self {
            embed: Default::default(),
        }
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

    /// Respond to an interaction with a standard response
    pub async fn interaction_response<D: LuroDatabaseDriver>(
        &self,
        framework: Framework<D>,
        ctx: InteractionContext,
    ) -> anyhow::Result<()> {
        ctx.respond(&framework, |response| response.add_embed(self.embed())).await?;
        Ok(())
    }
}
pub enum SimpleResponse<'a> {
    InternalError(&'a Error),
    PermissionNotBotStaff(),
    PermissionModifyServerOwner(&'a Id<UserMarker>),
    UnknownCommand(&'a str),
}

impl<'a, 'b> SimpleResponse<'a> {
    /// Convert the response to an embed
    pub fn embed(self) -> EmbedBuilder {
        match self {
            SimpleResponse::InternalError(error) => internal_error::internal_error(error),
            SimpleResponse::PermissionNotBotStaff() => permission_not_bot_staff::permission_not_bot_staff(),
            SimpleResponse::PermissionModifyServerOwner(user_id) => {
                permission_modify_server_owner::permission_server_owner(user_id)
            }
            SimpleResponse::UnknownCommand(name) => unknown_command::unknown_command(name),
        }
    }

    pub async fn respond<D: LuroDatabaseDriver, T: LuroInteraction>(
        self,
        framework: &Framework<D>,
        interaction: &'a T,
    ) -> anyhow::Result<()> {
        match self {
            SimpleResponse::PermissionNotBotStaff() => privelege_escalation(framework, interaction).await,
            _ => Ok(()),
        }?;

        interaction
            .respond(framework, |response| response.add_embed(self.embed()))
            .await
    }

    pub async fn unknown_command<D: LuroDatabaseDriver, T: LuroInteraction>(
        framework: &Framework<D>,
        interaction: &'a T,
    ) -> anyhow::Result<()> {
        let embed = Self::UnknownCommand(interaction.command_name()).embed();

        interaction.respond(framework, |response| response.add_embed(embed)).await
    }
}

async fn privelege_escalation<D: LuroDatabaseDriver, T: LuroInteraction>(
    framework: &Framework<D>,
    interaction: &T,
) -> anyhow::Result<()> {
    let mut user_data = framework.database.get_user(&interaction.author_id(), false).await?;
    user_data.moderation_actions.push(UserActions {
        action_type: vec![UserActionType::PrivilegeEscalation],
        guild_id: interaction.guild_id(),
        reason: Some(format!("Attempted to run the {} command", interaction.command_name())),
        responsible_user: interaction.author_id(),
    });
    framework.database.save_user(&interaction.author_id(), &user_data).await?;
    Ok(())
}

/// The type of punishment
pub enum PunishmentType {
    Kicked,
    Banned,
    Unbanned,
}
