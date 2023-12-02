use luro_database::Database;
use luro_model::{
    builders::EmbedBuilder,
    response::{safe_split, InteractionResponse},
    types::{Channel, CommandResponse, Guild, Role, User},
    ACCENT_COLOUR,
};
use std::{future::Future, sync::Arc};
use tracing::{info, warn};
use twilight_http::{client::InteractionClient, Client};

use twilight_model::{
    application::command::Command,
    channel::{message::MessageReference, Webhook},
    id::{
        marker::{ChannelMarker, GuildMarker, MessageMarker, UserMarker},
        Id,
    },
    oauth::Application,
};

/// A trait that enforces the things you can access in ANY context
pub trait Luro {
    fn accent_colour(&self) -> u32 {
        ACCENT_COLOUR
    }

    /// Create a response to an interaction.
    /// This automatically handles if the interaction had been deferred.
    fn respond<F>(&self, _: F) -> impl std::future::Future<Output = anyhow::Result<CommandResponse>> + Send
    where
        F: FnOnce(&mut InteractionResponse) -> &mut InteractionResponse + Send,
    {
        async { Ok(CommandResponse::default()) }
    }

    /// Create a default embed which has the guild's accent colour if available, otherwise falls back to Luro's accent colour
    fn default_embed(&self) -> impl std::future::Future<Output = EmbedBuilder> + Send
    where
        Self: Sync,
    {
        async {
            let mut embed = EmbedBuilder::default();
            embed.colour(self.accent_colour());
            embed
        }
    }

    /// Gets the [interaction client](InteractionClient) using this framework's
    /// [http client](Client) and [application id](ApplicationMarker)
    fn interaction_client(&self) -> impl Future<Output = anyhow::Result<InteractionClient>> + Send;

    fn application(&self) -> impl Future<Output = anyhow::Result<Application>> + Send
    where
        Self: Sync,
    {
        async { Ok(self.twilight_client().current_user_application().await?.model().await?) }
    }

    /// Register commands to the Discord API.
    fn register_commands(&self, commands: &[Command]) -> impl Future<Output = anyhow::Result<()>> + Send
    where
        Self: Sync,
    {
        async {
            let client = self.interaction_client().await?;

            match client.set_global_commands(commands).await {
                Ok(command_result) => Ok(info!(
                    "Successfully registered {} global commands!",
                    command_result.model().await?.len()
                )),
                Err(why) => Err(why.into()),
            }
        }
    }

    /// Returns the database used by this context
    fn database(&self) -> Arc<Database>;

    /// Returns the twilight_client used by this context
    fn twilight_client(&self) -> Arc<Client>;

    /// Returns the guild ID if present
    fn guild_id(&self) -> Option<Id<GuildMarker>>;

    /// Fetch and return a [LuroGuild], updating the database if not present
    /// Set fresh to true in order to fetch fresh data using the API
    ///
    /// Luro Database -> Twilight Guild
    fn get_guild(&self, guild_id: Id<GuildMarker>) -> impl Future<Output = anyhow::Result<Guild>> + Send
    where
        Self: Sync,
    {
        async move { self.database().guild_fetch(guild_id).await }
    }

    fn get_guilds(&self) -> impl Future<Output = anyhow::Result<Vec<Guild>>> + Send
    where
        Self: Sync,
    {
        async { self.database().guilds_fetch().await }
    }

    /// Fetch and return a [LuroUser], updating the database if not present. This version ensures a [Member] context is applied.
    /// Luro Database -> Twilight Client
    fn fetch_member_only(
        &self,
        user_id: Id<UserMarker>,
        guild_id: Id<GuildMarker>,
    ) -> impl std::future::Future<Output = anyhow::Result<User>> + Send
    where
        Self: Sync,
    {
        async move {
            match self.database().member_fetch(guild_id, user_id).await {
                Ok(member) => Ok(member),
                Err(why) => {
                    warn!(
                        why = ?why,
                        "fetch_member - Failed to fetch member `{user_id}` of guild `{guild_id}`, are they not a member of that guild?"
                    );
                    self.database().user_fetch(user_id).await
                }
            }
        }
    }

    /// Fetch and return a [LuroUser], updating the database if not present. This version does not check if a guild is present.
    /// Luro Database -> Twilight Client
    fn fetch_user_only(&self, user_id: Id<UserMarker>) -> impl std::future::Future<Output = anyhow::Result<User>> + Send
    where
        Self: Sync,
    {
        async move { self.database().user_fetch(user_id).await }
    }

    /// Fetch and return a [LuroUser], updating the database if not present. This version gets a member if a guild is present.
    /// Luro Database -> Twilight Client
    fn fetch_user(&self, user_id: Id<UserMarker>) -> impl std::future::Future<Output = anyhow::Result<User>> + Send
    where
        Self: Sync,
    {
        async move {
            match self.guild_id() {
                Some(guild_id) => self.fetch_member_only(user_id, guild_id).await,
                None => self.fetch_user_only(user_id).await,
            }
        }
    }

    /// Fetch and return a [LuroChannel], updating the database if not present.
    ///
    /// Luro Database -> Twilight Client
    ///
    /// TODO: Finish this implementation
    fn fetch_channel(&self, channel_id: Id<ChannelMarker>) -> impl std::future::Future<Output = anyhow::Result<Channel>> + Send
    where
        Self: Sync,
    {
        async move { self.database().channel_fetch(channel_id).await }
    }

