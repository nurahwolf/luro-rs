use tracing::error;
use twilight_gateway::{Event, MessageSender};

use crate::responses::LuroSlash;
use crate::{models::LuroFramework, LuroContext};

mod ban_add;
mod message_create;
mod message_delete;
mod message_update;
mod ready;

impl LuroFramework {
    pub async fn handle_event(ctx: LuroContext, event: Event, shard: MessageSender) -> anyhow::Result<()> {
        ctx.lavalink.process(&event).await?;
        ctx.twilight_cache.update(&event);

        let callback = match event {
            Event::Ready(ready) => ctx.ready_listener(ready, shard).await,
            Event::InteractionCreate(interaction) => LuroSlash::new(ctx, interaction.0, shard).handle().await,
            Event::MessageCreate(message) => ctx.message_create_listener(message).await,
            Event::MessageDelete(message) => ctx.message_delete_listener(message).await,
            Event::MessageUpdate(message) => LuroFramework::message_update_handler(message).await,
            Event::BanAdd(ban) => ctx.ban_add_listener(ban).await,
            _ => Ok(())
        };

        // TODO: Really shitty event handler, please change this
        if let Err(why) = callback {
            error!(why = ?why, "error while handling event");
        }

        Ok(())
    }
}
