use anyhow::anyhow;
use luro_database::LuroDatabase;
use luro_model::{guild::LuroGuild, user::LuroUser};
use std::{future::Future, sync::Arc};
use tracing::{info, warn};
use twilight_cache_inmemory::InMemoryCache;
use twilight_http::{client::InteractionClient, Client};

use twilight_model::{
    application::command::Command,
    guild::Role,
    id::{
        marker::{GuildMarker, RoleMarker, UserMarker},
        Id,
    },
    oauth::Application,
};

/// A trait that enforces the things you can access in ANY context
pub trait Luro {
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

    /// Returns the cache used by this context
    fn cache(&self) -> Arc<InMemoryCache>;

    /// Fetch and return a [LuroGuild], updating the database if not present
    /// Luro Database -> Twilight Guild
    fn get_guild(&self, guild_id: &Id<GuildMarker>) -> impl Future<Output = anyhow::Result<LuroGuild>> + Send
    where
        Self: Sync,
    {
        async {
            Ok(match self.database().get_guild(guild_id.get() as i64).await? {
                Some(guild) => guild.into(),
                None => self
                    .database()
                    .update_guild(self.twilight_client().guild(*guild_id).await?.model().await?)
                    .await?
                    .into(),
            })
        }
    }

    fn get_guilds(&self) -> impl Future<Output = anyhow::Result<Vec<LuroGuild>>> + Send
    where
        Self: Sync,
    {
        async {
            Ok(self
                .database()
                .get_all_guilds()
                .await
                .map(|x| x.into_iter().map(|x| x.into()).collect())?)
        }
    }

    /// Fetch and return a [LuroGuild], updating the database if not present
    /// Luro Database -> Twilight Client
    fn get_user(&self, user_id: &Id<UserMarker>) -> impl Future<Output = anyhow::Result<LuroUser>> + Send
    where
        Self: Sync,
    {
        async {
            Ok(match self.database().get_user(user_id.get() as i64).await? {
                Some(user) => user.into(),
                None => {
                    let user = self.twilight_client().user(*user_id).await?.model().await?;
                    match self.database().update_user(user.clone()).await {
                        Ok(db_user) => match db_user {
                            Some(user) => user.into(),
                            None => {
                                warn!("User did not exist in the database, attempted to update the user but the database still did not return the user");
                                (&user).into()
                            }
                        },
                        Err(why) => {
                            warn!(why = ?why, "Failed to get user, falling back to twilight");
                            (&user).into()
                        }
                    }
                }
            })
        }
    }

    /// Fetch and return a [LuroGuild], updating the database if not present
    /// Luro Database -> Twilight Cache
    fn get_role(&self, role_id: Id<RoleMarker>) -> impl Future<Output = anyhow::Result<Role>> + Send
    where
        Self: Sync,
    {
        async move {
            if let Ok(Some(role)) = self.database().get_role(role_id.get() as i64).await {
                return Ok(role.into());
            }

            let cache = self.cache();
            let cached_role = match cache.role(role_id) {
                Some(role) => role,
                None => return Err(anyhow!("No role referance in cache or database: {role_id}")),
            };

            Ok(self
                .database()
                .update_role((cached_role.guild_id(), cached_role.resource().clone()))
                .await?
                .into())
        }
    }

    /// Fetch all guild roles. Set bypass to true to force a flush of all roles, if you want to make sure we have the most up to date roles possible, such as for highly privileged commands.
    fn get_guild_roles(
        &self,
        guild_id: &Id<GuildMarker>,
        bypass: bool,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<Role>>> + Send
    where
        Self: Sync,
    {
        async move {
            // Get fresh roles at user request
            if bypass {
                let roles = self.twilight_client().roles(*guild_id).await?.model().await?;

                for role in &roles {
                    self.database().update_role((*guild_id, role.clone())).await?;
                }

                return Ok(roles);
            }

            // Get from database
            if let Ok(roles) = self.database().get_guild_roles(guild_id.get() as i64).await {
                return Ok(roles.into_iter().map(|x| x.into()).collect::<Vec<Role>>());
            }

            // Database failed, fetch from client.
            info!("Failed to find guild roles for guild {guild_id}, fetching using twilight_client");
            let roles = self.twilight_client().roles(*guild_id).await?.model().await?;

            for role in &roles {
                self.database().update_role((*guild_id, role.clone())).await?;
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

    // async fn user_permission_calculator(&self, user_id: Id<UserMarker>, guild_id: Id<GuildMarker>) -> anyhow::Result<PermissionCalculator>
    // where
    //     Self: Sync,
    // {
    //     let roles = self.get_guild_roles(&guild_id, true).await?;
    //     let guild = self.get_guild(&guild_id).await?;
    //     let user = self.get_user(&user_id).await?;

    //     // Temp
    //     let everyone: LuroRole = self.get_role(guild_id.cast()).await?.into();
    //     Ok(PermissionCalculator::new(guild_id, user_id, everyone.permissions, &guild.user_role_permissions(&user)).owner_id(guild.owner_id))
    // }
}
