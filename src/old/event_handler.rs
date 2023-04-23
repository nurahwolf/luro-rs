use std::sync::Arc;

use tracing::{info, warn};
use twilight_gateway::{Event, MessageSender};

use crate::event_handler::join::join;
use crate::event_handler::leave::leave;
use crate::event_handler::pause::pause;
use crate::event_handler::play::play;
use crate::event_handler::seek::seek;
use crate::event_handler::stop::stop;
use crate::event_handler::volume::volume;
use crate::{
    commands::{join, leave, pause, play, seek, stop, volume},
    LuroContext,
};

use self::ready::ready_handler;

pub mod ready;

impl LuroContext {
    pub async fn event_handler(
        luro: Arc<LuroContext>,
        sender: Arc<MessageSender>,
        event: Event,
    ) -> anyhow::Result<()> {
        luro.standby.process(&event);
        luro.lavalink.process(&event).await?;

        let event_callback = match event {
            Event::Ready(ready) => ready_handler(luro, ready).await,
            Event::InteractionCreate(interaction) => {
                let interaction_id = interaction.id;
                info!("Handling interaction - {interaction_id}");

                {
                    let mut c = luro.interaction_count.write().await;
                    *c += 1;
                }
                

                if let Err(why) = luro.handle_interaction(interaction.0).await {
                    warn!("Failed to handle interaction - {why}")
                };

                Ok(())
            }
            Event::MessageCreate(msg) => {
                if msg.guild_id.is_none() || !msg.content.starts_with('!') {
                    return Ok(());
                }

                match msg.content.split_whitespace().next() {
                    Some("!join") => LuroContext::spawn(join(msg.0, luro, sender)),
                    Some("!leave") => LuroContext::spawn(leave(msg.0, luro, sender)),
                    Some("!play") => LuroContext::spawn(play(msg.0, luro)),
                    Some("!seek") => LuroContext::spawn(seek(msg.0, luro)),
                    Some("!stop") => LuroContext::spawn(stop(msg.0, luro)),
                    Some("!volume") => LuroContext::spawn(volume(msg.0, luro)),
                    Some("!pause") => LuroContext::spawn(pause(msg.0, luro)),
                    _ => return Ok(()),
                }

                Ok(())
            }
            _ => return Ok(()),
        };

        match event_callback {
            Ok(ok) => Ok(ok),
            Err(why) => {
                warn!("Failed to handle event: {why}");
                Ok(())
            }
        }
    }
}
