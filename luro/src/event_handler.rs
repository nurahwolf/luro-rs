use std::sync::Arc;

use luro_model::database::drivers::LuroDatabaseDriver;
use tracing::error;
use twilight_gateway::{Event, Latency, MessageSender};

mod audit_log_handler;
mod ban_add;
mod bulk_message_delete;
mod member_add;
mod member_update;
mod message_create;
mod message_delete;
mod message_update;
mod ready;
mod role_update;
mod thread_create;
mod thread_delete;
mod thread_list_sync;
mod thread_member_update;
mod thread_members_update;
mod thread_update;
mod user_update;
mod member_remove;

use crate::interaction::LuroSlash;

use super::Framework;

impl<D: LuroDatabaseDriver> Framework<D> {
    pub async fn event_handler(self: Arc<Self>, event: Event, shard: MessageSender, latency: Latency) -> anyhow::Result<()> {
        // events we want an IMMEDIATE resposne to, such as if we don't want the cache to be updated yet.
        let callback = match event.clone() {
            Event::MessageUpdate(message) => self.message_update_handler(*message).await,
            Event::MessageDelete(message) => self.message_delete_listener(message).await,
            Event::MessageDeleteBulk(event) => self.listener_bulk_message_delete(event).await,
            _ => Ok(())
        };

        // TODO: Really shitty event handler, please change this
        if let Err(why) = callback {
            error!("error while handling event - {:#?}", why);
        }

        self.lavalink.process(&event).await?;
        self.twilight_cache.update(&event);

        let callback = match event.clone() {
            Event::BanAdd(ban) => self.ban_add_listener(ban).await,
            Event::GuildAuditLogEntryCreate(entry) => self.clone().audit_log_handler(entry).await,
            Event::InteractionCreate(interaction) => LuroSlash::new(self.clone(), interaction.0, shard, latency).handle().await,
            Event::MemberAdd(event) => self.member_add_listener(event).await,
            Event::MemberUpdate(event) => self.member_update_listener(event).await,
            Event::MessageCreate(message) => self.message_create_listener(*message).await,
            Event::Ready(ready) => self.ready_listener(ready, shard).await,
            Event::RoleUpdate(event) => self.role_update_listener(event).await,
            Event::ThreadCreate(event) => self.listener_thread_create(event).await,
            Event::ThreadDelete(event) => self.listener_thread_delete(event).await,
            Event::ThreadListSync(event) => self.listener_thread_list_sync(event).await,
            Event::ThreadMembersUpdate(event) => self.listener_thread_members_update(event).await,
            Event::ThreadMemberUpdate(event) => self.listener_thread_member_update(event).await,
            Event::ThreadUpdate(event) => self.listener_thread_update(event).await,
            Event::UserUpdate(event) => self.user_update_listener(event).await,
            Event::MemberRemove(event) => self.member_remove_listener(event).await,            

            _ => Ok(())
        };

        // TODO: Really shitty event handler, please change this
        if let Err(why) = callback {
            error!(why = ?why, "error while handling event");
        }

        Ok(())
    }
}
