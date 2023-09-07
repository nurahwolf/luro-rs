use luro_framework::{Context, Framework, InteractionContext};
use luro_model::database::drivers::LuroDatabaseDriver;
use twilight_gateway::Event;

mod interaction_create;
mod ready;
mod role_update;
mod user_update;

pub async fn event_handler<D: LuroDatabaseDriver>(framework: Framework<D>, ctx: Context, event: Event) -> anyhow::Result<()> {
    match event {
        Event::Ready(event) => ready::ready_listener(framework, ctx, event).await,
        Event::RoleUpdate(event) => role_update::role_update_listener(framework, ctx, event).await,
        Event::UserUpdate(event) => user_update::user_update_listener(framework, ctx, event).await,
        Event::InteractionCreate(event) => {
            interaction_create::interaction_create_listener(framework, InteractionContext::new(event.0, ctx)).await
        }
        _ => Ok(())
    }
}
