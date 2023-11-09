use luro_framework::LuroContext;
use tracing::error;
use twilight_gateway::Event;

mod guild_update;
mod interaction_create;
mod member;
mod message_create;
mod message_delete;
mod message_delete_bulk;
mod message_update;
mod ready;
mod user_update;

pub async fn event_handler(ctx: LuroContext) -> anyhow::Result<()> {
    if let Err(why) = ctx.database.sync_gateway(&ctx.event).await {
        tracing::warn!(why = ?why, "Failed to sync event to the database")
    }

    let callback = match ctx.event.clone() {
        Event::GuildUpdate(event) => guild_update::guild_update_listener(ctx, event).await,
        Event::InteractionCreate(event) => interaction_create::interaction_create_listener(ctx, event).await,
        Event::MemberAdd(event) => member::add(ctx, event).await,
        Event::MemberChunk(event) => member::chunk(ctx, event).await,
        Event::MemberRemove(event) => member::delete(ctx, event).await,
        Event::MemberUpdate(event) => member::update(ctx, event).await,
        Event::MessageCreate(event) => message_create::message_create_listener(ctx, event).await,
        Event::MessageDelete(event) => message_delete::message_delete_listener(ctx, event).await,
        Event::MessageDeleteBulk(event) => message_delete_bulk::message_delete_bulk_listener(ctx, event).await,
        Event::MessageUpdate(event) => message_update::message_update_listener(ctx, event).await,
        Event::Ready(event) => ready::ready_listener(ctx, event).await,
        Event::UserUpdate(event) => user_update::user_update_listener(ctx, event).await,

        _ => Ok(()),
    };

    if let Err(why) = callback {
        error!(why = ?why, "Unhandled error");
    }

    Ok(())
}
