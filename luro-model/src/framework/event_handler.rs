use std::sync::Arc;

use tracing::error;
use twilight_gateway::{Event, MessageSender};

mod audit_log_handler;
mod ban_add;
mod thread_create;
mod thread_delete;
mod thread_list_sync;
mod thread_member_update;
mod thread_members_update;
mod ready;
mod thread_update;
mod message_create;
mod message_delete;
mod message_update;

use super::Framework;

impl<D: LuroDatabaseDriver> Framework<D> {
    pub async fn event_handler(self: Arc<Self>, event: Event, shard: MessageSender) -> anyhow::Result<()> {
        // events we want an IMMEDIATE resposne to, such as if we don't want the cache to be updated yet.
        let callback = match event.clone() {
            Event::MessageUpdate(message) => self.message_update_handler(*message).await,
            Event::MessageDelete(message) => self.message_delete_listener(message).await,
            _ => Ok(())
        };

        // TODO: Really shitty event handler, please change this
        if let Err(why) = callback {
            error!(why = ?why, "error while handling event");
        }

        self.lavalink.process(&event).await?;
        self.twilight_cache.update(&event);

        let callback = match event {
            Event::Ready(ready) => self.ready_listener(ready, shard).await,
            Event::MessageCreate(message) => self.message_create_listener(*message).await,
            Event::InteractionCreate(interaction) => LuroSlash::new(self, interaction.0, shard).handle().await,
            Event::GuildAuditLogEntryCreate(entry) => self.audit_log_handler(entry).await,
            Event::BanAdd(ban) => self.ban_add_listener(ban).await,
            Event::ThreadCreate(event) => self.listener_thread_create(event).await,
            Event::ThreadDelete(event) => self.listener_thread_delete(event).await,
            Event::ThreadListSync(event) => self.listener_thread_list_sync(event).await,
            Event::ThreadMemberUpdate(event) => self.listener_thread_member_update(event).await,
            Event::ThreadMembersUpdate(event) => self.listener_thread_members_update(event).await,
            Event::ThreadUpdate(event) => self.listener_thread_update(event).await,
            _ => Ok(())
        };

        // TODO: Really shitty event handler, please change this
        if let Err(why) = callback {
            error!(why = ?why, "error while handling event");
        }

        Ok(())
    }
}