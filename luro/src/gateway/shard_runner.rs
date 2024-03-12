use std::sync::atomic::Ordering;

use futures_util::Future;
use twilight_gateway::{
    error::{ReceiveMessageError, ReceiveMessageErrorType},
    CloseFrame, Event, EventTypeFlags, Shard, StreamExt,
};

use crate::SHUTDOWN;

use self::{
    guild_create::guild_create_handler, guild_delete::guild_delete_handler,
    interaction_create::interaction_create, message_create::message_create_handler,
    no_handler::no_handler, ready::ready_listener,
};

use super::{GatewayArc, GatewayResult};

pub async fn shard_runner(gateway: GatewayArc, mut shard: Shard) {
    while let Some(event) = shard.next_event(EventTypeFlags::all()).await {
        let event = match event {
            Ok(Event::GatewayClose(event)) if SHUTDOWN.load(Ordering::Relaxed) => {
                handle_close(event, shard);
                break;
            }
            Err(error) => match handle_error(error, &shard) {
                true => break,
                false => continue,
            },
            Ok(event) => event,
        };

        let shrd_sndr = shard.sender();
        match event {
            Event::Ready(event) => spawn(ready_listener(gateway.clone(), shrd_sndr, event)),
            Event::GuildCreate(event) => guild_create_handler(gateway.clone(), shrd_sndr, event),
            Event::GuildDelete(event) => guild_delete_handler(gateway.clone(), shrd_sndr, event),
            Event::MessageCreate(event) => {
                spawn(message_create_handler(gateway.clone(), shrd_sndr, event))
            }
            Event::InteractionCreate(event) => {
                spawn(interaction_create(gateway.clone(), shrd_sndr, event))
            }
            event => no_handler(event),
        };
    }
}

/// Handles an event in a new Tokio task, to avoid blocking the shard. Also contains a error handler
fn spawn(fut: impl Future<Output = GatewayResult> + Send + 'static) {
    tokio::spawn(async move {
        if let Err(why) = fut.await {
            tracing::error!("GATEWAY: handler error: {why:#?}");
        }
    });
}

/// Handle when the gateway is closed
fn handle_close(event: Option<CloseFrame<'_>>, shard: Shard) {
    tracing::info!("GATEWAY: Gateway closed for shard `{}`.", shard.id());
    if let Some(event) = event {
        tracing::info!(
            "GATEWAY: CODE `{}` - Shutdown reason: {}",
            event.code,
            event.reason
        );
    }
}

/// Handles errors raised in the shard runner. Returns true if the user explicitly requested a shutdown.
fn handle_error(error: ReceiveMessageError, shard: &Shard) -> bool {
    tracing::error!(
        "GATEWAY: Shard `{}` raised the following error:",
        shard.id()
    );

    let requested_shutdown = SHUTDOWN.load(Ordering::Relaxed)
        && matches!(error.kind(), ReceiveMessageErrorType::WebSocket);

    match requested_shutdown {
        true => tracing::error!("GATEWAY: Explicit shutdown requested, raised error: {error:#?}"),
        false => tracing::error!("{error:#?}"),
    }

    requested_shutdown
}

mod guild_create;
mod guild_delete;
mod interaction_create;
mod message_create;
mod no_handler;
mod ready;
