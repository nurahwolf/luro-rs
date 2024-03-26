use time::error::ComponentRange;
use twilight_model::util::{datetime::TimestampParseError, image_hash::ImageHashParseError};

use crate::config::Config;

mod create;
mod delete;
mod fetch;
#[cfg(feature = "database-sync")]
mod sync;
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

    #[cfg(feature = "database-sync")]
    /// Sync data from the gateway to the database driver.
    /// Useful for keeping things like roles, channels, and messages in sync.
    /// This task should be spawned in the event loop of your bot.
    pub async fn sync_gateway(&self, event: &twilight_gateway::Event) {
        use sync::{channel, guild, interaction, member, presence, ready, user};
        use twilight_gateway::Event;

        let callback = match event {
            Event::ChannelCreate(event) => channel::create(self, event).await,
            Event::ChannelDelete(event) => channel::delete(self, event).await,
            Event::ChannelPinsUpdate(event) => channel::pins_update(self, event).await,
            Event::ChannelUpdate(event) => channel::update(self, event).await,
            Event::GuildCreate(event) => guild::create(self, event).await,
            Event::GuildUpdate(event) => guild::update(self, event).await,
            Event::InteractionCreate(event) => interaction::create(self, event).await,
            Event::MemberAdd(event) => member::add(self, event).await,
            Event::MemberChunk(event) => member::chunk(self, event).await,
            Event::MemberRemove(event) => member::delete(self, event).await,
            Event::MemberUpdate(event) => member::update(self, event).await,
            // Event::MessageCreate(event) => message::create(self, event).await,
            // Event::MessageDelete(event) => message::delete(self, event).await,
            // Event::MessageDeleteBulk(event) => message::delete_bulk(self, event).await,
            // Event::MessageUpdate(event) => message::update(self, event).await,
            Event::PresenceUpdate(event) => presence::update(self, event).await,
            Event::Ready(event) => ready::ready(self, event).await,
            Event::RoleCreate(event) => self.update_role(event).await.map(|_| ()),
            Event::RoleDelete(event) => self.update_role(event).await.map(|_| ()),
            Event::RoleUpdate(event) => self.update_role(event).await.map(|_| ()),
            Event::UserUpdate(event) => user::update(self, event).await,
            _ => Ok(()),
        };

        if let Err(why) = callback {
            tracing::warn!(why = ?why, "DATABASE: Failed to sync incoming data")
        }
    }
}
