use luro_builder::response::LuroResponse;
use luro_model::{luro_database_driver::LuroDatabaseDriver, luro_log_channel::LuroLogChannel};
use tracing::{debug, info};

use twilight_http::{client::InteractionClient, Error, Response};

use twilight_model::{
    application::command::Command,
    channel::Message,
    http::attachment::Attachment,
    id::{
        marker::{ApplicationMarker, ChannelMarker, GuildMarker},
        Id
    }
};
use twilight_util::builder::embed::EmbedBuilder;

use crate::{framework::Framework, traits::luro_functions::LuroFunctions, LuroFramework, ACCENT_COLOUR};

use super::Commands;

impl LuroFunctions for LuroFramework {}

impl<D: LuroDatabaseDriver> Framework<D> {
    /// Create a response to an interaction.
    /// This automatically handles if the interaction had been deferred.
    pub async fn send_message<F>(&self, channel: &Id<ChannelMarker>, response: F) -> Result<Response<Message>, Error>
    where
        F: FnOnce(&mut LuroResponse) -> &mut LuroResponse
    {
        let mut r = LuroResponse::default();
        response(&mut r);

        let mut create_message = self
            .twilight_client
            .create_message(*channel)
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

    /// Create a response to an interaction.
    /// This automatically handles if the interaction had been deferred.
    pub async fn send_log_channel_new<F>(
        &self,
        guild_id: &Option<Id<GuildMarker>>,
        log_channel: LuroLogChannel,
        response: F
    ) -> anyhow::Result<()>
    where
        F: FnOnce(&mut LuroResponse) -> &mut LuroResponse
    {
        debug!("Attempting to send to log channel");
        // TODO: Send event to main logging channel if not defined
        let (guild_data, guild_id) = match guild_id {
            Some(guild_id) => (self.database.get_guild(guild_id).await?, guild_id),
            None => return Ok(())
        };

        let log_channel_requested = match log_channel {
            LuroLogChannel::Catchall => guild_data.catchall_log_channel,
            LuroLogChannel::Message => guild_data.message_events_log_channel,
            LuroLogChannel::Moderator => guild_data.moderator_actions_log_channel,
            LuroLogChannel::Thread => guild_data.thread_events_log_channel
        };

        let log_channel = match log_channel_requested {
            Some(log_channel) => log_channel,
            None => match guild_data.catchall_log_channel {
                Some(channel) => channel,
                None => {
                    info!("Guild {guild_id} does not have a catchall channel defined");
                    return Ok(());
                }
            }
        };

        self.send_message(&log_channel, response).await?;
        Ok(())
    }

    /// Attempts to send to a log channel if it is present.
    pub async fn send_log_channel(
        &self,
        guild_id: &Option<Id<GuildMarker>>,
        embed: EmbedBuilder,
        log_channel: LuroLogChannel
    ) -> anyhow::Result<()> {
        debug!("Attempting to send to log channel");
        let guild_id = match guild_id {
            Some(data) => data,
            None => return Ok(())
        };
        let guild_data = self.database.get_guild(guild_id).await?;

        let log_channel = match log_channel {
            LuroLogChannel::Catchall => guild_data.catchall_log_channel,
            LuroLogChannel::Message => guild_data.message_events_log_channel,
            LuroLogChannel::Moderator => guild_data.moderator_actions_log_channel,
            LuroLogChannel::Thread => guild_data.thread_events_log_channel
        };

        let log_channel = match log_channel {
            Some(data) => data,
            None => match guild_data.catchall_log_channel {
                Some(channel) => channel,
                None => {
                    info!("Guild {guild_id} does not have a catchall channel defined");
                    return Ok(());
                }
            }
        };
        let mut embed = embed.build();
        let mut file_id = 0;
        let mut files = vec![];

        if let Some(description) = &mut embed.description {
            if description.len() > 4096 {
                file_id += 1;

                files.push(Attachment::from_bytes(
                    format!("Embed-{file_id}.txt"),
                    description.as_bytes().to_vec(),
                    file_id
                ));

                description.truncate(4093);
                description.push_str("...");
            }
        }

        for field in &mut embed.fields {
            if field.value.len() > 1000 {
                file_id += 1;

                files.push(Attachment::from_bytes(
                    format!("Field-{file_id}.txt"),
                    field.value.as_bytes().to_vec(),
                    file_id
                ));

                field.value.truncate(997);
                field.value.push_str("...");
            }
        }

        self.twilight_client
            .create_message(log_channel)
            .embeds(&[embed])
            .attachments(&files)
            .await?;

        Ok(())
    }

    /// Attempts to send to a moderator log channel if it is present.
    pub async fn send_moderator_log_channel(
        &self,
        guild_id: &Option<Id<GuildMarker>>,
        embed: EmbedBuilder
    ) -> anyhow::Result<()> {
        debug!("Attempting to send to log channel");
        let guild_id = match guild_id {
            Some(data) => data,
            None => return Ok(())
        };
        let guild_data = self.database.get_guild(guild_id).await?;
        let log_channel = match guild_data.moderator_actions_log_channel {
            Some(data) => data,
            None => return Ok(())
        };

        self.twilight_client
            .create_message(log_channel)
            .embeds(&[embed.build()])
            .await?;

        debug!("Successfully sent to log channel");
        Ok(())
    }

    /// Create a default embed which has the guild's accent colour if available, otherwise falls back to Luro's accent colour
    pub async fn default_embed(&self, guild_id: &Option<Id<GuildMarker>>) -> EmbedBuilder {
        EmbedBuilder::new().color(self.accent_colour(guild_id).await)
    }

    /// Attempts to get the guild's accent colour, else falls back to getting the hardcoded accent colour
    pub async fn accent_colour(&self, guild_id: &Option<Id<GuildMarker>>) -> u32 {
        if let Some(guild_id) = guild_id {
            let guild_settings = self.database.get_guild(guild_id).await;

            if let Ok(guild_settings) = guild_settings {
                // Check to see if a custom colour is defined
                if let Some(custom_accent_colour) = guild_settings.accent_colour_custom {
                    return custom_accent_colour;
                };

                if guild_settings.accent_colour != 0 {
                    return guild_settings.accent_colour;
                }
            }
        };

        ACCENT_COLOUR
    }

    /// Gets the [interaction client](InteractionClient) using this framework's
    /// [http client](Client) and [application id](ApplicationMarker)
    pub fn interaction_client(&self) -> InteractionClient {
        self.twilight_client.interaction(self.database.application.read().unwrap().id)
    }
    /// Register commands to the Discord API.
    pub async fn register_commands(&self, application_id: Id<ApplicationMarker>) -> anyhow::Result<()> {
        let client = self.twilight_client.interaction(application_id);

        match client
            .set_global_commands(
                &Commands::default_commands()
                    .global_commands
                    .into_values()
                    .collect::<Vec<Command>>()
            )
            .await
        {
            Ok(command_result) => Ok(info!(
                "Successfully registered {} global commands!",
                command_result.model().await?.len()
            )),
            Err(why) => Err(why.into())
        }
    }
}
