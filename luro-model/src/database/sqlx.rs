use time::error::ComponentRange;
use twilight_model::util::{datetime::TimestampParseError, image_hash::ImageHashParseError};

use crate::config::Config;

mod count;
mod create;
mod delete;
mod fetch;
mod update;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("The SQLx driver MUST have a connection string passed, so that it knows what database to connect to.")]
    NoConnectionString,
    #[error("The SQLx driver itself had an error")]
    SqlxError(#[from] ::sqlx::Error),
    #[error("A image returned from the database failed to be converted to a Twilight type")]
    ImageHashParseError(#[from] ImageHashParseError),
    #[error("A timestamp returned from the database failed to be converted to a Twilight type")]
    TimestampParseError(#[from] TimestampParseError),
    #[error("Time range was outside of the allowed range")]
    TimeParseError(#[from] ComponentRange),
}

#[derive(Debug)]
pub struct Database {
    pub pool: ::sqlx::Pool<::sqlx::Postgres>,
}

impl Database {
    /// Create a new database instance
    pub async fn new(config: &Config) -> Result<Self, Error> {
        let Some(ref connection_string) = config.connection_string else {
            return Err(Error::NoConnectionString);
        };

        Ok(Self {
            pool: ::sqlx::postgres::PgPoolOptions::new()
                .max_connections(5)
                .connect(connection_string)
                .await?,
        })
    }
}

#[cfg(feature = "database-sync")]
mod sync {
    use twilight_gateway::Event;
    use twilight_model::gateway::payload::incoming::{PresenceUpdate, Ready};

    use super::{Database, Error};

    impl Database {
        /// Sync data from the gateway to the database driver.
        /// Useful for keeping things like roles, channels, and messages in sync.
        /// This task should be spawned in the event loop of your bot.
        pub async fn sync_gateway(&self, event: &twilight_gateway::Event) {
            let callback = match event {
                Event::ChannelCreate(event) => self.update_channel(event.as_ref()).await,
                Event::ChannelDelete(event) => self.update_channel(event.as_ref()).await,
                Event::ChannelPinsUpdate(event) => self.update_channel(event).await,
                Event::ChannelUpdate(event) => self.update_channel(event.as_ref()).await,
                Event::GuildCreate(event) => self.update_guild(event.as_ref()).await,
                Event::GuildUpdate(event) => self.update_guild(event.as_ref()).await,
                Event::InteractionCreate(event) => self.update_interaction(&event.0).await,
                Event::MemberAdd(event) => self.update_user(event.as_ref()).await,
                Event::MemberChunk(event) => self.update_user(event).await,
                Event::MemberRemove(event) => self.update_user(event).await,
                Event::MemberUpdate(event) => self.update_user(event.as_ref()).await,
                // Event::MessageCreate(event) => message::create(self, event).await,
                // Event::MessageDelete(event) => message::delete(self, event).await,
                // Event::MessageDeleteBulk(event) => message::delete_bulk(self, event).await,
                // Event::MessageUpdate(event) => message::update(self, event).await,
                Event::PresenceUpdate(event) => presence(self, event).await,
                Event::Ready(event) => ready(self, event).await,
                Event::RoleCreate(event) => self.update_role(event).await,
                Event::RoleDelete(event) => self.update_role(event).await,
                Event::RoleUpdate(event) => self.update_role(event).await,
                Event::UserUpdate(event) => self.update_user(event).await,
                _ => Ok(0),
            };

            match callback {
                Ok(rows_updated) => tracing::debug!("DATABASE: Updated {rows_updated} rows of data"),
                Err(why) => tracing::warn!(why = ?why, "DATABASE: Failed to sync incoming data"),
            }
        }
    }

    pub async fn presence(db: &Database, event: &PresenceUpdate) -> Result<u64, Error> {
        if let twilight_model::gateway::presence::UserOrId::User(user) = &event.user {
            match db.update_user(user).await {
                Ok(rows_updated) => return Ok(rows_updated),
                Err(why) => tracing::warn!(why = ?why, "PRESENCE: Failed to sync user {}", user.id),
            }
        }

        Ok(0)
    }

    async fn ready(db: &Database, event: &Ready) -> Result<u64, Error> {
        let mut rows_updated = 0;

        match db.update_application(&event.application).await {
            Ok(rows) => rows_updated += rows,
            Err(why) => tracing::warn!(why = ?why, "READY: Failed to sync application data {:?}", event.application),
        }

        for guild in &event.guilds {
            match db.update_guild(guild).await {
                Ok(rows) => rows_updated += rows,
                Err(why) => tracing::warn!(why = ?why, "READY: Failed to sync guild {}", guild.id),
            }
        }

        match db.update_user(&event.user).await {
            Ok(rows) => rows_updated += rows,
            Err(why) => tracing::warn!(why = ?why, "READY: Failed to sync current user {}", event.user.name),
        }

        Ok(rows_updated)
    }
}
