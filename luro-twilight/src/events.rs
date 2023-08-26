use luro_framework::{Context, Framework, InteractionContext};
use luro_model::database::drivers::LuroDatabaseDriver;
use twilight_gateway::Event;

use self::interaction_create::interaction_create;
use self::ready::ready_listener;

mod interaction_create;
mod ready;

pub async fn event_handler<D: LuroDatabaseDriver>(framework: Framework<D>, ctx: Context, event: Event) -> anyhow::Result<()> {
    match event {
        Event::Ready(event) => ready_listener(framework, ctx, event).await,
        Event::InteractionCreate(event) => interaction_create(framework, InteractionContext::new(event.0, ctx)).await,
        _ => Ok(())
    }
}
