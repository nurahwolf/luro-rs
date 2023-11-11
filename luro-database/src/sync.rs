use twilight_gateway::Event;

mod role;
mod channel;
mod presence;
mod member;
mod guild;
mod ready;

impl crate::Database {
    /// Syncronise data from the passed event into the database context. This automatically updates the cache, and database if the data is new.
    /// 
    /// This task should be spawned in the event loop of your bot.
    pub async fn sync_gateway(&self, event: &twilight_gateway::Event) -> anyhow::Result<()> {
        #[cfg(feature = "database-cache-twilight")]
        self.cache.update(event);

        match event {
            Event::ChannelCreate(event) => channel::create(self, event).await,
            Event::ChannelDelete(event) => channel::delete(self, event).await,
            Event::ChannelPinsUpdate(event) => channel::pins_update(self, event).await,
            Event::ChannelUpdate(event) => channel::update(self, event).await,
            Event::RoleCreate(event) => role::role_create_listener(self, event).await,
            Event::RoleDelete(event) => role::role_delete_listener(self, event).await,
            Event::RoleUpdate(event) => role::role_update_listener(self, event).await,
            Event::PresenceUpdate(event) => presence::update(self, event).await,
            Event::MemberAdd(event) => member::add(self, event).await,
            Event::MemberChunk(event) => member::chunk(self, event).await,
            Event::MemberRemove(event) => member::delete(self, event).await,
            Event::MemberUpdate(event) => member::update(self, event).await,
            Event::GuildUpdate(event) => guild::update(self, event).await,
            Event::Ready(event) => ready::ready(self, event).await,
            _ => Ok(())
        }
    }
}