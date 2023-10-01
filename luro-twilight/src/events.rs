use luro_framework::Context;
use tracing::error;
use twilight_gateway::Event;

mod interaction_create;
mod ready;
mod role_update;
mod user_update;
mod message_create;
mod message_delete;
mod message_update;
mod message_delete_bulk;

pub async fn event_handler(ctx: Context) -> anyhow::Result<()> {
    let callback = match ctx.event.clone() {
        Event::Ready(event) => ready::ready_listener(ctx, event).await,
        Event::RoleUpdate(event) => role_update::role_update_listener(ctx, event).await,
        Event::UserUpdate(event) => user_update::user_update_listener(ctx, event).await,
        Event::InteractionCreate(event) => interaction_create::interaction_create_listener(ctx, event).await,
        Event::MessageCreate(event) => message_create::message_create_listener(ctx, event).await,
        Event::MessageDelete(event) => message_delete::message_delete_listener(ctx, event).await,
        Event::MessageDeleteBulk(event) => message_delete_bulk::message_delete_bulk_listener(ctx, event).await,
        Event::MessageUpdate(event) => message_update::message_update_listener(ctx, event).await,
        _ => Ok(()),
    };

    if let Err(why) = callback {
        error!(why = ?why, "Unhandled error");
    }

    Ok(())
}
