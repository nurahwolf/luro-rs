use luro_database::{LuroChannel, LuroDatabase, LuroGuild, LuroUser};
use luro_model::{builders::EmbedBuilder, response::LuroResponse, ACCENT_COLOUR};
use std::{future::Future, sync::Arc};
use tracing::{error, info, warn};
use twilight_cache_inmemory::InMemoryCache;
use twilight_http::{client::InteractionClient, Client};

use twilight_model::{
    application::command::Command,
    guild::Role,
    id::{
        marker::{ChannelMarker, GuildMarker, UserMarker},
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
    fn respond<F>(&self, _: F) -> impl std::future::Future<Output = anyhow::Result<()>> + Send
    where
        F: FnOnce(&mut LuroResponse) -> &mut LuroResponse + Send,
    {
        async { Ok(()) }
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
    fn database(&self) -> Arc<LuroDatabase>;

    /// Returns the twilight_client used by this context
    fn twilight_client(&self) -> Arc<Client>;

    /// Returns the guild ID if present
    fn guild_id(&self) -> Option<Id<GuildMarker>>;

    /// Returns the cache used by this context
    fn cache(&self) -> Arc<InMemoryCache>;

    /// Fetch and return a [LuroGuild], updating the database if not present
    /// Set fresh to true in order to fetch fresh data using the API
    ///
    /// Luro Database -> Twilight Guild
    fn get_guild(&self, guild_id: Id<GuildMarker>, fresh: bool) -> impl Future<Output = anyhow::Result<LuroGuild>> + Send
    where
        Self: Sync,
    {
        async move {
            if fresh {
                let twilight_guild = self.twilight_client().guild(guild_id).await?.model().await?;
                if let Err(why) = self.database().update_guild(twilight_guild).await {
                    error!(why = ?why, "failed to sync guild `{guild_id}` to the database");
                }
            }

            self.database().get_guild(guild_id).await
        }
    }

    fn get_guilds(&self) -> impl Future<Output = anyhow::Result<Vec<LuroGuild>>> + Send
    where
        Self: Sync,
    {
        async { self.database().get_all_guilds().await }
    }

    /// Fetch and return a [LuroUser], updating the database if not present. This version ensures a [Member] context is applied.
    /// Luro Database -> Twilight Client
    fn fetch_member_only(
        &self,
        user_id: Id<UserMarker>,
        guild_id: Id<GuildMarker>,
    ) -> impl std::future::Future<Output = anyhow::Result<LuroUser>> + Send
    where
        Self: Sync,
    {
        async move {
            match self.database().get_member(user_id, guild_id).await {
                Ok(member) => Ok(member),
                Err(why) => {
                    warn!(
                        why = ?why,
                        "fetch_member - Failed to fetch member `{user_id}` of guild `{guild_id}`, are they not a member of that guild?"
                    );
                    self.database().get_user(user_id).await
                }
            }
        }
    }

    /// Fetch and return a [LuroUser], updating the database if not present. This version does not check if a guild is present.
    /// Luro Database -> Twilight Client
    fn fetch_user_only(&self, user_id: Id<UserMarker>) -> impl std::future::Future<Output = anyhow::Result<LuroUser>> + Send
    where
        Self: Sync,
    {
        async move { self.database().get_user(user_id).await }
    }

    /// Fetch and return a [LuroUser], updating the database if not present. This version gets a member if a guild is present.
    /// Luro Database -> Twilight Client
    fn fetch_user(&self, user_id: Id<UserMarker>) -> impl std::future::Future<Output = anyhow::Result<LuroUser>> + Send
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
    /// Set fresh to true in order to fetch fresh data using the API
    ///
    /// Luro Database -> Twilight Client
    ///
    /// TODO: Finish this implementation
    fn fetch_channel(&self, channel_id: Id<ChannelMarker>) -> impl std::future::Future<Output = anyhow::Result<LuroChannel>> + Send
    where
        Self: Sync,
    {
        async move {
            if let Ok(Some(channel)) = self.database().get_channel(channel_id).await {
                return Ok(channel);
            }

            warn!("Failed to find channel `{channel_id}` in the database, falling back to Twilight");
            let twilight_channel = self.twilight_client().channel(channel_id).await?.model().await?;

            if let Err(why) = self.database().update_channel(twilight_channel.clone()).await {
                error!(why = ?why, "failed to sync channel `{channel_id}` to the database");
            }

            if let Ok(Some(channel)) = self.database().get_channel(channel_id).await {
                return Ok(channel);
            }

            Ok(twilight_channel.into())
        }
    }

    /// Fetch all guild roles.
    /// Set bypass to true to force a flush of all roles, if you want to make sure we have the most up to date roles possible, such as for highly privileged commands.
    fn get_guild_roles(
        &self,
        guild_id: Id<GuildMarker>,
        bypass: bool,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<Role>>> + Send
    where
        Self: Sync,
    {
        async move {
            // Get fresh roles at user request
            if bypass {
                let roles = self.twilight_client().roles(guild_id).await?.model().await?;

                for role in &roles {
                    self.database().update_role((guild_id, role.clone())).await?;
                }

                return Ok(roles);
            }

            // Get from database
            if let Ok(roles) = self.database().get_guild_roles(guild_id).await {
                return Ok(roles.into_iter().map(|x| x.into()).collect::<Vec<_>>());
            }

            // Database failed, fetch from client.
            info!("Failed to find guild roles for guild {guild_id}, fetching using twilight_client");
            let roles = self.twilight_client().roles(guild_id).await?.model().await?;

            for role in &roles {
                self.database().update_role((guild_id, role.clone())).await?;
            }

            Ok(roles)
        }
    }

    // async fn get_guild_member_roles(&self, guild_id: &Id<GuildMarker>, user_id: &Id<UserMarker>, bypass: bool) -> anyhow::Result<Vec<Role>>
    // where
    //     Self: Sync,
    // {
    //     let guild_roles = self.get_guild_roles(guild_id, true).await?;

    // }
}
