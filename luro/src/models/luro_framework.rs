use luro_model::{luro_database_driver::LuroDatabaseDriver, luro_log_channel::LuroLogChannel};
use tracing::{debug, info, warn};

use twilight_http::client::InteractionClient;

use twilight_model::{
    application::command::Command,
    id::{
        marker::{ApplicationMarker, GuildMarker},
        Id
    }, http::attachment::Attachment
};
use twilight_util::builder::embed::EmbedBuilder;

use crate::{framework::Framework, traits::luro_functions::LuroFunctions, LuroFramework, ACCENT_COLOUR};

use super::Commands;

impl LuroFunctions for LuroFramework {}

impl<D: LuroDatabaseDriver> Framework<D> {
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
                    warn!("Guild {guild_id} does not have a catchall channel defined");
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
            
            self.twilight_client.create_message(log_channel).embeds(&[embed]).attachments(&files).await?;

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
