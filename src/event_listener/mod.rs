/// **Luro's event listener**
///
/// This module calls Luro's custom [event_listener], which handles Discord's events.
use crate::{Data, Error};
use poise::serenity_prelude::Context;

mod interaction_create;
mod message;
mod ready;

/// **Luro's event listener**
///
/// This function is called every time Discord pushes an event, which is then matched and reacted to accordingly.
pub async fn event_listener(ctx: &Context, event: &poise::Event<'_>, framework: poise::FrameworkContext<'_, Data, Error>, user_data: &Data) -> Result<(), Error> {
    match event {
        poise::Event::Ready { data_about_bot } => ready::ready_listener(data_about_bot, ctx).await?,
        poise::Event::InteractionCreate { interaction } => interaction_create::interaction_create(interaction).await?,
        poise::Event::Message { new_message } => message::message(new_message, ctx, &framework, user_data).await?,
        poise::Event::PresenceUpdate { new_data: _ } => {}                    // Ignore this event
        poise::Event::TypingStart { event: _ } => {}                          // Ignore this event
        poise::Event::GuildMemberUpdate { old_if_available: _, new: _ } => {} // Ignore this event

        _ => {
            println!("Got an event in listener: {:?}", event.name());
        }
    }

    Ok(())
}
