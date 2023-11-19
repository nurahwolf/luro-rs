use luro_framework::LuroContext;
use tracing::error;
use twilight_gateway::Event;

mod interaction_create;
mod message;
mod ready;

pub async fn event_handler(ctx: LuroContext) -> anyhow::Result<()> {
    if let Err(why) = ctx.database.sync_gateway(&ctx.event).await {
        tracing::warn!(why = ?why, "Failed to sync event to the database")
    }

    let callback = match ctx.event.clone() {
        Event::InteractionCreate(event) => interaction_create::interaction_create_listener(ctx, event).await,
        Event::Ready(event) => ready::ready_listener(ctx, event).await,
        Event::MessageCreate(event) => message::create(ctx, event).await,
        Event::GuildCreate(event) => {
            tracing::info!("guild_create - Joined guild {}", event.id);
            Ok(())
        }
        Event::GuildDelete(event) => {
            tracing::info!("guild_delete - Left guild {}", event.id);
            Ok(())
        }
        _ => Ok(()),
    };

    if let Err(why) = callback {
        error!(why = ?why, "Unhandled error");
    }

    Ok(())
}
