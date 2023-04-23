use futures::Future;
use tracing::{info, warn};
use twilight_gateway::{stream::ShardRef, Event};

use crate::{Luro, State, commands::{join::join, leave::leave, play::play, seek::seek, stop::stop, volume::volume, pause::pause}};

use self::ready::ready_handler;

pub mod interaction_create;
pub mod message_create;
pub mod ready;

impl Luro {
    pub async fn event_handler(
        state: State,
        shard: ShardRef<'_>,
        event: Event,
    ) -> anyhow::Result<()> {
        state.twilight_standby.process(&event);
        state.lavalink.process(&event).await?;

        let event_callback = match event {
            Event::Ready(ready) => ready_handler(state, ready).await,
            Event::InteractionCreate(interaction) => {
                let interaction_id = interaction.id;
                info!("Handling interaction - {interaction_id}");

                {
                    match state.data.interaction_count.write() {
                        Ok(mut interaction_count) => {
                            *interaction_count += 1
                        },
                        Err(_) => todo!(),
                    };
                }
                
                if let Err(why) = Luro::handle_interaction(state, interaction).await {
                    warn!("Failed to handle interaction - {why}")
                };

                Ok(())
            }
            Event::MessageCreate(msg) => {
                if msg.guild_id.is_none() || !msg.content.starts_with('!') {
                    return Ok(());
                }

                let test = match msg.content.split_whitespace().next() {
                    Some("!join") => join(msg, state, shard).await,
                    Some("!leave") => leave(msg, state, shard).await,
                    Some("!play") => play(msg, state).await,
                    Some("!seek") => seek(msg, state).await,
                    Some("!stop") => stop(msg, state).await,
                    Some("!volume") => volume(msg, state).await,
                    Some("!pause") => pause(msg, state).await,
                    _ => return Ok(()),
                };

                match test {
                    Ok(_) => info!("Success!"),
                    Err(why) => warn!("Error! {why}"),
                };


                // match msg.content.split_whitespace().next() {
                //     Some("!join") => spawn(join(msg, state, shard)),
                //     Some("!leave") => spawn(leave(msg, state, shard)),
                //     Some("!play") => spawn(play(msg, state)),
                //     Some("!seek") => spawn(seek(msg, state)),
                //     Some("!stop") => spawn(stop(msg, state)),
                //     Some("!volume") => spawn(volume(msg, state)),
                //     Some("!pause") => spawn(pause(msg, state)),
                //     _ => return Ok(()),
                // };

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