use luro_model::ACCENT_COLOUR;
use tracing::warn;
use twilight_model::{
    application::interaction::message_component::MessageComponentInteractionData,
    gateway::payload::incoming::InteractionCreate, user::User,
};

use crate::{ComponentInteraction, Context, Luro};

use super::InteractionTrait;

mod accent_colour;
mod author;
mod command_name;
mod interaction_client;
mod respond;
mod respond_create;
mod response_simple;
mod response_update;

impl<T> Luro for ComponentInteraction<T> {
    async fn interaction_client(&self) -> anyhow::Result<twilight_http::client::InteractionClient> {
        Ok(self.twilight_client.interaction(self.application_id))
    }

    fn database(&self) -> std::sync::Arc<crate::DatabaseEngine> {
        self.database.clone()
    }

    fn twilight_client(&self) -> std::sync::Arc<twilight_http::Client> {
        self.twilight_client.clone()
    }
}

impl<T> InteractionTrait for ComponentInteraction<T> {
    fn command_name(&self) -> &str {
        &self.data.custom_id
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

impl<T> ComponentInteraction<T> {
    pub fn new(
        ctx: Context,
        interaction: Box<InteractionCreate>,
        data: Box<MessageComponentInteractionData>,
        command: T,
    ) -> Self {
        ComponentInteraction {
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
            #[cfg(feature = "lavalink")]
            lavalink: ctx.lavalink,
            locale: interaction.locale.clone(),
            member: interaction.member.clone(),
            message: interaction.message.clone().unwrap(),
            original: interaction.0.clone(),
            shard: ctx.shard,
            token: interaction.token.clone(),
            tracing_subscriber: ctx.tracing_subscriber,
            twilight_client: ctx.twilight_client,
            user: interaction.user.clone(),
        }
    }
}
