use std::sync::Arc;

use anyhow::Error;
use futures::Future;
use tracing::info;
use twilight_gateway::{stream::ShardRef, Event};
use twilight_model::application::interaction::InteractionData;

use crate::{commands, event_handler::ready::ready_listener, luro::Luro};
mod ready;

impl Luro {
    pub async fn handle_event(
        self: Arc<Self>,
        event: Event,
        shard: ShardRef<'_>,
    ) -> Result<(), Error> {
        self.twilight_cache.update(&event);
        self.twilight_standby.process(&event);
        self.lavalink.process(&event).await?;

        match event {
            Event::Ready(ready) => {
                info!("Handling Event");
                spawn(ready_listener(self, ready));
            }
            Event::InteractionCreate(interaction) => match &interaction.data {
                Some(InteractionData::ApplicationCommand(command)) => {
                    commands::handle_command(&self, &interaction, command, shard).await;
                }
                // Some(InteractionData::MessageComponent(component)) => {
                //     commands::handle_component(&self, &interaction, component).await;
                // }
                _ => {}
            },
            _ => {}
        }

        Ok(())
    }
}

fn spawn(future: impl Future<Output = anyhow::Result<()>> + Send + 'static) {
    tokio::spawn(async move {
        if let Err(why) = future.await {
            tracing::warn!("handler error: {why:?}");
        }
    });
}
