use std::sync::Arc;

use tracing::error;
use twilight_gateway::{Event, MessageSender};

use crate::models::{LuroFramework, LuroSlash};

mod audit_log_handler;
mod ban_add;
mod message_create;
mod message_delete;
mod message_update;
mod ready;
mod thread_create;
mod thread_delete;
mod thread_list_sync;
mod thread_member_update;
mod thread_members_update;
mod thread_update;

impl LuroFramework {
    pub async fn handle_event(self: Arc<Self>, event: Event, shard: MessageSender) -> anyhow::Result<()> {
        self.lavalink.process(&event).await?;
        self.twilight_cache.update(&event);

        let callback = match event {
            Event::Ready(ready) => self.ready_listener(ready, shard).await,
            Event::InteractionCreate(interaction) => LuroSlash::new(self, interaction.0, shard).handle().await,
            Event::MessageCreate(message) => self.message_create_listener(message).await,
            Event::MessageDelete(message) => self.message_delete_listener(message).await,
            Event::MessageUpdate(message) => self.message_update_handler(message).await,
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
