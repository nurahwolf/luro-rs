use std::sync::Arc;

use anyhow::Error;
use twilight_gateway::{Event, MessageSender};
use twilight_model::id::Id;
use twilight_util::builder::embed::EmbedBuilder;

use crate::models::luro::Luro;

use self::{
    interaction_create::interaction_create_listener, message_create::message_create_listener,
    message_delete::message_delete_listener, message_update::message_update_handler,
    ready::ready_listener,
};

mod interaction_create;
mod message_create;
mod message_delete;
mod message_update;
mod ready;

pub async fn handle_event(
    luro: Arc<Luro>,
    event: Event,
    shard: MessageSender,
) -> Result<(), Error> {
    luro.lavalink.process(&event).await?;
    luro.twilight_cache.update(&event);

    match event {
        Event::Ready(ready) => ready_listener(luro, ready).await,
        Event::InteractionCreate(interaction) => {
            interaction_create_listener(luro, interaction, shard).await
        }
        Event::MessageCreate(message) => message_create_listener(message).await,
        Event::MessageDelete(message) => message_delete_listener(luro, message).await,
        Event::MessageUpdate(message) => message_update_handler(message).await,
        _ => send_event_embed(
            luro,
            "Event without handler".to_string(),
            "I just got an event that I don't have a handler for!".to_string(),
        ).await,
    }?;

    Ok(())
}

pub async fn send_event_embed(
    luro: Arc<Luro>,
    title: String,
    description: String,
) -> Result<(), Error> {
    let embed = EmbedBuilder::default()
        .title(title)
        .description(description)
        .color(0xDABEEF)
        .build();
    let _message = luro
        .twilight_client
        .create_message(Id::new(1066690358588743760))
        .embeds(&[embed])?
        .await?;
    Ok(())
}
