use luro_model::ACCENT_COLOUR;
use tracing::warn;
use twilight_model::{application::interaction::{Interaction, application_command::CommandData}, gateway::payload::incoming::InteractionCreate, user::User};

use crate::{CommandInteraction, Context};

use super::InteractionTrait;

mod acknowledge_interaction;
mod author;
mod command_name;
mod get_interaction_author;
mod get_specific_user_or_author;
mod interaction_client;
mod respond;
mod respond_message;
mod response_create;
mod response_send;
mod response_update;
mod response_simple;
mod send_log_channel;
mod send_message;
mod command_data;
mod webhook;

impl<T> InteractionTrait for CommandInteraction<T> {
    fn command_name(&self) -> &str {
        &self.data.name
    }

        /// The user that invoked the interaction.
    ///
    /// This will first check for the [`member`]'s
    /// [`user`][`PartialMember::user`] and then, if not present, check the
    /// [`user`].
    ///
    /// [`member`]: Self::member
    /// [`user`]: Self::user
    fn author(&self) -> &User {
        match self.member.as_ref() {
            Some(member) if member.user.is_some() => member.user.as_ref().unwrap(),
            _ => self.user.as_ref().unwrap(),
        }
    }

    /// Attempts to get the guild's accent colour, else falls back to getting the hardcoded accent colour
    async fn accent_colour(&self) -> u32 {
        match self.guild_id {
            Some(guild_id) => {
                match self
                    .database
                    .get_guild(&guild_id)
                    .await
                    .map(|mut x| x.highest_role_colour().map(|x| x.0))
                {
                    Ok(colour) => colour.unwrap_or(ACCENT_COLOUR),
                    Err(why) => {
                        warn!(why = ?why, "Failed to get guild accent colour");
                        ACCENT_COLOUR
                    }
                }
            }
            None => ACCENT_COLOUR, // There is no guild for this interaction
        }
    }
}

impl<T> CommandInteraction<T> {
    pub fn new(ctx: Context, interaction: Box<InteractionCreate>, data: Box<CommandData>, command: T) -> Self {
        CommandInteraction {
            command,
            app_permissions: interaction.app_permissions,
            application_id: interaction.application_id,
            cache: ctx.cache,
            channel: interaction.channel.clone().unwrap(),
            data,
            database: ctx.database,
            global_commands: ctx.global_commands,
            guild_commands: ctx.guild_commands,
            guild_id: interaction.guild_id,
            guild_locale: interaction.guild_locale.clone(),
            http_client: ctx.http_client,
            id: interaction.id,
            kind: interaction.kind,
            latency: ctx.latency,
            lavalink: ctx.lavalink,
            locale: interaction.locale.clone(),
            member: interaction.member.clone(),
            message: interaction.message.clone(),
            original: interaction.0.clone(),
            shard: ctx.shard,
            token: interaction.token.clone(),
            tracing_subscriber: ctx.tracing_subscriber,
            twilight_client: ctx.twilight_client,
            user: interaction.user.clone(),
        }
    }
}

impl<T> From<CommandInteraction<T>> for Interaction {
    fn from(val: CommandInteraction<T>) -> Self {
        val.original
    }
}
