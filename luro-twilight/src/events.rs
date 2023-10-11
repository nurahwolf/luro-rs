use luro_framework::Context;
use tracing::error;
use twilight_gateway::Event;

mod interaction_create;
mod message_create;
mod message_delete;
mod message_delete_bulk;
mod message_update;
mod ready;
mod role;
mod user_update;
mod guild_update;

pub async fn event_handler(ctx: Context) -> anyhow::Result<()> {
    let callback = match ctx.event.clone() {
        Event::Ready(event) => ready::ready_listener(ctx, event).await,
        Event::RoleCreate(event) => role::role_create_listener(ctx, event).await,
        Event::RoleDelete(event) => role::role_delete_listener(ctx, event).await,
        Event::RoleUpdate(event) => role::role_update_listener(ctx, event).await,
        Event::UserUpdate(event) => user_update::user_update_listener(ctx, event).await,
        Event::InteractionCreate(event) => interaction_create::interaction_create_listener(ctx, event).await,
        Event::MessageCreate(event) => message_create::message_create_listener(ctx, event).await,
        Event::MessageDelete(event) => message_delete::message_delete_listener(ctx, event).await,
        Event::MessageDeleteBulk(event) => message_delete_bulk::message_delete_bulk_listener(ctx, event).await,
        Event::MessageUpdate(event) => message_update::message_update_listener(ctx, event).await,
        Event::GuildUpdate(event) => guild_update::guild_update_listener(ctx, event).await,
        _ => Ok(()),
    };

    if let Err(why) = callback {
        error!(why = ?why, "Unhandled error");
    }

    Ok(())
}
