use anyhow::anyhow;
use std::sync::Arc;

use luro_builder::response::LuroResponse;
use luro_database::TomlDatabaseDriver;
use tracing::error;
use twilight_http::client::InteractionClient;
use twilight_http::Error;
use twilight_http::Response;
use twilight_interactions::command::ResolvedUser;
use twilight_model::application::interaction::Interaction;
use twilight_model::channel::Message;
use twilight_model::http::interaction::InteractionResponseType;
use twilight_model::http::interaction::InteractionResponseType::DeferredChannelMessageWithSource;
use twilight_model::http::interaction::InteractionResponseType::DeferredUpdateMessage;
use twilight_model::user::User;

use crate::framework::Framework;
use crate::models::SlashUser;
use crate::ACCENT_COLOUR;

mod user_utils;

/// Some nice stuff about formatting a response, ready to send via twilight's client
#[derive(Clone, Debug)]
pub struct LuroInteraction {
    /// The framework for being able to respond to an interaction
    pub framework: Arc<Framework<TomlDatabaseDriver>>,
    /// The client is wrapped around this interaction
    pub interaction: Interaction
}

impl LuroInteraction {
    /// Create a client wrapped around an interaction. Note that not setting anything else will not cause a response to be sent!
    /// This is set with some defaults:
    /// - AllowedMentions - All
    /// - InteractionResponseType - [`InteractionResponseType::ChannelMessageWithSource`]
    pub fn new(framework: Arc<Framework<TomlDatabaseDriver>>, interaction: Interaction) -> Self {
        Self { framework, interaction }
    }

    /// Create a response to an interaction.
    /// This automatically handles if the interaction had been deferred.
    pub async fn respond<F>(&self, response: F) -> anyhow::Result<()>
    where
        F: FnOnce(&mut LuroResponse) -> &mut LuroResponse
    {
        let mut r = LuroResponse::default();
        response(&mut r);

        if r.interaction_response_type == DeferredChannelMessageWithSource
            || r.interaction_response_type == DeferredUpdateMessage
        {
            self.update_response(r).await?;
            return Ok(());
        }

        self.create_response(&r).await
    }

    /// Create an interaction client
    pub fn interaction_client(&self) -> InteractionClient {
        self.framework.twilight_client.interaction(self.interaction.application_id)
    }

    /// Create a response. This is used for sending a response to an interaction, as well as to defer interactions.
    pub async fn create_response(&self, response: &LuroResponse) -> anyhow::Result<()> {
        self.interaction_client()
            .create_response(self.interaction.id, &self.interaction.token, &response.interaction_response())
            .await?;
        Ok(())
    }

    pub async fn update_response(&self, response: LuroResponse) -> Result<Response<Message>, Error> {
        let client = self.interaction_client();
        let update_response = client
            .update_response(&self.interaction.token)
            .allowed_mentions(response.allowed_mentions.as_ref())
            .components(response.components.as_deref())
            .content(response.content.as_deref())
            .embeds(response.embeds.as_deref());

        update_response.await
    }

    /// Acknowledge the interaction, showing a loading state. This will then be updated later.
    ///
    /// Use this for operations that take a long time. Generally its best to send this as soon as the reaction has been received.
    pub async fn acknowledge_interaction(&self) -> anyhow::Result<()> {
        let mut response = LuroResponse::default();
        response.interaction_response_type = InteractionResponseType::DeferredChannelMessageWithSource;

        self.create_response(&response).await
    }

    /// Attempts to get the guild's accent colour, else falls back to getting the hardcoded accent colour
    pub async fn accent_colour(&self) -> u32 {
        if let Some(guild_id) = &self.interaction.guild_id {
            let guild_settings = match self.framework.database.get_guild(guild_id).await {
                Ok(guild_settings) => guild_settings,
                Err(why) => {
                    error!(why = ?why, "Failed to get guild settings when attempting to get guild's accent colour");
                    return ACCENT_COLOUR;
                }
            };

            // Check to see if a custom colour is defined
            if let Some(custom_accent_colour) = guild_settings.accent_colour_custom {
                return custom_accent_colour;
            };

            if guild_settings.accent_colour != 0 {
                return guild_settings.accent_colour;
            }
        };

        ACCENT_COLOUR
    }
}
