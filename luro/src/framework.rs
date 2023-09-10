use std::sync::Arc;

use hyper::client::HttpConnector;
use luro_builder::embed::EmbedBuilder;

use luro_database::LuroDatabase;
use luro_model::{
    guild::log_channel::LuroLogChannel,
    response::LuroResponse,
    user::LuroUser,
    ACCENT_COLOUR, configuration::Configuration, database_driver::LuroDatabaseDriver,
};
use tracing::{debug, info};
use tracing_subscriber::{filter::LevelFilter, reload::Handle, Registry};
use twilight_cache_inmemory::InMemoryCache;
use twilight_gateway::{stream, Shard};
use twilight_http::{client::InteractionClient, Error, Response};
use twilight_lavalink::Lavalink;
use twilight_model::{
    application::command::Command,
    channel::{message::Embed, Message},
    http::attachment::Attachment,
    id::{
        marker::{ApplicationMarker, ChannelMarker, GuildMarker, UserMarker},
        Id,
    },
};

use crate::interactions::Commands;

/// The core of Luro. Used to handle our global state and generally wrapped in an [Arc].
#[derive(Debug)]
pub struct Framework<D: LuroDatabaseDriver> {
    pub database: LuroDatabase<D>,
    /// HTTP client used for making outbound API requests
    pub hyper_client: hyper::Client<HttpConnector>,
    /// Lavalink client, for playing music
    pub lavalink: Lavalink,
    /// Twilight's client for interacting with the Discord API
    pub twilight_client: Arc<twilight_http::Client>,
    /// Twilight's cache
    pub twilight_cache: Arc<InMemoryCache>,
    /// The global tracing subscriber, for allowing manipulation within commands
    pub tracing_subscriber: Handle<LevelFilter, Registry>,
}

