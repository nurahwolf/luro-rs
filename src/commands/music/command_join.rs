use std::time::Duration;

use songbird::Event;

use crate::{commands::music::struct_music::ChannelDurationNotifier, Context, Error};
/// Join a VC channel.
#[poise::command(slash_command, prefix_command, guild_only, ephemeral, category = "Music")]
pub async fn join(ctx: Context<'_>, #[description = "Announce how long the bot has been in the voice channel"] announce: Option<bool>) -> Result<(), Error> {
    if let Some(guild) = ctx.guild() {
        let guild_id = guild.id;
        let voice_channel = guild.voice_states.get(&ctx.author().id);

        match voice_channel {
            Some(channel) => {
                let (handle_lock, success) = ctx.data().songbird.join(guild_id, channel.channel_id.unwrap()).await;

                if let Ok(_channel) = success {
                    ctx.say(&format!("Joined {}", channel.channel_id.unwrap())).await?;

                    let chan_id = ctx.channel_id();
                    let mut handle = handle_lock.lock().await;

                    if announce.is_some() {
                        let send_http = ctx.serenity_context().http.clone();

                        handle.add_global_event(
                            Event::Periodic(Duration::from_secs(60), None),
                            ChannelDurationNotifier {
                                chan_id,
                                count: Default::default(),
                                http: send_http
                            }
                        );
                    }
                } else {
                    ctx.say("Error joining the channel").await?;
                }
            }
            None => {
                ctx.say("Not in a voice channel").await?;

                return Ok(());
            }
        };
    } else {
        ctx.say("You need to be in a guild for me to play music!").await?;
    }

    Ok(())
}
