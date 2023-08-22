use std::sync::Arc;

use luro_builder::embed::EmbedBuilder;
use luro_builder::response::LuroResponse;
use luro_model::database::drivers::LuroDatabaseDriver;
use luro_model::user::LuroUser;
use tracing::debug;
use tracing::error;
use twilight_gateway::Latency;
use twilight_gateway::MessageSender;
use twilight_http::client::InteractionClient;
use twilight_http::Error;
use twilight_http::Response;
use twilight_model::application::interaction::Interaction;
use twilight_model::channel::message::MessageFlags;
use twilight_model::channel::Message;
use twilight_model::http::interaction::InteractionResponseType;
use twilight_model::http::interaction::InteractionResponseType::DeferredChannelMessageWithSource;
use twilight_model::http::interaction::InteractionResponseType::DeferredUpdateMessage;

use crate::framework::Framework;
use crate::ACCENT_COLOUR;

mod handle;
mod parse_modal_field;
mod parsers;
mod send_log_message;
mod user_utils;

/// Some nice stuff about formatting a response, ready to send via twilight's client
#[derive(Clone, Debug)]
pub struct LuroSlash<D: LuroDatabaseDriver> {
    /// The framework for being able to respond to an interaction
    pub framework: Arc<Framework<D>>,
    /// The client is wrapped around this interaction
    pub interaction: Interaction,
    pub shard: MessageSender,
    pub latency: Latency
}

impl<D: LuroDatabaseDriver> LuroSlash<D> {
    /// Create a client wrapped around an interaction. Note that not setting anything else will not cause a response to be sent!
    /// This is set with some defaults:
    /// - AllowedMentions - All
    /// - InteractionResponseType - [`InteractionResponseType::ChannelMessageWithSource`]
    pub fn new(framework: Arc<Framework<D>>, interaction: Interaction, shard: MessageSender, latency: Latency) -> Self {
        Self {
            framework,
            interaction,
            shard,
            latency
        }
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
            self.update_response(&r).await?;
            return Ok(());
        }

        self.create_response(&r).await
    }

    /// Send an existing response builder a response to an interaction.
    /// This automatically handles if the interaction had been deferred.
    pub async fn send_respond(&self, response: LuroResponse) -> anyhow::Result<()> {
        self.respond(|r| {
            *r = response;
            r
        })
        .await
    }

    /// Create a default embed which has the guild's accent colour if available, otherwise falls back to Luro's accent colour
    pub async fn default_embed(&self) -> EmbedBuilder {
        EmbedBuilder::default().colour(self.accent_colour().await).clone()
    }

    /// Create an interaction client
    pub fn interaction_client(&self) -> InteractionClient {
        self.framework.twilight_client.interaction(self.interaction.application_id)
    }

    /// Create a response. This is used for sending a response to an interaction, as well as to defer interactions.
    async fn create_response(&self, response: &LuroResponse) -> anyhow::Result<()> {
        let request = response.interaction_response();
        debug!("{:#?}", request);
        let response_result = self
            .interaction_client()
            .create_response(self.interaction.id, &self.interaction.token, &request)
            .await;

        if let Err(why) = response_result {
            if self.update_response(response).await.is_err() {
                error!("Failed to respond to interaction - {:#?}", why);
            }
        }

        Ok(())
    }

    async fn update_response(&self, response: &LuroResponse) -> Result<Response<Message>, Error> {
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
    pub async fn acknowledge_interaction(&self, ephemeral: bool) -> anyhow::Result<LuroResponse> {
        let response = LuroResponse {
            interaction_response_type: InteractionResponseType::DeferredChannelMessageWithSource,
            flags: if ephemeral { Some(MessageFlags::EPHEMERAL) } else { None },
            ..Default::default()
        };

        self.create_response(&response).await?;
        Ok(response)
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

    /// Send a message in the same channel as the interaction
    ///
    /// #PANIC
    /// This function panics if its called in a ping function... No idea why you would try that, but please don't.
    pub async fn send_message<F>(&self, response: F) -> Result<Response<Message>, Error>
    where
        F: FnOnce(&mut LuroResponse) -> &mut LuroResponse
    {
        let mut r = LuroResponse::default();
        response(&mut r);

        let mut create_message = self
            .framework
            .twilight_client
            .create_message(self.interaction.channel.as_ref().unwrap().id)
            .allowed_mentions(r.allowed_mentions.as_ref());

        if let Some(attachments) = &r.attachments {
            create_message = create_message.attachments(attachments);
        }
        if let Some(components) = &r.components {
            create_message = create_message.components(components);
        }
        if let Some(content) = &r.content {
            create_message = create_message.content(content);
        }
        if let Some(embeds) = &r.embeds {
            create_message = create_message.embeds(embeds);
        }
        if let Some(flags) = r.flags {
            create_message = create_message.flags(flags);
        }
        if let Some(reply) = r.reply {
            create_message = create_message.reply(reply);
        }
        if let Some(stickers) = &r.stickers {
            create_message = create_message.sticker_ids(stickers);
        }
        if let Some(tts) = r.tts {
            create_message = create_message.tts(tts);
        }

        create_message.await
    }

    pub async fn update_user<'a>(&'a self, user: &'a mut LuroUser) -> anyhow::Result<&'a mut LuroUser> {
        match self.interaction.guild_id {
            Some(guild_id) => user.update_member(
                &guild_id,
                &self
                    .framework
                    .twilight_client
                    .guild_member(guild_id, user.id)
                    .await?
                    .model()
                    .await?
            ),
            None => user.update_user(&self.framework.twilight_client.user(user.id).await?.model().await?)
        };
        Ok(user)
    }
}