impl<D: LuroDatabaseDriver> Framework<D> {
    /// Create a response to an interaction.
    /// This automatically handles if the interaction had been deferred.
    pub async fn send_message<F>(&self, channel: &Id<ChannelMarker>, response: F) -> Result<Response<Message>, Error>
    where
        F: FnOnce(&mut LuroResponse) -> &mut LuroResponse,
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
        response: F,
    ) -> anyhow::Result<()>
    where
        F: FnOnce(&mut LuroResponse) -> &mut LuroResponse,
    {
        debug!("Attempting to send to log channel");
        // TODO: Send event to main logging channel if not defined
        let (guild_data, guild_id) = match guild_id {
            Some(guild_id) => (self.database.get_guild(guild_id).await?, guild_id),
            None => return Ok(()),
        };

        let log_channel_requested = match log_channel {
            LuroLogChannel::Catchall => guild_data.catchall_log_channel,
            LuroLogChannel::Message => guild_data.message_events_log_channel,
            LuroLogChannel::Moderator => guild_data.moderator_actions_log_channel,
            LuroLogChannel::Thread => guild_data.thread_events_log_channel,
        };

        let log_channel = match log_channel_requested {
            Some(log_channel) => log_channel,
            None => match guild_data.catchall_log_channel {
                Some(channel) => channel,
                None => {
                    debug!("Guild {guild_id} does not have a catchall channel defined");
                    return Ok(());
                }
            },
        };

        self.send_message(&log_channel, response).await?;
        Ok(())
    }

    /// Attempts to send to a log channel if it is present.
    pub async fn send_log_channel(
        &self,
        guild_id: &Option<Id<GuildMarker>>,
        mut embed: Embed,
        log_channel: LuroLogChannel,
    ) -> anyhow::Result<()> {
        debug!("Attempting to send to log channel");
        let guild_id = match guild_id {
            Some(data) => data,
            None => return Ok(()),
        };
        let guild_data = self.database.get_guild(guild_id).await?;
        let log_channel = match log_channel {
            LuroLogChannel::Catchall => guild_data.catchall_log_channel,
            LuroLogChannel::Message => guild_data.message_events_log_channel,
            LuroLogChannel::Moderator => guild_data.moderator_actions_log_channel,
            LuroLogChannel::Thread => guild_data.thread_events_log_channel,
        };

        let log_channel = match log_channel {
            Some(data) => data,
            None => match guild_data.catchall_log_channel {
                Some(channel) => channel,
                None => {
                    debug!("Guild {guild_id} does not have a catchall channel defined");
                    return Ok(());
                }
            },
        };
        let mut file_id = 0;
        let mut files = vec![];

        if let Some(description) = &mut embed.description {
            if description.len() > 4096 {
                file_id += 1;

                files.push(Attachment::from_bytes(
                    format!("Embed-{file_id}.txt"),
                    description.as_bytes().to_vec(),
                    file_id,
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
                    file_id,
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
        embed: EmbedBuilder,
    ) -> anyhow::Result<()> {
        debug!("Attempting to send to log channel");
        let guild_id = match guild_id {
            Some(data) => data,
            None => return Ok(()),
        };
        let guild_data = self.database.get_guild(guild_id).await?;
        let log_channel = match guild_data.moderator_actions_log_channel {
            Some(data) => data,
            None => return Ok(()),
        };

        self.twilight_client
            .create_message(log_channel)
            .embeds(&[embed.into()])
            .await?;

        debug!("Successfully sent to log channel");
        Ok(())
    }

    /// Create a default embed which has the guild's accent colour if available, otherwise falls back to Luro's accent colour
    pub async fn default_embed(&self, guild_id: &Option<Id<GuildMarker>>) -> EmbedBuilder {
        EmbedBuilder::default().colour(self.accent_colour(guild_id).await).clone()
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

                if let Some(colour) = guild_settings.accent_colour {
                    return colour;
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
                    .collect::<Vec<Command>>(),
            )
            .await
        {
            Ok(command_result) => Ok(info!(
                "Successfully registered {} global commands!",
                command_result.model().await?.len()
            )),
            Err(why) => Err(why.into()),
        }
    }
}

impl<D: LuroDatabaseDriver> Framework<D> {
    pub async fn builder(
        tracing_subscriber: Handle<LevelFilter, Registry>,
        config: Arc<Configuration<D>>
    ) -> anyhow::Result<(Arc<Framework<D>>, Vec<Shard>)> {
        let (database, current_user_id) = initialise_database(config.clone()).await?;
        let shards = stream::create_recommended(&config.twilight_client, config.shard_config.clone(), |_, c| c.build())
            .await?
            .collect::<Vec<_>>();

        #[cfg(feature = "lavalink")]
        let lavalink = {
            let socket = <std::net::SocketAddr as std::str::FromStr>::from_str(&config.lavalink_host)?;
            let lavalink = twilight_lavalink::Lavalink::new(current_user_id, shards.len().try_into()?);
            lavalink.add(socket, &config.lavalink_auth).await?;
            lavalink
        };

        #[cfg(feature = "http-client-hyper")]
        let http_client = hyper::Client::new();

        let framework = Self {
            #[cfg(feature = "cache-memory")]
            twilight_cache: config.cache.clone(),
            #[cfg(feature = "http-client-hyper")]
            http_client,
            database,
            #[cfg(feature = "lavalink")]
            lavalink,
            tracing_subscriber,
            twilight_client: config.twilight_client.clone(),
            hyper_client: hyper::Client::new()
        };

        Ok((framework.into(), shards))
    }

    pub async fn update_user<'a>(&'a self, user: &'a mut LuroUser) -> anyhow::Result<&'a mut LuroUser> {
        user.update_user(&self.twilight_client.user(user.id).await?.model().await?);
        Ok(user)
    }
}

async fn initialise_database<D: LuroDatabaseDriver>(
    config: Arc<Configuration<D>>
) -> anyhow::Result<(LuroDatabase<D>, Id<UserMarker>)> {
    let application = config.twilight_client.current_user_application().await?.model().await?;
    let current_user = config.twilight_client.current_user().await?.model().await?;
    let current_user_id = current_user.id;
    Ok((
        LuroDatabase::build(application, current_user, config),
        current_user_id,
    ))
}
