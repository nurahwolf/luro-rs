use twilight_gateway::Event;

mod channel;
mod guild;
mod interaction;
mod member;
mod message;
mod presence;
mod ready;
mod role;
#[cfg(feature = "database-cache-twilight")]
mod twilight_cache;
mod user;

impl crate::Database {
    /// Syncronise data from the passed event into the database context. This automatically updates the cache, and database if the data is new.
    ///
    /// This task should be spawned in the event loop of your bot.
    pub async fn sync_gateway(&self, event: twilight_gateway::Event) {
        #[cfg(feature = "database-cache-twilight")]
        self.cache.update(&event);

        let callback = match event {
            Event::ChannelCreate(event) => channel::create(self, &event).await,
            Event::ChannelDelete(event) => channel::delete(self, &event).await,
            Event::ChannelPinsUpdate(event) => channel::pins_update(self, &event).await,
            Event::ChannelUpdate(event) => channel::update(self, &event).await,
            Event::GuildCreate(event) => guild::create(self, &event).await,
            Event::GuildUpdate(event) => guild::update(self, &event).await,
            Event::InteractionCreate(event) => interaction::create(self, &event).await,
            Event::MemberAdd(event) => member::add(self, &event).await,
            Event::MemberChunk(event) => member::chunk(self, &event).await,
            Event::MemberRemove(event) => member::delete(self, &event).await,
            Event::MemberUpdate(event) => member::update(self, &event).await,
            Event::MessageCreate(event) => message::create(self, &event).await,
            Event::MessageDelete(event) => message::delete(self, &event).await,
            Event::MessageDeleteBulk(event) => message::delete_bulk(self, &event).await,
            Event::MessageUpdate(event) => message::update(self, &event).await,
            Event::PresenceUpdate(event) => presence::update(self, &event).await,
            Event::Ready(event) => ready::ready(self, &event).await,
            Event::RoleCreate(event) => role::role_create_listener(self, &event).await,
            Event::RoleDelete(event) => role::role_delete_listener(self, &event).await,
            Event::RoleUpdate(event) => role::role_update_listener(self, &event).await,
            Event::UserUpdate(event) => user::update(self, &event).await,
            _ => Ok(()),
        };

        if let Err(why) = callback {
            tracing::warn!(why = ?why, "Failed to sync data to the database")
        }
    }
}
