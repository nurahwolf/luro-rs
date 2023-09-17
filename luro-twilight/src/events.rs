use luro_framework::Context;
use twilight_gateway::Event;

mod interaction_create;
mod ready;
mod role_update;
mod user_update;

pub async fn event_handler(ctx: Context) -> anyhow::Result<()> {
    match ctx.event.clone() {
        Event::Ready(event) => ready::ready_listener(ctx, event).await,
        Event::RoleUpdate(event) => role_update::role_update_listener(ctx, event).await,
        Event::UserUpdate(event) => user_update::user_update_listener(ctx, event).await,
        Event::InteractionCreate(event) => interaction_create::interaction_create_listener(ctx, event).await,
        _ => Ok(()),
    }
}