    /// Fetch all guild roles.
    /// Set bypass to true to force a flush of all roles, if you want to make sure we have the most up to date roles possible, such as for highly privileged commands.
    fn get_guild_roles(&self, guild_id: Id<GuildMarker>) -> impl std::future::Future<Output = anyhow::Result<Vec<Role>>> + Send
    where
        Self: Sync,
    {
        async move { self.database().role_fetch_guild(guild_id).await }
    }

    // Get a webhook for a channel, or create it if it does not exist
    fn get_webhook(&self, channel_id: Id<ChannelMarker>) -> impl std::future::Future<Output = anyhow::Result<Webhook>> + Send
    where
        Self: Sync,
    {
        async move {
            let webhooks = self.twilight_client().channel_webhooks(channel_id).await?.model().await?;
            let mut webhook = None;

            for wh in webhooks {
                if let Some(ref webhook_name) = wh.name {
                    if webhook_name == &self.database().current_user.to_string() {
                        webhook = Some(wh);
                        break;
                    }
                }
            }

            match webhook {
                Some(webhook) => Ok(webhook),
                None => self.create_webhook(channel_id).await,
            }
        }
    }

    fn create_webhook(&self, channel_id: Id<ChannelMarker>) -> impl std::future::Future<Output = anyhow::Result<Webhook>> + Send
    where
        Self: Sync,
    {
        async move {
            Ok(self
                .twilight_client()
                .create_webhook(channel_id, &self.database().current_user.to_string())
                .await?
                .model()
                .await?)
        }
    }

    // async fn get_guild_member_roles(&self, guild_id: &Id<GuildMarker>, user_id: &Id<UserMarker>, bypass: bool) -> anyhow::Result<Vec<Role>>
    // where
    //     Self: Sync,
    // {
    //     let guild_roles = self.get_guild_roles(guild_id, true).await?;

    // }

    fn proxy_character(
        &self,
        author: &User,
        channel_id: Id<ChannelMarker>,
        character: luro_model::types::CharacterProfile,
        proxy_message: String,
        message_reference: Option<&MessageReference>,
        reply_id: Id<MessageMarker>,
    ) -> impl std::future::Future<Output = anyhow::Result<CommandResponse>> + Send
    where
        Self: Sync,
    {
        async move {
            let character_icon = match self.database().channel_fetch(channel_id).await {
                Ok(channel) => match channel.nsfw.unwrap_or_default() {
                    true => character.nsfw_icon.unwrap_or(character.sfw_icon),
                    false => character.sfw_icon,
                },
                Err(why) => {
                    tracing::warn!(why = ?why, "Failed to fetch channel");
                    character.sfw_icon
                }
            };

            let proxy_message_chunked = safe_split(&proxy_message, 2000);

            // Attempt to first send as a webhook
            if let Ok(webhook) = self.get_webhook(channel_id).await {
                if let Some(token) = webhook.token {
                    if let Some(data) = message_reference {
                        if let Some(message_id) = data.message_id {
                            if let Ok(mut message) = self.database().message_fetch(message_id, data.channel_id).await {
                                luro_model::response::safe_truncate(&mut message.content, 150);
                                let mut embed = self.default_embed().await;
                                let description = format!("[Replying to:]({}) {}", message.link(), message.content);

                                embed
                                    .author(|a| a.name(message.author.name()).icon_url(message.author.avatar_url()))
                                    .description(description);

                                let response = self
                                    .twilight_client()
                                    .execute_webhook(webhook.id, &token)
                                    .embeds(&vec![embed.into()])
                                    .username(&format!("{} [{}]", character.name, author.name()))
                                    .content(&proxy_message)
                                    .avatar_url(&character_icon)
                                    .await;

                                if response.is_ok() {
                                    return Ok(CommandResponse::default());
                                }
                            }
                        }
                    }

                    let response = self
                        .twilight_client()
                        .execute_webhook(webhook.id, &token)
                        .username(&format!("{} [{}]", character.name, author.name()))
                        .content(proxy_message_chunked.0)
                        .avatar_url(&character_icon)
                        .await;

                    if !proxy_message_chunked.1.is_empty() {
                        let response = self
                            .twilight_client()
                            .execute_webhook(webhook.id, &token)
                            .username(&format!("{} [{}]", character.name, author.name()))
                            .content(proxy_message_chunked.1)
                            .avatar_url(&character_icon)
                            .await;

                        if response.is_ok() {
                            return Ok(CommandResponse::default());
                        }
                    }

                    if response.is_ok() {
                        return Ok(CommandResponse::default());
                    }
                }
            }

            // Send as an embed
            let mut embed = EmbedBuilder::default();
            embed
                .author(|a| {
                    a.name(format!("{} - [{}]", character.nickname.unwrap_or(character.name), author.name()))
                        .icon_url(&character_icon)
                })
                .colour(character.colour.unwrap_or(self.accent_colour()))
                .description(proxy_message);

            self.twilight_client()
                .create_message(channel_id)
                .embeds(&vec![embed.into()])
                .reply(reply_id)
                .await?;

            Ok(CommandResponse::default())
        }
    }
}
